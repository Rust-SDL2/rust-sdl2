/*!
Event Handling
 */

use std::ffi::CStr;
use std::mem;
use libc::{c_int, uint32_t};
use num::FromPrimitive;
use std::ptr;
use std::borrow::ToOwned;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::collections::HashMap;
use std::sync::Mutex;
use std::mem::transmute;
use std::os::raw::c_void;

use controller;
use controller::{Axis, Button};
use joystick;
use joystick::HatState;
use keyboard;
use keyboard::Mod;
use keyboard::Keycode;
use mouse;
use mouse::{MouseButton, MouseState, MouseWheelDirection};
use keyboard::Scancode;
use get_error;

use sys;

struct CustomEventTypeMaps {
    sdl_id_to_type_id: HashMap<u32, ::std::any::TypeId>,
    type_id_to_sdl_id: HashMap<::std::any::TypeId, u32>
}

impl CustomEventTypeMaps {
    fn new() -> Self {
        CustomEventTypeMaps {
            sdl_id_to_type_id: HashMap::new(),
            type_id_to_sdl_id: HashMap::new()
        }
    }
}

lazy_static! {
    static ref CUSTOM_EVENT_TYPES : Mutex<CustomEventTypeMaps> = { Mutex::new(CustomEventTypeMaps::new()) };
}

impl ::EventSubsystem {
    /// Removes all events in the event queue that match the specified event type.
    pub fn flush_event(&self, event_type: EventType) {
        unsafe { sys::SDL_FlushEvent(event_type as uint32_t) };
    }

    /// Removes all events in the event queue that match the specified type range.
    pub fn flush_events(&self, min_type: u32, max_type: u32) {
        unsafe { sys::SDL_FlushEvents(min_type, max_type) };
    }

    /// Reads the events at the front of the event queue, until the maximum amount
    /// of events is read.
    ///
    /// The events will _not_ be removed from the queue.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::event::Event;
    ///
    /// let sdl_context = sdl2::init().unwrap();
    /// let event_subsystem = sdl_context.event().unwrap();
    ///
    /// // Read up to 1024 events
    /// let events: Vec<Event> = event_subsystem.peek_events(1024);
    ///
    /// // Print each one
    /// for event in events {
    ///     println!("{:?}", event);
    /// }
    /// ```
    pub fn peek_events<B>(&self, max_amount: u32) -> B
    where B: FromIterator<Event>
    {
        unsafe {
            let mut events = Vec::with_capacity(max_amount as usize);

            let result = {
                let events_ptr = events.as_mut_ptr();

                sys::SDL_PeepEvents(
                    events_ptr,
                    max_amount as c_int,
                    sys::SDL_eventaction::SDL_PEEKEVENT,
                    sys::SDL_EventType::SDL_FIRSTEVENT as u32,
                    sys::SDL_EventType::SDL_LASTEVENT as u32
                )
            };

            if result < 0 {
                // The only error possible is "Couldn't lock event queue"
                panic!(get_error());
            } else {
                events.set_len(result as usize);

                events.into_iter().map(|event_raw| {
                    Event::from_ll(event_raw)
                }).collect()
            }
        }
    }

    /// Pushes an event to the event queue.
    pub fn push_event(&self, event: Event) -> Result<(), String> {
        match event.to_ll() {
            Some(mut raw_event) => {
                let ok = unsafe { sys::SDL_PushEvent(&mut raw_event) == 1 };
                if ok { Ok(()) }
                else { Err(get_error()) }
            },
            None => {
                Err("Cannot push unsupported event type to the queue".to_owned())
            }
        }
    }


    /// Register a custom SDL event.
    ///
    /// When pushing a user event, you must make sure that the ``type_`` field is set to a
    /// registered SDL event number.
    ///
    /// The ``code``, ``data1``,  and ``data2`` fields can be used to store user defined data.
    ///
    /// See the [SDL documentation](https://wiki.libsdl.org/SDL_UserEvent) for more information.
    ///
    /// # Example
    /// ```
    /// let sdl = sdl2::init().unwrap();
    /// let ev = sdl.event().unwrap();
    ///
    /// let custom_event_type_id = unsafe { ev.register_event().unwrap() };
    /// let event = sdl2::event::Event::User {
    ///    timestamp: 0,
    ///    window_id: 0,
    ///    type_: custom_event_type_id,
    ///    code: 456,
    ///    data1: 0x1234 as *mut ::std::os::raw::c_void,
    ///    data2: 0x5678 as *mut ::std::os::raw::c_void,
    /// };
    ///
    /// ev.push_event(event);
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn register_event(&self) -> Result<u32, String> {
        Ok(*try!(self.register_events(1)).first().unwrap())
    }

    /// Registers custom SDL events.
    ///
    /// Returns an error, if no more user events can be created.
    pub unsafe fn register_events(&self, nr: u32) -> Result<Vec<u32>, String> {
        let result = sys::SDL_RegisterEvents(nr as ::libc::c_int);
        const ERR_NR:u32 = ::std::u32::MAX - 1;

        match result {
            ERR_NR => {
                Err("No more user events can be created; SDL_LASTEVENT reached"
                    .to_owned())
            },
            _ => {
                let event_ids = (result..(result+nr)).collect();
                Ok(event_ids)
            }
        }
    }

    /// Register a custom event
    ///
    /// It returns an error when the same type is registered twice.
    ///
    /// # Example
    /// See [push_custom_event](#method.push_custom_event)
    #[inline(always)]
    pub fn register_custom_event<T: ::std::any::Any>(&self)
            -> Result<(), String> {
        use ::std::any::TypeId;
        let event_id = *try!(unsafe { self.register_events(1) }).first().unwrap();
        let mut cet = CUSTOM_EVENT_TYPES.lock().unwrap();
        let type_id = TypeId::of::<Box<T>>();

        if cet.type_id_to_sdl_id.contains_key(&type_id) {
            return Err(
                "The same event type can not be registered twice!".to_owned()
            );
        }

        cet.sdl_id_to_type_id.insert(event_id, type_id);
        cet.type_id_to_sdl_id.insert(type_id, event_id);

        Ok(())
    }

    /// Push a custom event
    ///
    /// If the event type ``T`` was not registered using
    /// [register_custom_event](#method.register_custom_event),
    /// this method will panic.
    ///
    /// # Example: pushing and receiving a custom event
    /// ```
    /// struct SomeCustomEvent {
    ///     a: i32
    /// }
    ///
    /// let sdl = sdl2::init().unwrap();
    /// let ev = sdl.event().unwrap();
    /// let mut ep = sdl.event_pump().unwrap();
    ///
    /// ev.register_custom_event::<SomeCustomEvent>().unwrap();
    ///
    /// let event = SomeCustomEvent { a: 42 };
    ///
    /// ev.push_custom_event(event);
    ///
    /// let received = ep.poll_event().unwrap(); // or within a for event in ep.poll_iter()
    /// if received.is_user_event() {
    ///     let e2 = received.as_user_event_type::<SomeCustomEvent>().unwrap();
    ///     assert_eq!(e2.a, 42);
    /// }
    /// ```
    pub fn push_custom_event<T: ::std::any::Any>(&self, event:T)
            -> Result<(), String> {
        use ::std::any::TypeId;
        let cet = CUSTOM_EVENT_TYPES.lock().unwrap();
        let type_id = TypeId::of::<Box<T>>();

        let user_event_id = *match cet.type_id_to_sdl_id.get(&type_id) {
            Some(id) => id,
            None => {
                return Err(
                    "Type is not registered as a custom event type!".to_owned()
                );
            }
        };

        let event_box = Box::new(event);
        let event = Event::User {
           timestamp: 0,
           window_id: 0,
           type_: user_event_id,
           code: 0,
           data1: Box::into_raw(event_box) as *mut c_void,
           data2: ::std::ptr::null_mut()
        };

        try!(self.push_event(event));

        Ok(())
    }
}

/// Types of events that can be delivered.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u32)]
pub enum EventType {
    First = sys::SDL_EventType::SDL_FIRSTEVENT as u32,

    Quit = sys::SDL_EventType::SDL_QUIT as u32,
    AppTerminating = sys::SDL_EventType::SDL_APP_TERMINATING as u32,
    AppLowMemory = sys::SDL_EventType::SDL_APP_LOWMEMORY as u32,
    AppWillEnterBackground = sys::SDL_EventType::SDL_APP_WILLENTERBACKGROUND as u32,
    AppDidEnterBackground = sys::SDL_EventType::SDL_APP_DIDENTERBACKGROUND as u32,
    AppWillEnterForeground = sys::SDL_EventType::SDL_APP_WILLENTERFOREGROUND as u32,
    AppDidEnterForeground = sys::SDL_EventType::SDL_APP_DIDENTERFOREGROUND as u32,

    Window = sys::SDL_EventType::SDL_WINDOWEVENT as u32,
    // TODO: SysWM = sys::SDL_SYSWMEVENT as u32,

    KeyDown = sys::SDL_EventType::SDL_KEYDOWN as u32,
    KeyUp = sys::SDL_EventType::SDL_KEYUP as u32,
    TextEditing = sys::SDL_EventType::SDL_TEXTEDITING as u32,
    TextInput = sys::SDL_EventType::SDL_TEXTINPUT as u32,

    MouseMotion = sys::SDL_EventType::SDL_MOUSEMOTION as u32,
    MouseButtonDown = sys::SDL_EventType::SDL_MOUSEBUTTONDOWN as u32,
    MouseButtonUp = sys::SDL_EventType::SDL_MOUSEBUTTONUP as u32,
    MouseWheel = sys::SDL_EventType::SDL_MOUSEWHEEL as u32,

    JoyAxisMotion = sys::SDL_EventType::SDL_JOYAXISMOTION as u32,
    JoyBallMotion = sys::SDL_EventType::SDL_JOYBALLMOTION as u32,
    JoyHatMotion = sys::SDL_EventType::SDL_JOYHATMOTION as u32,
    JoyButtonDown = sys::SDL_EventType::SDL_JOYBUTTONDOWN as u32,
    JoyButtonUp = sys::SDL_EventType::SDL_JOYBUTTONUP as u32,
    JoyDeviceAdded = sys::SDL_EventType::SDL_JOYDEVICEADDED as u32,
    JoyDeviceRemoved = sys::SDL_EventType::SDL_JOYDEVICEREMOVED as u32,

    ControllerAxisMotion = sys::SDL_EventType::SDL_CONTROLLERAXISMOTION as u32,
    ControllerButtonDown = sys::SDL_EventType::SDL_CONTROLLERBUTTONDOWN as u32,
    ControllerButtonUp = sys::SDL_EventType::SDL_CONTROLLERBUTTONUP as u32,
    ControllerDeviceAdded = sys::SDL_EventType::SDL_CONTROLLERDEVICEADDED as u32,
    ControllerDeviceRemoved = sys::SDL_EventType::SDL_CONTROLLERDEVICEREMOVED as u32,
    ControllerDeviceRemapped = sys::SDL_EventType::SDL_CONTROLLERDEVICEREMAPPED as u32,

    FingerDown = sys::SDL_EventType::SDL_FINGERDOWN as u32,
    FingerUp = sys::SDL_EventType::SDL_FINGERUP as u32,
    FingerMotion = sys::SDL_EventType::SDL_FINGERMOTION as u32,
    DollarGesture = sys::SDL_EventType::SDL_DOLLARGESTURE as u32,
    DollarRecord = sys::SDL_EventType::SDL_DOLLARRECORD as u32,
    MultiGesture = sys::SDL_EventType::SDL_MULTIGESTURE as u32,

    ClipboardUpdate = sys::SDL_EventType::SDL_CLIPBOARDUPDATE as u32,
    DropFile = sys::SDL_EventType::SDL_DROPFILE as u32,

    User = sys::SDL_EventType::SDL_USEREVENT as u32,
    Last = sys::SDL_EventType::SDL_LASTEVENT as u32,
}

impl FromPrimitive for EventType {
    fn from_i64(n: i64) -> Option<EventType> {
        use self::EventType::*;
        use sys::SDL_EventType::*;
        let n = n as u32;

        Some( match unsafe { transmute(n) } {
            SDL_FIRSTEVENT => First,

            SDL_QUIT => Quit,
            SDL_APP_TERMINATING => AppTerminating,
            SDL_APP_LOWMEMORY => AppLowMemory,
            SDL_APP_WILLENTERBACKGROUND => AppWillEnterBackground,
            SDL_APP_DIDENTERBACKGROUND => AppDidEnterBackground,
            SDL_APP_WILLENTERFOREGROUND => AppWillEnterForeground,
            SDL_APP_DIDENTERFOREGROUND => AppDidEnterForeground,

            SDL_WINDOWEVENT => Window,

            SDL_KEYDOWN => KeyDown,
            SDL_KEYUP => KeyUp,
            SDL_TEXTEDITING => TextEditing,
            SDL_TEXTINPUT => TextInput,

            SDL_MOUSEMOTION => MouseMotion,
            SDL_MOUSEBUTTONDOWN => MouseButtonDown,
            SDL_MOUSEBUTTONUP => MouseButtonUp,
            SDL_MOUSEWHEEL => MouseWheel,

            SDL_JOYAXISMOTION => JoyAxisMotion,
            SDL_JOYBALLMOTION => JoyBallMotion,
            SDL_JOYHATMOTION => JoyHatMotion,
            SDL_JOYBUTTONDOWN => JoyButtonDown,
            SDL_JOYBUTTONUP => JoyButtonUp,
            SDL_JOYDEVICEADDED => JoyDeviceAdded,
            SDL_JOYDEVICEREMOVED => JoyDeviceRemoved,

            SDL_CONTROLLERAXISMOTION => ControllerAxisMotion,
            SDL_CONTROLLERBUTTONDOWN => ControllerButtonDown,
            SDL_CONTROLLERBUTTONUP => ControllerButtonUp,
            SDL_CONTROLLERDEVICEADDED => ControllerDeviceAdded,
            SDL_CONTROLLERDEVICEREMOVED => ControllerDeviceRemoved,
            SDL_CONTROLLERDEVICEREMAPPED => ControllerDeviceRemapped,

            SDL_FINGERDOWN => FingerDown,
            SDL_FINGERUP => FingerUp,
            SDL_FINGERMOTION => FingerMotion,
            SDL_DOLLARGESTURE => DollarGesture,
            SDL_DOLLARRECORD => DollarRecord,
            SDL_MULTIGESTURE => MultiGesture,

            SDL_CLIPBOARDUPDATE => ClipboardUpdate,
            SDL_DROPFILE => DropFile,

            SDL_USEREVENT => User,
            SDL_LASTEVENT => Last,

            _ => return None,
        })
    }

    fn from_u64(n: u64) -> Option<EventType> { FromPrimitive::from_i64(n as i64) }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
/// An enum of window events.
pub enum WindowEvent {
    None,
    Shown,
    Hidden,
    Exposed,
    Moved(i32,i32),
    Resized(i32,i32),
    SizeChanged(i32,i32),
    Minimized,
    Maximized,
    Restored,
    Enter,
    Leave,
    FocusGained,
    FocusLost,
    Close,
    TakeFocus,
    HitTest,
}

impl WindowEvent {
    #[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
    fn from_ll(id: u8, data1: i32, data2: i32) -> WindowEvent {
        match id {
            0  => WindowEvent::None,
            1  => WindowEvent::Shown,
            2  => WindowEvent::Hidden,
            3  => WindowEvent::Exposed,
            4  => WindowEvent::Moved(data1,data2),
            5  => WindowEvent::Resized(data1,data2),
            6  => WindowEvent::SizeChanged(data1,data2),
            7  => WindowEvent::Minimized,
            8  => WindowEvent::Maximized,
            9  => WindowEvent::Restored,
            10 => WindowEvent::Enter,
            11 => WindowEvent::Leave,
            12 => WindowEvent::FocusGained,
            13 => WindowEvent::FocusLost,
            14 => WindowEvent::Close,
            15 => WindowEvent::TakeFocus,
            16 => WindowEvent::HitTest,
            _  => WindowEvent::None
        }
    }

    fn to_ll(&self) -> (u8, i32, i32) {
        match *self {
            WindowEvent::None => (0, 0, 0),
            WindowEvent::Shown => (1, 0, 0),
            WindowEvent::Hidden => (2, 0, 0),
            WindowEvent::Exposed => (3, 0, 0),
            WindowEvent::Moved(d1,d2) => (4, d1, d2),
            WindowEvent::Resized(d1,d2) => (5, d1, d2),
            WindowEvent::SizeChanged(d1,d2) => (6, d1, d2),
            WindowEvent::Minimized => (7, 0, 0),
            WindowEvent::Maximized => (8, 0, 0),
            WindowEvent::Restored => (9, 0, 0),
            WindowEvent::Enter => (10, 0, 0),
            WindowEvent::Leave => (11, 0, 0),
            WindowEvent::FocusGained => (12, 0, 0),
            WindowEvent::FocusLost => (13, 0, 0),
            WindowEvent::Close => (14, 0, 0),
            WindowEvent::TakeFocus => (15, 0, 0),
            WindowEvent::HitTest => (16, 0, 0),
        }
    }

}

#[derive(Clone, PartialEq)]
/// Different event types.
pub enum Event {
    Quit { timestamp: u32 },
    AppTerminating { timestamp: u32 },
    AppLowMemory { timestamp: u32 },
    AppWillEnterBackground { timestamp: u32 },
    AppDidEnterBackground { timestamp: u32 },
    AppWillEnterForeground { timestamp: u32 },
    AppDidEnterForeground { timestamp: u32 },

    Window {
        timestamp: u32,
        window_id: u32,
        win_event: WindowEvent,
    },
    // TODO: SysWMEvent

    KeyDown {
        timestamp: u32,
        window_id: u32,
        keycode: Option<Keycode>,
        scancode: Option<Scancode>,
        keymod: Mod,
        repeat: bool
    },
    KeyUp {
        timestamp: u32,
        window_id: u32,
        keycode: Option<Keycode>,
        scancode: Option<Scancode>,
        keymod: Mod,
        repeat: bool
    },

    TextEditing {
        timestamp: u32,
        window_id: u32,
        text: String,
        start: i32,
        length: i32
    },

    TextInput {
        timestamp: u32,
        window_id: u32,
        text: String
    },

    MouseMotion {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mousestate: MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32
    },

    MouseButtonDown {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mouse_btn: MouseButton,
        clicks: u8,
        x: i32,
        y: i32
    },
    MouseButtonUp {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mouse_btn: MouseButton,
        clicks: u8,
        x: i32,
        y: i32
    },

    MouseWheel {
        timestamp: u32,
        window_id: u32,
        which: u32,
        x: i32,
        y: i32,
        direction: MouseWheelDirection,
    },

    JoyAxisMotion {
        timestamp: u32,
        /// The joystick's `id`
        which: i32,
        axis_idx: u8,
        value: i16
    },

    JoyBallMotion {
        timestamp: u32,
        /// The joystick's `id`
        which: i32,
        ball_idx: u8,
        xrel: i16,
        yrel: i16
    },

    JoyHatMotion {
        timestamp: u32,
        /// The joystick's `id`
        which: i32,
        hat_idx: u8,
        state: HatState
    },

    JoyButtonDown {
        timestamp: u32,
        /// The joystick's `id`
        which: i32,
        button_idx: u8
    },
    JoyButtonUp {
        timestamp: u32,
        /// The joystick's `id`
        which: i32,
        button_idx: u8
    },

    JoyDeviceAdded {
        timestamp: u32,
        /// The newly added joystick's `joystick_index`
        which: u32
    },
    JoyDeviceRemoved {
        timestamp: u32,
        /// The joystick's `id`
        which: i32
    },

    ControllerAxisMotion {
        timestamp: u32,
        /// The controller's joystick `id`
        which: i32,
        axis: Axis,
        value: i16
    },

    ControllerButtonDown {
        timestamp: u32,
        /// The controller's joystick `id`
        which: i32,
        button: Button
    },
    ControllerButtonUp {
        timestamp: u32,
        /// The controller's joystick `id`
        which: i32,
        button: Button
    },

    ControllerDeviceAdded {
        timestamp: u32,
        /// The newly added controller's `joystick_index`
        which: u32
    },
    ControllerDeviceRemoved {
        timestamp: u32,
        /// The controller's joystick `id`
        which: i32
    },
    ControllerDeviceRemapped {
        timestamp: u32,
        /// The controller's joystick `id`
        which: i32
    },

    FingerDown {
        timestamp: u32,
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32
    },
    FingerUp {
        timestamp: u32,
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32
    },
    FingerMotion {
        timestamp: u32,
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32
    },

    DollarGesture {
        timestamp: u32,
        touch_id: i64,
        gesture_id: i64,
        num_fingers: u32,
        error: f32,
        x: f32,
        y: f32
    },
    DollarRecord {
        timestamp: u32,
        touch_id: i64,
        gesture_id: i64,
        num_fingers: u32,
        error: f32,
        x: f32,
        y: f32
    },

    MultiGesture {
        timestamp: u32,
        touch_id: i64,
        d_theta: f32,
        d_dist: f32,
        x: f32,
        y: f32,
        num_fingers: u16
    },

    ClipboardUpdate {
        timestamp: u32
    },

    DropFile {
        timestamp: u32,
        filename: String
    },

    User {
        timestamp: u32,
        window_id: u32,
        type_: u32,
        code: i32,
        data1: *mut c_void,
        data2: *mut c_void
    },

    Unknown {
        timestamp: u32,
        type_: u32
    }
}

impl ::std::fmt::Debug for Event {
    fn fmt(&self, out: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        out.write_str(match *self {
            Event::Quit{..} => "Event::Quit",
            Event::AppTerminating{..} => "Event::AppTerminating",
            Event::AppLowMemory{..} => "Event::AppLowMemory",
            Event::AppWillEnterBackground{..} => "Event::AppWillEnterBackground",
            Event::AppDidEnterBackground{..} => "Event::AppDidEnterBackground",
            Event::AppWillEnterForeground{..} => "Event::AppWillEnterForeground",
            Event::AppDidEnterForeground{..} => "Event::AppDidEnterForeground",
            Event::Window{..} => "Event::Window",
            Event::KeyDown{..} => "Event::KeyDown",
            Event::KeyUp{..} => "Event::KeyUp",
            Event::TextEditing{..} => "Event::TextEditing",
            Event::TextInput{..} => "Event::TextInput",
            Event::MouseMotion{..} => "Event::MouseMotion",
            Event::MouseButtonDown{..} => "Event::MouseButtonDown",
            Event::MouseButtonUp{..} => "Event::MouseButtonUp",
            Event::MouseWheel{..} => "Event::MouseWheel",
            Event::JoyAxisMotion{..} => "Event::JoyAxisMotion",
            Event::JoyBallMotion{..} => "Event::JoyBallMotion",
            Event::JoyHatMotion{..} => "Event::JoyHatMotion",
            Event::JoyButtonDown{..} => "Event::JoyButtonDown",
            Event::JoyButtonUp{..} => "Event::JoyButtonUp",
            Event::JoyDeviceAdded{..} => "Event::JoyDeviceAdded",
            Event::JoyDeviceRemoved{..} => "Event::JoyDeviceRemoved",
            Event::ControllerAxisMotion{..} => "Event::ControllerAxisMotion",
            Event::ControllerButtonDown{..} => "Event::ControllerButtonDown",
            Event::ControllerButtonUp{..} => "Event::ControllerButtonUp",
            Event::ControllerDeviceAdded{..} => "Event::ControllerDeviceAdded",
            Event::ControllerDeviceRemoved{..} => "Event::ControllerDeviceRemoved",
            Event::ControllerDeviceRemapped{..} => "Event::ControllerDeviceRemapped",
            Event::FingerDown{..} => "Event::FingerDown",
            Event::FingerUp{..} => "Event::FingerUp",
            Event::FingerMotion{..} => "Event::FingerMotion",
            Event::DollarGesture{..} => "Event::DollarGesture",
            Event::DollarRecord{..} => "Event::DollarRecord",
            Event::MultiGesture{..} => "Event::MultiGesture",
            Event::ClipboardUpdate{..} => "Event::ClipboardUpdate",
            Event::DropFile{..} => "Event::DropFile",
            Event::User{..} => "Event::User",
            Event::Unknown{..} => "Event::Unknown",
        })
    }
}

/// Helper function to make converting scancodes
/// and keycodes to primitive `SDL_Keysym` types.
fn mk_keysym<S, K>(scancode: S,
             keycode: K,
             keymod: Mod) -> sys::SDL_Keysym
where S: Into<Option<Scancode>>,
      K: Into<Option<Keycode>>,
{
    let scancode = scancode.into()
        .map(|sc| unsafe { transmute::<u32, sys::SDL_Scancode>(sc as u32) })
        .unwrap_or(sys::SDL_Scancode::SDL_SCANCODE_UNKNOWN);
    let keycode = keycode.into()
        .map(|kc| kc as sys::SDL_Keycode)
        .unwrap_or(sys::SDLK_UNKNOWN as i32);
    let keymod = keymod.bits() as u16;
    sys::SDL_Keysym {
        scancode: scancode,
        sym: keycode,
        mod_: keymod,
        unused: 0,
    }
}

// TODO: Remove this when from_utf8 is updated in Rust
// This would honestly be nice if it took &self instead of self,
// but Event::User's raw pointers kind of removes that possibility.
impl Event {
    fn to_ll(&self) -> Option<sys::SDL_Event> {
        let mut ret = unsafe { mem::uninitialized() };
        match *self {
            Event::User { window_id, type_, code, data1, data2, timestamp} => {
                let event = sys::SDL_UserEvent {
                    type_: type_ as uint32_t,
                    timestamp: timestamp,
                    windowID: window_id,
                    code: code as i32,
                    data1: data1,
                    data2: data2
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_UserEvent, 1);
                }
                Some(ret)
            },

            Event::Quit{timestamp} => {
                let event = sys::SDL_QuitEvent {
                    type_: sys::SDL_EventType::SDL_QUIT as u32,
                    timestamp: timestamp,
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_QuitEvent, 1);
                }
                Some(ret)
            },

            Event::Window{
                timestamp,
                window_id,
                win_event
            } => {
                let (win_event_id, data1, data2) = win_event.to_ll();
                let event = sys::SDL_WindowEvent {
                    type_: sys::SDL_EventType::SDL_WINDOWEVENT as u32,
                    timestamp: timestamp,
                    windowID: window_id,
                    event: win_event_id,
                    padding1: 0,
                    padding2: 0,
                    padding3: 0,
                    data1: data1,
                    data2: data2,
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_WindowEvent, 1);
                }
                Some(ret)
            },

            Event::KeyDown{
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
            } => {
                let keysym = mk_keysym(scancode, keycode, keymod);
                let event = sys::SDL_KeyboardEvent{
                    type_: sys::SDL_EventType::SDL_KEYDOWN as u32,
                    timestamp: timestamp,
                    windowID: window_id,
                    state: sys::SDL_PRESSED as u8,
                    repeat: repeat as u8,
                    padding2: 0,
                    padding3: 0,
                    keysym: keysym,
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_KeyboardEvent, 1);
                }
                Some(ret)
            },
            Event::KeyUp {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
            } => {
                let keysym = mk_keysym(scancode, keycode, keymod);
                let event = sys::SDL_KeyboardEvent{
                    type_: sys::SDL_EventType::SDL_KEYUP as u32,
                    timestamp: timestamp,
                    windowID: window_id,
                    state: sys::SDL_RELEASED as u8,
                    repeat: repeat as u8,
                    padding2: 0,
                    padding3: 0,
                    keysym: keysym,
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_KeyboardEvent, 1);
                }
                Some(ret)
            },
            Event::MouseMotion{
                timestamp,
                window_id,
                which,
                mousestate,
                x,
                y,
                xrel,
                yrel
            } => {
                let state = mousestate.to_sdl_state();
                let event = sys::SDL_MouseMotionEvent {
                    type_: sys::SDL_EventType::SDL_MOUSEMOTION as u32,
                    timestamp: timestamp,
                    windowID: window_id,
                    which: which,
                    state: state,
                    x: x,
                    y: y,
                    xrel: xrel,
                    yrel: yrel,
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_MouseMotionEvent, 1);
                }
                Some(ret)

            },
            Event::MouseButtonDown{
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y
            } => {
                let event = sys::SDL_MouseButtonEvent {
                    type_: sys::SDL_EventType::SDL_MOUSEBUTTONDOWN as u32,
                    timestamp: timestamp,
                    windowID: window_id,
                    which: which,
                    button: mouse_btn as u8,
                    state: sys::SDL_PRESSED as u8,
                    clicks: clicks,
                    padding1: 0,
                    x: x,
                    y: y
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_MouseButtonEvent, 1);
                }
                Some(ret)
            },
            Event::MouseButtonUp{
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y
            } => {
                let event = sys::SDL_MouseButtonEvent {
                    type_: sys::SDL_EventType::SDL_MOUSEBUTTONUP as u32,
                    timestamp: timestamp,
                    windowID: window_id,
                    which: which,
                    button: mouse_btn as u8,
                    state: sys::SDL_RELEASED as u8,
                    clicks: clicks,
                    padding1: 0,
                    x: x,
                    y: y
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_MouseButtonEvent, 1);
                }
                Some(ret)
            },

            Event::MouseWheel{
                timestamp,
                window_id,
                which,
                x,
                y,
                direction,
            } => {
                let event = sys::SDL_MouseWheelEvent {
                    type_: sys::SDL_EventType::SDL_MOUSEWHEEL as u32,
                    timestamp: timestamp,
                    windowID: window_id,
                    which: which,
                    x: x,
                    y: y,
                    direction : direction.to_ll(),
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_MouseWheelEvent, 1);
                }
                Some(ret)
            },
            Event::JoyAxisMotion{
                timestamp,
                which,
                axis_idx,
                value
            } => {
                let event = sys::SDL_JoyAxisEvent {
                    type_: sys::SDL_EventType::SDL_JOYAXISMOTION as u32,
                    timestamp: timestamp,
                    which: which,
                    axis: axis_idx,
                    value: value,
                    padding1: 0,
                    padding2: 0,
                    padding3: 0,
                    padding4: 0
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_JoyAxisEvent, 1);
                }
                Some(ret)

            },
            Event::JoyBallMotion{
                timestamp,
                which,
                ball_idx,
                xrel,
                yrel
            } => {
                let event = sys::SDL_JoyBallEvent {
                    type_: sys::SDL_EventType::SDL_JOYBALLMOTION as u32,
                    timestamp: timestamp,
                    which: which,
                    ball: ball_idx,
                    xrel: xrel,
                    yrel: yrel,
                    padding1: 0,
                    padding2: 0,
                    padding3: 0
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_JoyBallEvent, 1);
                }
                Some(ret)

            },
            Event::JoyHatMotion{
                timestamp,
                which,
                hat_idx,
                state,
            } => {
                let hatvalue = state.to_raw();
                let event = sys::SDL_JoyHatEvent {
                    type_: sys::SDL_EventType::SDL_JOYHATMOTION as u32,
                    timestamp: timestamp,
                    which: which,
                    hat: hat_idx,
                    value: hatvalue,
                    padding1: 0,
                    padding2: 0
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_JoyHatEvent, 1);
                }
                Some(ret)

            },
            Event::JoyButtonDown{
                timestamp,
                which,
                button_idx
            } => {
                let event = sys::SDL_JoyButtonEvent {
                    type_: sys::SDL_EventType::SDL_JOYBUTTONDOWN as u32,
                    timestamp: timestamp,
                    which: which,
                    button: button_idx,
                    state: sys::SDL_PRESSED as u8,
                    padding1: 0,
                    padding2: 0,
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_JoyButtonEvent, 1);
                }
                Some(ret)

            },

            Event::JoyButtonUp{
                timestamp,
                which,
                button_idx,
            } => {
                let event = sys::SDL_JoyButtonEvent {
                    type_: sys::SDL_EventType::SDL_JOYBUTTONUP as u32,
                    timestamp: timestamp,
                    which: which,
                    button: button_idx,
                    state: sys::SDL_RELEASED as u8,
                    padding1: 0,
                    padding2: 0,
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_JoyButtonEvent, 1);
                }
                Some(ret)

            },

            Event::JoyDeviceAdded{
                timestamp,
                which,
            } => {
                let event = sys::SDL_JoyDeviceEvent {
                    type_: sys::SDL_EventType::SDL_JOYDEVICEADDED as u32,
                    timestamp: timestamp,
                    which: which as i32,
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_JoyDeviceEvent, 1);
                }
                Some(ret)
            },

            Event::JoyDeviceRemoved{
                timestamp,
                which,
            } => {
                let event = sys::SDL_JoyDeviceEvent {
                    type_: sys::SDL_EventType::SDL_JOYDEVICEREMOVED as u32,
                    timestamp: timestamp,
                    which: which,
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_JoyDeviceEvent, 1);
                }
                Some(ret)

            },
            Event::ControllerAxisMotion{
                timestamp,
                which,
                axis,
                value,
            } => {
                let axisval = axis.to_ll();
                let event = sys::SDL_ControllerAxisEvent {
                    type_: sys::SDL_EventType::SDL_CONTROLLERAXISMOTION as u32,
                    timestamp: timestamp,
                    which: which,
                    axis: axisval as u8,
                    value: value,
                    padding1: 0,
                    padding2: 0,
                    padding3: 0,
                    padding4: 0,
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_ControllerAxisEvent, 1);
                }
                Some(ret)
            },
            Event::ControllerButtonDown{
                timestamp,
                which,
                button,
            } => {
                let buttonval = button.to_ll();
                let event = sys::SDL_ControllerButtonEvent {
                    type_: sys::SDL_EventType::SDL_CONTROLLERBUTTONDOWN as u32,
                    timestamp: timestamp,
                    which: which,
                    // This conversion turns an i32 into a u8; signed-to-unsigned conversions
                    // are a bit of a code smellx, but that appears to be how SDL defines it.
                    button: buttonval as u8,
                    state: sys::SDL_PRESSED as u8,
                    padding1: 0,
                    padding2: 0,
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_ControllerButtonEvent, 1);
                }
                Some(ret)
            },

            Event::ControllerButtonUp{
                timestamp,
                which,
                button,
            } => {
                let buttonval = button.to_ll();
                let event = sys::SDL_ControllerButtonEvent {
                    type_: sys::SDL_EventType::SDL_CONTROLLERBUTTONUP as u32,
                    timestamp: timestamp,
                    which: which,
                    button: buttonval as u8,
                    state: sys::SDL_RELEASED as u8,
                    padding1: 0,
                    padding2: 0,
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_ControllerButtonEvent, 1);
                }
                Some(ret)
            },

            Event::ControllerDeviceAdded{
                timestamp,
                which,
            } => {
                let event = sys::SDL_ControllerDeviceEvent {
                    type_: sys::SDL_EventType::SDL_CONTROLLERDEVICEADDED as u32,
                    timestamp: timestamp,
                    which: which as i32,
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_ControllerDeviceEvent, 1);
                }
                Some(ret)

            },

            Event::ControllerDeviceRemoved{
                timestamp,
                which,
            } => {
                let event = sys::SDL_ControllerDeviceEvent {
                    type_: sys::SDL_EventType::SDL_CONTROLLERDEVICEREMOVED as u32,
                    timestamp: timestamp,
                    which: which,
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_ControllerDeviceEvent, 1);
                }
                Some(ret)

            },

            Event::ControllerDeviceRemapped{
                timestamp,
                which,
            } => {
                let event = sys::SDL_ControllerDeviceEvent {
                    type_: sys::SDL_EventType::SDL_CONTROLLERDEVICEREMAPPED as u32,
                    timestamp: timestamp,
                    which: which,
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut sys::SDL_Event as *mut sys::SDL_ControllerDeviceEvent, 1);
                }
                Some(ret)

            },


            Event::FingerDown{..} |
            Event::FingerUp{..} |
            Event::FingerMotion{..} |
            Event::DollarGesture{..} |
            Event::DollarRecord{..} |
            Event::MultiGesture{..} |
            Event::ClipboardUpdate{..} |
            Event::DropFile{..} |
            Event::TextEditing{..} |
            Event::TextInput{..} |
            Event::Unknown{..} |
            _ => {
                // don't know how to convert!
                None
            }
        }
    }

    fn from_ll(raw: sys::SDL_Event) -> Event {
        let raw_type = unsafe { raw.type_ };

        // if event type has not been defined, treat it as a UserEvent
        let event_type: EventType = FromPrimitive::from_usize(raw_type as usize).unwrap_or(EventType::User);
        unsafe { match event_type {
            EventType::Quit => {
                let event = raw.quit;
                Event::Quit { timestamp: event.timestamp }
            }
            EventType::AppTerminating => {
                let event = raw.common;
                Event::AppTerminating { timestamp: event.timestamp }
            }
            EventType::AppLowMemory => {
                let event = raw.common;
                Event::AppLowMemory { timestamp: event.timestamp }
            }
            EventType::AppWillEnterBackground => {
                let event = raw.common;
                Event::AppWillEnterBackground { timestamp: event.timestamp }
            }
            EventType::AppDidEnterBackground => {
                let event = raw.common;
                Event::AppDidEnterBackground { timestamp: event.timestamp }
            }
            EventType::AppWillEnterForeground => {
                let event = raw.common;
                Event::AppWillEnterForeground { timestamp: event.timestamp }
            }
            EventType::AppDidEnterForeground => {
                let event = raw.common;
                Event::AppDidEnterForeground { timestamp: event.timestamp }
            }

            EventType::Window => {
                let event = raw.window;

                Event::Window {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    win_event: WindowEvent::from_ll(event.event, event.data1, event.data2),
                }
            }
            // TODO: SysWMEventType

            EventType::KeyDown => {
                let event = raw.key;
                Event::KeyDown {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    keycode: Keycode::from_i32(event.keysym.sym as i32),
                    scancode: Scancode::from_i32(event.keysym.scancode as i32),
                    keymod: keyboard::Mod::from_bits(event.keysym.mod_).unwrap(),
                    repeat: event.repeat != 0
                }
            }
            EventType::KeyUp => {
                let event = raw.key;

                Event::KeyUp {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    keycode: Keycode::from_i32(event.keysym.sym as i32),
                    scancode: Scancode::from_i32(event.keysym.scancode as i32),
                    keymod: keyboard::Mod::from_bits(event.keysym.mod_).unwrap(),
                    repeat: event.repeat != 0
                }
            }
            EventType::TextEditing => {
                let event = raw.edit;

                let text = String::from_utf8(
                    event.text.iter()
                    .take_while(|&b| (*b) != 0)
                    .map(|&b| b as u8)
                    .collect::<Vec<u8>>()
                ).expect("Invalid TextEditing string");
                Event::TextEditing {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    text: text,
                    start: event.start,
                    length: event.length
                }
            }
            EventType::TextInput => {
                let event = raw.text;

                let text = String::from_utf8(
                        event.text.iter()
                            .take_while(|&b| (*b) != 0)
                            .map(|&b| b as u8)
                            .collect::<Vec<u8>>()
                    ).expect("Invalid TextInput string");
                Event::TextInput {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    text: text
                }
            }

            EventType::MouseMotion => {
                let event = raw.motion;

                Event::MouseMotion {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    which: event.which,
                    mousestate: mouse::MouseState::from_sdl_state(event.state),
                    x: event.x,
                    y: event.y,
                    xrel: event.xrel,
                    yrel: event.yrel
                }
            }
            EventType::MouseButtonDown => {
                let event = raw.button;

                Event::MouseButtonDown {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    which: event.which,
                    mouse_btn: mouse::MouseButton::from_ll(event.button),
                    clicks: event.clicks,
                    x: event.x,
                    y: event.y
                }
            }
            EventType::MouseButtonUp => {
                let event = raw.button;

                Event::MouseButtonUp {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    which: event.which,
                    mouse_btn: mouse::MouseButton::from_ll(event.button),
                    clicks: event.clicks,
                    x: event.x,
                    y: event.y
                }
            }
            EventType::MouseWheel => {
                let event = raw.wheel;

                Event::MouseWheel {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    which: event.which,
                    x: event.x,
                    y: event.y,
                    direction: mouse::MouseWheelDirection::from_ll(event.direction),
                }
            }

            EventType::JoyAxisMotion => {
                let event = raw.jaxis;
                Event::JoyAxisMotion {
                    timestamp: event.timestamp,
                    which: event.which,
                    axis_idx: event.axis,
                    value: event.value
                }
            }
            EventType::JoyBallMotion => {
                let event = raw.jball;
                Event::JoyBallMotion {
                    timestamp: event.timestamp,
                    which: event.which,
                    ball_idx: event.ball,
                    xrel: event.xrel,
                    yrel: event.yrel
                }
            }
            EventType::JoyHatMotion => {
                let event = raw.jhat;
                Event::JoyHatMotion {
                    timestamp: event.timestamp,
                    which: event.which,
                    hat_idx: event.hat,
                    state: joystick::HatState::from_raw(event.value),
                }
            }
            EventType::JoyButtonDown => {
                let event = raw.jbutton;
                Event::JoyButtonDown {
                    timestamp: event.timestamp,
                    which: event.which,
                    button_idx: event.button
                }
            }
            EventType::JoyButtonUp => {
                let event = raw.jbutton;
                Event::JoyButtonUp {
                    timestamp: event.timestamp,
                    which: event.which,
                    button_idx: event.button
                }
            }
            EventType::JoyDeviceAdded => {
                let event = raw.jdevice;
                Event::JoyDeviceAdded {
                    timestamp: event.timestamp,
                    which: event.which as u32
                }
            }
            EventType::JoyDeviceRemoved => {
                let event = raw.jdevice;
                Event::JoyDeviceRemoved {
                    timestamp: event.timestamp,
                    which: event.which
                }
            }

            EventType::ControllerAxisMotion => {
                let event = raw.caxis;
                let axis = controller::Axis::from_ll(transmute(event.axis as i32)).unwrap();

                Event::ControllerAxisMotion {
                    timestamp: event.timestamp,
                    which: event.which,
                    axis: axis,
                    value: event.value
                }
            }
            EventType::ControllerButtonDown => {
                let event = raw.cbutton;
                let button = controller::Button::from_ll(transmute(event.button as i32)).unwrap();

                Event::ControllerButtonDown {
                    timestamp: event.timestamp,
                    which: event.which,
                    button: button
                }
            }
            EventType::ControllerButtonUp => {
                let event = raw.cbutton;
                let button = controller::Button::from_ll(transmute(event.button as i32)).unwrap();

                Event::ControllerButtonUp {
                    timestamp: event.timestamp,
                    which: event.which,
                    button: button
                }
            }
            EventType::ControllerDeviceAdded => {
                let event = raw.cdevice;
                Event::ControllerDeviceAdded {
                    timestamp: event.timestamp,
                    which: event.which as u32
                }
            }
            EventType::ControllerDeviceRemoved => {
                let event = raw.cdevice;
                Event::ControllerDeviceRemoved {
                    timestamp: event.timestamp,
                    which: event.which
                }
            }
            EventType::ControllerDeviceRemapped => {
                let event = raw.cdevice;
                Event::ControllerDeviceRemapped {
                    timestamp: event.timestamp,
                    which: event.which
                }
            }

            EventType::FingerDown => {
                let event = raw.tfinger;
                Event::FingerDown {
                    timestamp: event.timestamp,
                    touch_id: event.touchId,
                    finger_id: event.fingerId,
                    x: event.x,
                    y: event.y,
                    dx: event.dx,
                    dy: event.dy,
                    pressure: event.pressure
                }
            }
            EventType::FingerUp => {
                let event = raw.tfinger;
                Event::FingerUp {
                    timestamp: event.timestamp,
                    touch_id: event.touchId,
                    finger_id: event.fingerId,
                    x: event.x,
                    y: event.y,
                    dx: event.dx,
                    dy: event.dy,
                    pressure: event.pressure
                }
            }
            EventType::FingerMotion => {
                let event = raw.tfinger;
                Event::FingerMotion {
                    timestamp: event.timestamp,
                    touch_id: event.touchId,
                    finger_id: event.fingerId,
                    x: event.x,
                    y: event.y,
                    dx: event.dx,
                    dy: event.dy,
                    pressure: event.pressure
                }
            }
            EventType::DollarGesture => {
                let event = raw.dgesture;
                Event::DollarGesture {
                    timestamp: event.timestamp,
                    touch_id: event.touchId,
                    gesture_id: event.gestureId,
                    num_fingers: event.numFingers,
                    error: event.error,
                    x: event.x,
                    y: event.y
                }
            }
            EventType::DollarRecord => {
                let event = raw.dgesture;
                Event::DollarRecord {
                    timestamp: event.timestamp,
                    touch_id: event.touchId,
                    gesture_id: event.gestureId,
                    num_fingers: event.numFingers,
                    error: event.error,
                    x: event.x,
                    y: event.y
                }
            }
            EventType::MultiGesture => {
                let event = raw.mgesture;
                Event::MultiGesture {
                    timestamp: event.timestamp,
                    touch_id: event.touchId,
                    d_theta: event.dTheta,
                    d_dist: event.dDist,
                    x: event.x,
                    y: event.y,
                    num_fingers: event.numFingers
                }
            }

            EventType::ClipboardUpdate => {
                let event = raw.common;
                Event::ClipboardUpdate {
                    timestamp: event.timestamp
                }
            }
            EventType::DropFile => {
                let event = raw.drop;

                let buf = CStr::from_ptr(event.file as *const _).to_bytes();
                let text = String::from_utf8_lossy(buf).to_string();
                sys::SDL_free(event.file as *mut c_void);

                Event::DropFile {
                    timestamp: event.timestamp,
                    filename: text
                }
            }

            EventType::First => panic!("Unused event, EventType::First, was encountered"),
            EventType::Last => panic!("Unusable event, EventType::Last, was encountered"),

            // If we have no other match and the event type is >= 32768
            // this is a user event
            EventType::User => {
                if raw_type < 32_768 {
                    // The type is unknown to us.
                    // It's a newer SDL2 type.
                    let event = raw.common;

                    Event::Unknown {
                        timestamp: event.timestamp,
                        type_: event.type_
                    }
                } else {
                    let event = raw.user;

                    Event::User {
                        timestamp: event.timestamp,
                        window_id: event.windowID,
                        type_: raw_type,
                        code: event.code,
                        data1: event.data1,
                        data2: event.data2
                    }
                }
            }
        }}                      // close unsafe & match
    }

    pub fn is_user_event(&self) -> bool {
        match *self {
            Event::User { .. } => true,
            _ => false
        }
    }

    pub fn as_user_event_type<T: ::std::any::Any>(&self) -> Option<T> {
        use ::std::any::TypeId;
        let type_id = TypeId::of::<Box<T>>();

        let (event_id, event_box_ptr) = match *self {
            Event::User { type_, data1, .. } => { (type_, data1) },
            _ => { return None }
        };

        let cet = CUSTOM_EVENT_TYPES.lock().unwrap();

        let event_type_id = match cet.sdl_id_to_type_id.get(&event_id) {
            Some(id) => id,
            None => { panic!("internal error; could not find typeid") }
        };

        if &type_id != event_type_id {
            return None;
        }

        let event_box : Box<T> = unsafe { Box::from_raw(event_box_ptr as *mut T) };

        Some(*event_box)
    }
}

unsafe fn poll_event() -> Option<Event> {
    let mut raw = mem::uninitialized();
    let has_pending = sys::SDL_PollEvent(&mut raw) == 1;

    if has_pending { Some(Event::from_ll(raw)) }
    else { None }
}

unsafe fn wait_event() -> Event {
    let mut raw = mem::uninitialized();
    let success = sys::SDL_WaitEvent(&mut raw) == 1;

    if success { Event::from_ll(raw) }
    else { panic!(get_error()) }
}

unsafe fn wait_event_timeout(timeout: u32) -> Option<Event> {
    let mut raw = mem::uninitialized();
    let success = sys::SDL_WaitEventTimeout(&mut raw, timeout as c_int) == 1;

    if success { Some(Event::from_ll(raw)) }
    else { None }
}

impl ::EventPump {
    /// Query if an event type is enabled.
    pub fn is_event_enabled(&self, event_type: EventType) -> bool {
        let result = unsafe { sys::SDL_EventState(event_type as u32, sys::SDL_QUERY) };

        result != sys::SDL_DISABLE as u8
    }

    /// Enable an event type. Returns if the event type was enabled before the call.
    pub fn enable_event(&mut self, event_type: EventType) -> bool {
        let result = unsafe { sys::SDL_EventState(event_type as u32, sys::SDL_ENABLE as c_int) };

        result != sys::SDL_DISABLE as u8
    }

    /// Disable an event type. Returns if the event type was enabled before the call.
    pub fn disable_event(&mut self, event_type: EventType) -> bool {
        let result = unsafe { sys::SDL_EventState(event_type as u32, sys::SDL_DISABLE as c_int) };

        result != sys::SDL_DISABLE as u8
    }

    /// Polls for currently pending events.
    ///
    /// If no events are pending, `None` is returned.
    pub fn poll_event(&mut self) -> Option<Event> {
        unsafe { poll_event() }
    }

    /// Returns a polling iterator that calls `poll_event()`.
    /// The iterator will terminate once there are no more pending events.
    ///
    /// # Example
    /// ```no_run
    /// let sdl_context = sdl2::init().unwrap();
    /// let mut event_pump = sdl_context.event_pump().unwrap();
    ///
    /// for event in event_pump.poll_iter() {
    ///     use sdl2::event::Event;
    ///     match event {
    ///         Event::KeyDown {..} => { /*...*/ },
    ///         _ => ()
    ///     }
    /// }
    /// ```
    pub fn poll_iter(&mut self) -> EventPollIterator {
        EventPollIterator {
            _marker: PhantomData
        }
    }

    /// Pumps the event loop, gathering events from the input devices.
    pub fn pump_events(&mut self) {
        unsafe { sys::SDL_PumpEvents(); };
    }

    /// Waits indefinitely for the next available event.
    pub fn wait_event(&mut self) -> Event {
        unsafe { wait_event() }
    }

    /// Waits until the specified timeout (in milliseconds) for the next available event.
    pub fn wait_event_timeout(&mut self, timeout: u32) -> Option<Event> {
        unsafe { wait_event_timeout(timeout) }
    }

    /// Returns a waiting iterator that calls `wait_event()`.
    ///
    /// Note: The iterator will never terminate.
    pub fn wait_iter(&mut self) -> EventWaitIterator {
        EventWaitIterator {
            _marker: PhantomData
        }
    }

    /// Returns a waiting iterator that calls `wait_event_timeout()`.
    ///
    /// Note: The iterator will never terminate, unless waiting for an event
    /// exceeds the specified timeout.
    pub fn wait_timeout_iter(&mut self, timeout: u32) -> EventWaitTimeoutIterator {
        EventWaitTimeoutIterator {
            _marker: PhantomData,
            timeout: timeout
        }
    }

    #[inline]
    pub fn keyboard_state(&self) -> ::keyboard::KeyboardState {
        ::keyboard::KeyboardState::new(self)
    }

    #[inline]
    pub fn mouse_state(&self) -> ::mouse::MouseState {
        ::mouse::MouseState::new(self)
    }

    #[inline]
    pub fn relative_mouse_state(&self) -> ::mouse::RelativeMouseState {
        ::mouse::RelativeMouseState::new(self)
    }
}

/// An iterator that calls `EventPump::poll_event()`.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EventPollIterator<'a> {
    _marker: PhantomData<&'a ()>
}

impl<'a> Iterator for EventPollIterator<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> { unsafe { poll_event() } }
}

/// An iterator that calls `EventPump::wait_event()`.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EventWaitIterator<'a> {
    _marker: PhantomData<&'a ()>
}

impl<'a> Iterator for EventWaitIterator<'a> {
    type Item = Event;
    fn next(&mut self) -> Option<Event> { unsafe { Some(wait_event()) } }
}

/// An iterator that calls `EventPump::wait_event_timeout()`.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EventWaitTimeoutIterator<'a> {
    _marker: PhantomData<&'a ()>,
    timeout: u32
}

impl<'a> Iterator for EventWaitTimeoutIterator<'a> {
    type Item = Event;
    fn next(&mut self) -> Option<Event> { unsafe { wait_event_timeout(self.timeout) } }
}

#[cfg(test)]
mod test {
    use super::Event;
    use super::WindowEvent;
    use super::super::controller::{Button, Axis};
    use super::super::joystick::{HatState};
    use super::super::mouse::{MouseButton, MouseState, MouseWheelDirection};
    use super::super::keyboard::{Keycode, Scancode, Mod};

    // Tests a round-trip conversion from an Event type to
    // the SDL event type and back, to make sure it's sane.
    #[test]
    fn test_to_from_ll() {
        {
            let e = Event::Quit{timestamp: 0};
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::Window{
                timestamp: 0,
                window_id: 0,
                win_event: WindowEvent::Resized(1, 2),
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::KeyDown{
                timestamp: 0,
                window_id: 1,
                keycode: None,
                scancode: Some(Scancode::Q),
                keymod: Mod::all(),
                repeat: false,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::KeyUp{
                timestamp: 123,
                window_id: 0,
                keycode: Some(Keycode::R),
                scancode: Some(Scancode::R),
                keymod: Mod::empty(),
                repeat: true,

            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::MouseMotion{
                timestamp: 0,
                window_id: 0,
                which: 1,
                mousestate: MouseState::from_sdl_state(1),
                x: 3,
                y: 91,
                xrel: -1,
                yrel: 43,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::MouseButtonDown{
                timestamp: 5634,
                window_id: 2,
                which: 0,
                mouse_btn: MouseButton::Left,
                clicks: 1,
                x: 543,
                y: 345,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::MouseButtonUp{
                timestamp: 0,
                window_id: 2,
                which: 0,
                mouse_btn: MouseButton::Left,
                clicks: 1,
                x: 543,
                y: 345,

            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::MouseWheel{
                timestamp: 1,
                window_id: 0,
                which: 32,
                x: 23,
                y: 91,
                direction: MouseWheelDirection::Flipped,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::JoyAxisMotion{
                timestamp: 0,
                which: 1,
                axis_idx: 1,
                value: 12,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::JoyBallMotion{
                timestamp: 0,
                which: 0,
                ball_idx: 1,
                xrel: 123,
                yrel: 321,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::JoyHatMotion{
                timestamp: 0,
                which: 3,
                hat_idx: 1,
                state: HatState::Left,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::JoyButtonDown{
                timestamp: 0,
                which: 0,
                button_idx: 3
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::JoyButtonUp{
                timestamp: 9876,
                which: 1,
                button_idx: 2,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::JoyDeviceAdded{
                timestamp: 0,
                which: 1
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::JoyDeviceRemoved{
                timestamp: 0,
                which: 2,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::ControllerAxisMotion{
                timestamp: 53,
                which: 0,
                axis: Axis::LeftX,
                value: 3
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::ControllerButtonDown{
                timestamp: 0,
                which: 1,
                button: Button::Guide,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::ControllerButtonUp{
                timestamp: 654214,
                which: 0,
                button: Button::DPadRight,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::ControllerDeviceAdded{
                timestamp: 543,
                which: 3
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::ControllerDeviceRemoved{
                timestamp: 555,
                which: 3
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::ControllerDeviceRemapped{
                timestamp: 654,
                which: 0
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }

    }
}
