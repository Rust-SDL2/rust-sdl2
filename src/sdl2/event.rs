/*!
Event Handling
 */

use std::borrow::ToOwned;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ffi::CStr;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::mem;
use std::mem::transmute;
use std::ptr;
use std::sync::Mutex;

use libc::c_int;
use libc::c_void;

use crate::controller;
use crate::controller::{Axis, Button};
use crate::get_error;
use crate::joystick;
use crate::joystick::HatState;
use crate::keyboard;
use crate::keyboard::Keycode;
use crate::keyboard::Mod;
use crate::keyboard::Scancode;
use crate::mouse;
use crate::mouse::{MouseButton, MouseState, MouseWheelDirection};
use crate::sys;
use crate::sys::SDL_EventFilter;
use crate::sys::SDL_EventType;
use crate::video::Orientation;

struct CustomEventTypeMaps {
    sdl_id_to_type_id: HashMap<u32, ::std::any::TypeId>,
    type_id_to_sdl_id: HashMap<::std::any::TypeId, u32>,
}

impl CustomEventTypeMaps {
    fn new() -> Self {
        CustomEventTypeMaps {
            sdl_id_to_type_id: HashMap::new(),
            type_id_to_sdl_id: HashMap::new(),
        }
    }
}

lazy_static! {
    static ref CUSTOM_EVENT_TYPES: Mutex<CustomEventTypeMaps> =
        Mutex::new(CustomEventTypeMaps::new());
}

impl crate::EventSubsystem {
    /// Removes all events in the event queue that match the specified event type.
    #[doc(alias = "SDL_FlushEvent")]
    pub fn flush_event(&self, event_type: EventType) {
        unsafe { sys::SDL_FlushEvent(event_type as u32) };
    }

    /// Removes all events in the event queue that match the specified type range.
    #[doc(alias = "SDL_FlushEvents")]
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
    #[doc(alias = "SDL_PeepEvents")]
    pub fn peek_events<B>(&self, max_amount: u32) -> B
    where
        B: FromIterator<Event>,
    {
        unsafe {
            let mut events = Vec::with_capacity(max_amount as usize);

            let result = {
                let events_ptr = events.as_mut_ptr();

                sys::SDL_PeepEvents(
                    events_ptr,
                    max_amount as c_int,
                    sys::SDL_eventaction::SDL_PEEKEVENT,
                    SDL_EventType::SDL_FIRSTEVENT as u32,
                    SDL_EventType::SDL_LASTEVENT as u32,
                )
            };

            if result < 0 {
                // The only error possible is "Couldn't lock event queue"
                panic!("{}", get_error());
            } else {
                events.set_len(result as usize);

                events
                    .into_iter()
                    .map(|event_raw| Event::from_ll(event_raw))
                    .collect()
            }
        }
    }

    /// Pushes an event to the event queue.
    pub fn push_event(&self, event: Event) -> Result<(), String> {
        self.event_sender().push_event(event)
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
    ///    data1: 0x1234 as *mut libc::c_void,
    ///    data2: 0x5678 as *mut libc::c_void,
    /// };
    ///
    /// ev.push_event(event);
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn register_event(&self) -> Result<u32, String> {
        Ok(*self.register_events(1)?.first().unwrap())
    }

    /// Registers custom SDL events.
    ///
    /// Returns an error, if no more user events can be created.
    pub unsafe fn register_events(&self, nr: u32) -> Result<Vec<u32>, String> {
        let result = sys::SDL_RegisterEvents(nr as ::libc::c_int);
        const ERR_NR: u32 = ::std::u32::MAX - 1;

        match result {
            ERR_NR => Err("No more user events can be created; SDL_LASTEVENT reached".to_owned()),
            _ => {
                let event_ids = (result..(result + nr)).collect();
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
    pub fn register_custom_event<T: ::std::any::Any>(&self) -> Result<(), String> {
        use std::any::TypeId;
        let event_id = *(unsafe { self.register_events(1) })?.first().unwrap();
        let mut cet = CUSTOM_EVENT_TYPES.lock().unwrap();
        let type_id = TypeId::of::<Box<T>>();

        if cet.type_id_to_sdl_id.contains_key(&type_id) {
            return Err("The same event type can not be registered twice!".to_owned());
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
    pub fn push_custom_event<T: ::std::any::Any>(&self, event: T) -> Result<(), String> {
        self.event_sender().push_custom_event(event)
    }

    /// Create an event sender that can be sent to other threads.
    ///
    /// An `EventSender` will not keep the event subsystem alive. If the event subsystem is
    /// shut down calls to `push_event` and `push_custom_event` will return errors.
    pub fn event_sender(&self) -> EventSender {
        EventSender { _priv: () }
    }

    /// Create an event watcher which is called every time an event is added to event queue.
    ///
    /// The watcher is disabled when the return value is dropped.
    /// Just calling this function without binding to a variable immediately disables the watcher.
    /// In order to make it persistent, you have to bind in a variable and keep it until it's no
    /// longer needed.
    ///
    /// # Example: dump every event to stderr
    /// ```
    /// let sdl = sdl2::init().unwrap();
    /// let ev = sdl.event().unwrap();
    ///
    /// // `let _ = ...` is insufficient, as it is dropped immediately.
    /// let _event_watch = ev.add_event_watch(|event| {
    ///     dbg!(event);
    /// });
    /// ```
    pub fn add_event_watch<'a, CB: EventWatchCallback + 'a>(
        &self,
        callback: CB,
    ) -> EventWatch<'a, CB> {
        EventWatch::add(callback)
    }
}

/// Types of events that can be delivered.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u32)]
pub enum EventType {
    First = SDL_EventType::SDL_FIRSTEVENT as u32,

    Quit = SDL_EventType::SDL_QUIT as u32,
    AppTerminating = SDL_EventType::SDL_APP_TERMINATING as u32,
    AppLowMemory = SDL_EventType::SDL_APP_LOWMEMORY as u32,
    AppWillEnterBackground = SDL_EventType::SDL_APP_WILLENTERBACKGROUND as u32,
    AppDidEnterBackground = SDL_EventType::SDL_APP_DIDENTERBACKGROUND as u32,
    AppWillEnterForeground = SDL_EventType::SDL_APP_WILLENTERFOREGROUND as u32,
    AppDidEnterForeground = SDL_EventType::SDL_APP_DIDENTERFOREGROUND as u32,

    Display = SDL_EventType::SDL_DISPLAYEVENT as u32,
    Window = SDL_EventType::SDL_WINDOWEVENT as u32,
    // TODO: SysWM = sys::SDL_SYSWMEVENT as u32,
    KeyDown = SDL_EventType::SDL_KEYDOWN as u32,
    KeyUp = SDL_EventType::SDL_KEYUP as u32,
    TextEditing = SDL_EventType::SDL_TEXTEDITING as u32,
    TextInput = SDL_EventType::SDL_TEXTINPUT as u32,

    MouseMotion = SDL_EventType::SDL_MOUSEMOTION as u32,
    MouseButtonDown = SDL_EventType::SDL_MOUSEBUTTONDOWN as u32,
    MouseButtonUp = SDL_EventType::SDL_MOUSEBUTTONUP as u32,
    MouseWheel = SDL_EventType::SDL_MOUSEWHEEL as u32,

    JoyAxisMotion = SDL_EventType::SDL_JOYAXISMOTION as u32,
    JoyBallMotion = SDL_EventType::SDL_JOYBALLMOTION as u32,
    JoyHatMotion = SDL_EventType::SDL_JOYHATMOTION as u32,
    JoyButtonDown = SDL_EventType::SDL_JOYBUTTONDOWN as u32,
    JoyButtonUp = SDL_EventType::SDL_JOYBUTTONUP as u32,
    JoyDeviceAdded = SDL_EventType::SDL_JOYDEVICEADDED as u32,
    JoyDeviceRemoved = SDL_EventType::SDL_JOYDEVICEREMOVED as u32,

    ControllerAxisMotion = SDL_EventType::SDL_CONTROLLERAXISMOTION as u32,
    ControllerButtonDown = SDL_EventType::SDL_CONTROLLERBUTTONDOWN as u32,
    ControllerButtonUp = SDL_EventType::SDL_CONTROLLERBUTTONUP as u32,
    ControllerDeviceAdded = SDL_EventType::SDL_CONTROLLERDEVICEADDED as u32,
    ControllerDeviceRemoved = SDL_EventType::SDL_CONTROLLERDEVICEREMOVED as u32,
    ControllerDeviceRemapped = SDL_EventType::SDL_CONTROLLERDEVICEREMAPPED as u32,
    ControllerTouchpadDown = SDL_EventType::SDL_CONTROLLERTOUCHPADDOWN as u32,
    ControllerTouchpadMotion = SDL_EventType::SDL_CONTROLLERTOUCHPADMOTION as u32,
    ControllerTouchpadUp = SDL_EventType::SDL_CONTROLLERTOUCHPADUP as u32,
    #[cfg(feature = "hidapi")]
    ControllerSensorUpdated = SDL_EventType::SDL_CONTROLLERSENSORUPDATE as u32,

    FingerDown = SDL_EventType::SDL_FINGERDOWN as u32,
    FingerUp = SDL_EventType::SDL_FINGERUP as u32,
    FingerMotion = SDL_EventType::SDL_FINGERMOTION as u32,
    DollarGesture = SDL_EventType::SDL_DOLLARGESTURE as u32,
    DollarRecord = SDL_EventType::SDL_DOLLARRECORD as u32,
    MultiGesture = SDL_EventType::SDL_MULTIGESTURE as u32,

    ClipboardUpdate = SDL_EventType::SDL_CLIPBOARDUPDATE as u32,
    DropFile = SDL_EventType::SDL_DROPFILE as u32,
    DropText = SDL_EventType::SDL_DROPTEXT as u32,
    DropBegin = SDL_EventType::SDL_DROPBEGIN as u32,
    DropComplete = SDL_EventType::SDL_DROPCOMPLETE as u32,

    AudioDeviceAdded = SDL_EventType::SDL_AUDIODEVICEADDED as u32,
    AudioDeviceRemoved = SDL_EventType::SDL_AUDIODEVICEREMOVED as u32,

    RenderTargetsReset = SDL_EventType::SDL_RENDER_TARGETS_RESET as u32,
    RenderDeviceReset = SDL_EventType::SDL_RENDER_DEVICE_RESET as u32,

    User = SDL_EventType::SDL_USEREVENT as u32,
    Last = SDL_EventType::SDL_LASTEVENT as u32,
}

impl TryFrom<u32> for EventType {
    type Error = ();

    fn try_from(n: u32) -> Result<Self, Self::Error> {
        use self::EventType::*;
        use crate::sys::SDL_EventType::*;

        Ok(match unsafe { transmute(n) } {
            SDL_FIRSTEVENT => First,

            SDL_QUIT => Quit,
            SDL_APP_TERMINATING => AppTerminating,
            SDL_APP_LOWMEMORY => AppLowMemory,
            SDL_APP_WILLENTERBACKGROUND => AppWillEnterBackground,
            SDL_APP_DIDENTERBACKGROUND => AppDidEnterBackground,
            SDL_APP_WILLENTERFOREGROUND => AppWillEnterForeground,
            SDL_APP_DIDENTERFOREGROUND => AppDidEnterForeground,

            SDL_DISPLAYEVENT => Display,
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
            SDL_CONTROLLERTOUCHPADDOWN => ControllerTouchpadDown,
            SDL_CONTROLLERTOUCHPADMOTION => ControllerTouchpadMotion,
            SDL_CONTROLLERTOUCHPADUP => ControllerTouchpadUp,
            #[cfg(feature = "hidapi")]
            SDL_CONTROLLERSENSORUPDATE => ControllerSensorUpdated,

            SDL_FINGERDOWN => FingerDown,
            SDL_FINGERUP => FingerUp,
            SDL_FINGERMOTION => FingerMotion,
            SDL_DOLLARGESTURE => DollarGesture,
            SDL_DOLLARRECORD => DollarRecord,
            SDL_MULTIGESTURE => MultiGesture,

            SDL_CLIPBOARDUPDATE => ClipboardUpdate,
            SDL_DROPFILE => DropFile,
            SDL_DROPTEXT => DropText,
            SDL_DROPBEGIN => DropBegin,
            SDL_DROPCOMPLETE => DropComplete,

            SDL_AUDIODEVICEADDED => AudioDeviceAdded,
            SDL_AUDIODEVICEREMOVED => AudioDeviceRemoved,

            SDL_RENDER_TARGETS_RESET => RenderTargetsReset,
            SDL_RENDER_DEVICE_RESET => RenderDeviceReset,

            SDL_USEREVENT => User,
            SDL_LASTEVENT => Last,

            _ => return Err(()),
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
/// An enum of display events.
pub enum DisplayEvent {
    None,
    Orientation(Orientation),
    Connected,
    Disconnected,
}

impl DisplayEvent {
    #[allow(clippy::match_same_arms)]
    fn from_ll(id: u8, data1: i32) -> DisplayEvent {
        if id > sys::SDL_DisplayEventID::SDL_DISPLAYEVENT_DISCONNECTED as u8 {
            return DisplayEvent::None;
        }
        match unsafe { transmute(id as u32) } {
            sys::SDL_DisplayEventID::SDL_DISPLAYEVENT_NONE => DisplayEvent::None,
            sys::SDL_DisplayEventID::SDL_DISPLAYEVENT_ORIENTATION => {
                let orientation = if data1 as u32
                    > sys::SDL_DisplayOrientation::SDL_ORIENTATION_PORTRAIT_FLIPPED as u32
                {
                    Orientation::Unknown
                } else {
                    Orientation::from_ll(unsafe { transmute(data1 as u32) })
                };
                DisplayEvent::Orientation(orientation)
            }
            sys::SDL_DisplayEventID::SDL_DISPLAYEVENT_CONNECTED => DisplayEvent::Connected,
            sys::SDL_DisplayEventID::SDL_DISPLAYEVENT_DISCONNECTED => DisplayEvent::Disconnected,
        }
    }

    fn to_ll(&self) -> (u8, i32) {
        match *self {
            DisplayEvent::None => (sys::SDL_DisplayEventID::SDL_DISPLAYEVENT_NONE as u8, 0),
            DisplayEvent::Orientation(orientation) => (
                sys::SDL_DisplayEventID::SDL_DISPLAYEVENT_ORIENTATION as u8,
                orientation.to_ll() as i32,
            ),
            DisplayEvent::Connected => {
                (sys::SDL_DisplayEventID::SDL_DISPLAYEVENT_CONNECTED as u8, 0)
            }
            DisplayEvent::Disconnected => (
                sys::SDL_DisplayEventID::SDL_DISPLAYEVENT_DISCONNECTED as u8,
                0,
            ),
        }
    }

    pub fn is_same_kind_as(&self, other: &DisplayEvent) -> bool {
        match (self, other) {
            (Self::None, Self::None)
            | (Self::Orientation(_), Self::Orientation(_))
            | (Self::Connected, Self::Connected)
            | (Self::Disconnected, Self::Disconnected) => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
/// An enum of window events.
pub enum WindowEvent {
    None,
    Shown,
    Hidden,
    Exposed,
    Moved(i32, i32),
    Resized(i32, i32),
    SizeChanged(i32, i32),
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
    ICCProfChanged,
    DisplayChanged(i32),
}

impl WindowEvent {
    #[allow(clippy::match_same_arms)]
    fn from_ll(id: u8, data1: i32, data2: i32) -> WindowEvent {
        match id {
            0 => WindowEvent::None,
            1 => WindowEvent::Shown,
            2 => WindowEvent::Hidden,
            3 => WindowEvent::Exposed,
            4 => WindowEvent::Moved(data1, data2),
            5 => WindowEvent::Resized(data1, data2),
            6 => WindowEvent::SizeChanged(data1, data2),
            7 => WindowEvent::Minimized,
            8 => WindowEvent::Maximized,
            9 => WindowEvent::Restored,
            10 => WindowEvent::Enter,
            11 => WindowEvent::Leave,
            12 => WindowEvent::FocusGained,
            13 => WindowEvent::FocusLost,
            14 => WindowEvent::Close,
            15 => WindowEvent::TakeFocus,
            16 => WindowEvent::HitTest,
            17 => WindowEvent::ICCProfChanged,
            18 => WindowEvent::DisplayChanged(data1),
            _ => WindowEvent::None,
        }
    }

    fn to_ll(&self) -> (u8, i32, i32) {
        match *self {
            WindowEvent::None => (0, 0, 0),
            WindowEvent::Shown => (1, 0, 0),
            WindowEvent::Hidden => (2, 0, 0),
            WindowEvent::Exposed => (3, 0, 0),
            WindowEvent::Moved(d1, d2) => (4, d1, d2),
            WindowEvent::Resized(d1, d2) => (5, d1, d2),
            WindowEvent::SizeChanged(d1, d2) => (6, d1, d2),
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
            WindowEvent::ICCProfChanged => (17, 0, 0),
            WindowEvent::DisplayChanged(d1) => (18, d1, 0),
        }
    }

    pub fn is_same_kind_as(&self, other: &WindowEvent) -> bool {
        match (self, other) {
            (Self::None, Self::None)
            | (Self::Shown, Self::Shown)
            | (Self::Hidden, Self::Hidden)
            | (Self::Exposed, Self::Exposed)
            | (Self::Moved(_, _), Self::Moved(_, _))
            | (Self::Resized(_, _), Self::Resized(_, _))
            | (Self::SizeChanged(_, _), Self::SizeChanged(_, _))
            | (Self::Minimized, Self::Minimized)
            | (Self::Maximized, Self::Maximized)
            | (Self::Restored, Self::Restored)
            | (Self::Enter, Self::Enter)
            | (Self::Leave, Self::Leave)
            | (Self::FocusGained, Self::FocusGained)
            | (Self::FocusLost, Self::FocusLost)
            | (Self::Close, Self::Close)
            | (Self::TakeFocus, Self::TakeFocus)
            | (Self::HitTest, Self::HitTest)
            | (Self::ICCProfChanged, Self::ICCProfChanged)
            | (Self::DisplayChanged(_), Self::DisplayChanged(_)) => true,
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
/// Different event types.
pub enum Event {
    Quit {
        timestamp: u32,
    },
    AppTerminating {
        timestamp: u32,
    },
    AppLowMemory {
        timestamp: u32,
    },
    AppWillEnterBackground {
        timestamp: u32,
    },
    AppDidEnterBackground {
        timestamp: u32,
    },
    AppWillEnterForeground {
        timestamp: u32,
    },
    AppDidEnterForeground {
        timestamp: u32,
    },

    Display {
        timestamp: u32,
        display_index: i32,
        display_event: DisplayEvent,
    },
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
        repeat: bool,
    },
    KeyUp {
        timestamp: u32,
        window_id: u32,
        keycode: Option<Keycode>,
        scancode: Option<Scancode>,
        keymod: Mod,
        repeat: bool,
    },

    TextEditing {
        timestamp: u32,
        window_id: u32,
        text: String,
        start: i32,
        length: i32,
    },

    TextInput {
        timestamp: u32,
        window_id: u32,
        text: String,
    },

    MouseMotion {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mousestate: MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    },

    MouseButtonDown {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mouse_btn: MouseButton,
        clicks: u8,
        x: i32,
        y: i32,
    },
    MouseButtonUp {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mouse_btn: MouseButton,
        clicks: u8,
        x: i32,
        y: i32,
    },

    MouseWheel {
        timestamp: u32,
        window_id: u32,
        which: u32,
        x: i32,
        y: i32,
        direction: MouseWheelDirection,
        precise_x: f32,
        precise_y: f32,
    },

    JoyAxisMotion {
        timestamp: u32,
        /// The joystick's `id`
        which: u32,
        axis_idx: u8,
        value: i16,
    },

    JoyBallMotion {
        timestamp: u32,
        /// The joystick's `id`
        which: u32,
        ball_idx: u8,
        xrel: i16,
        yrel: i16,
    },

    JoyHatMotion {
        timestamp: u32,
        /// The joystick's `id`
        which: u32,
        hat_idx: u8,
        state: HatState,
    },

    JoyButtonDown {
        timestamp: u32,
        /// The joystick's `id`
        which: u32,
        button_idx: u8,
    },
    JoyButtonUp {
        timestamp: u32,
        /// The joystick's `id`
        which: u32,
        button_idx: u8,
    },

    JoyDeviceAdded {
        timestamp: u32,
        /// The newly added joystick's `joystick_index`
        which: u32,
    },
    JoyDeviceRemoved {
        timestamp: u32,
        /// The joystick's `id`
        which: u32,
    },

    ControllerAxisMotion {
        timestamp: u32,
        /// The controller's joystick `id`
        which: u32,
        axis: Axis,
        value: i16,
    },

    ControllerButtonDown {
        timestamp: u32,
        /// The controller's joystick `id`
        which: u32,
        button: Button,
    },
    ControllerButtonUp {
        timestamp: u32,
        /// The controller's joystick `id`
        which: u32,
        button: Button,
    },

    ControllerDeviceAdded {
        timestamp: u32,
        /// The newly added controller's `joystick_index`
        which: u32,
    },
    ControllerDeviceRemoved {
        timestamp: u32,
        /// The controller's joystick `id`
        which: u32,
    },
    ControllerDeviceRemapped {
        timestamp: u32,
        /// The controller's joystick `id`
        which: u32,
    },

    ControllerTouchpadDown {
        timestamp: u32,
        /// The joystick instance id
        which: u32,
        /// The index of the touchpad
        touchpad: u32,
        /// The index of the finger on the touchpad
        finger: u32,
        /// Normalized in the range 0...1 with 0 being on the left
        x: f32,
        /// Normalized in the range 0...1 with 0 being at the top
        y: f32,
        /// Normalized in the range 0...1
        pressure: f32,
    },
    ControllerTouchpadMotion {
        timestamp: u32,
        /// The joystick instance id
        which: u32,
        /// The index of the touchpad
        touchpad: u32,
        /// The index of the finger on the touchpad
        finger: u32,
        /// Normalized in the range 0...1 with 0 being on the left
        x: f32,
        /// Normalized in the range 0...1 with 0 being at the top
        y: f32,
        /// Normalized in the range 0...1
        pressure: f32,
    },
    ControllerTouchpadUp {
        timestamp: u32,
        /// The joystick instance id
        which: u32,
        /// The index of the touchpad
        touchpad: u32,
        /// The index of the finger on the touchpad
        finger: u32,
        /// Normalized in the range 0...1 with 0 being on the left
        x: f32,
        /// Normalized in the range 0...1 with 0 being at the top
        y: f32,
        /// Normalized in the range 0...1
        pressure: f32,
    },

    /// Triggered when the gyroscope or accelerometer is updated
    #[cfg(feature = "hidapi")]
    ControllerSensorUpdated {
        timestamp: u32,
        which: u32,
        sensor: crate::sensor::SensorType,
        /// Data from the sensor.
        ///
        /// See the `sensor` module for more information.
        data: [f32; 3],
    },

    FingerDown {
        timestamp: u32,
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32,
    },
    FingerUp {
        timestamp: u32,
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32,
    },
    FingerMotion {
        timestamp: u32,
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32,
    },

    DollarGesture {
        timestamp: u32,
        touch_id: i64,
        gesture_id: i64,
        num_fingers: u32,
        error: f32,
        x: f32,
        y: f32,
    },
    DollarRecord {
        timestamp: u32,
        touch_id: i64,
        gesture_id: i64,
        num_fingers: u32,
        error: f32,
        x: f32,
        y: f32,
    },

    MultiGesture {
        timestamp: u32,
        touch_id: i64,
        d_theta: f32,
        d_dist: f32,
        x: f32,
        y: f32,
        num_fingers: u16,
    },

    ClipboardUpdate {
        timestamp: u32,
    },

    DropFile {
        timestamp: u32,
        window_id: u32,
        filename: String,
    },
    DropText {
        timestamp: u32,
        window_id: u32,
        filename: String,
    },
    DropBegin {
        timestamp: u32,
        window_id: u32,
    },
    DropComplete {
        timestamp: u32,
        window_id: u32,
    },

    AudioDeviceAdded {
        timestamp: u32,
        which: u32,
        iscapture: bool,
    },
    AudioDeviceRemoved {
        timestamp: u32,
        which: u32,
        iscapture: bool,
    },

    RenderTargetsReset {
        timestamp: u32,
    },
    RenderDeviceReset {
        timestamp: u32,
    },

    User {
        timestamp: u32,
        window_id: u32,
        type_: u32,
        code: i32,
        data1: *mut c_void,
        data2: *mut c_void,
    },

    Unknown {
        timestamp: u32,
        type_: u32,
    },
}

/// This does not auto-derive because `User`'s `data` fields can be used to
/// store pointers to types that are `!Send`. Dereferencing these as pointers
/// requires using `unsafe` and ensuring your own safety guarantees.
unsafe impl Send for Event {}

/// This does not auto-derive because `User`'s `data` fields can be used to
/// store pointers to types that are `!Sync`. Dereferencing these as pointers
/// requires using `unsafe` and ensuring your own safety guarantees.
unsafe impl Sync for Event {}

/// Helper function to make converting scancodes
/// and keycodes to primitive `SDL_Keysym` types.
#[doc(alias = "SDL_Keysym")]
fn mk_keysym<S, K>(scancode: S, keycode: K, keymod: Mod) -> sys::SDL_Keysym
where
    S: Into<Option<Scancode>>,
    K: Into<Option<Keycode>>,
{
    let scancode = scancode
        .into()
        .map(|sc| unsafe { transmute::<u32, sys::SDL_Scancode>(sc as u32) })
        .unwrap_or(sys::SDL_Scancode::SDL_SCANCODE_UNKNOWN);
    let keycode = keycode
        .into()
        .map(|kc| kc as sys::SDL_Keycode)
        .unwrap_or(sys::SDL_KeyCode::SDLK_UNKNOWN as i32);
    let keymod = keymod.bits() as u16;
    sys::SDL_Keysym {
        scancode,
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
        let mut ret = mem::MaybeUninit::uninit();
        match *self {
            Event::User {
                window_id,
                type_,
                code,
                data1,
                data2,
                timestamp,
            } => {
                let event = sys::SDL_UserEvent {
                    type_: type_ as u32,
                    timestamp,
                    windowID: window_id,
                    code: code as i32,
                    data1,
                    data2,
                };
                unsafe {
                    ptr::copy(&event, ret.as_mut_ptr() as *mut sys::SDL_UserEvent, 1);
                    Some(ret.assume_init())
                }
            }

            Event::Quit { timestamp } => {
                let event = sys::SDL_QuitEvent {
                    type_: SDL_EventType::SDL_QUIT as u32,
                    timestamp,
                };
                unsafe {
                    ptr::copy(&event, ret.as_mut_ptr() as *mut sys::SDL_QuitEvent, 1);
                    Some(ret.assume_init())
                }
            }

            Event::Display {
                timestamp,
                display_index,
                display_event,
            } => {
                let (display_event_id, data1) = display_event.to_ll();
                let event = sys::SDL_DisplayEvent {
                    type_: SDL_EventType::SDL_DISPLAYEVENT as u32,
                    timestamp,
                    display: display_index as u32,
                    event: display_event_id,
                    padding1: 0,
                    padding2: 0,
                    padding3: 0,
                    data1,
                };
                unsafe {
                    ptr::copy(&event, ret.as_mut_ptr() as *mut sys::SDL_DisplayEvent, 1);
                    Some(ret.assume_init())
                }
            }

            Event::Window {
                timestamp,
                window_id,
                win_event,
            } => {
                let (win_event_id, data1, data2) = win_event.to_ll();
                let event = sys::SDL_WindowEvent {
                    type_: SDL_EventType::SDL_WINDOWEVENT as u32,
                    timestamp,
                    windowID: window_id,
                    event: win_event_id,
                    padding1: 0,
                    padding2: 0,
                    padding3: 0,
                    data1,
                    data2,
                };
                unsafe {
                    ptr::copy(&event, ret.as_mut_ptr() as *mut sys::SDL_WindowEvent, 1);
                    Some(ret.assume_init())
                }
            }

            Event::KeyDown {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
            } => {
                let keysym = mk_keysym(scancode, keycode, keymod);
                let event = sys::SDL_KeyboardEvent {
                    type_: SDL_EventType::SDL_KEYDOWN as u32,
                    timestamp,
                    windowID: window_id,
                    state: sys::SDL_PRESSED as u8,
                    repeat: repeat as u8,
                    padding2: 0,
                    padding3: 0,
                    keysym,
                };
                unsafe {
                    ptr::copy(&event, ret.as_mut_ptr() as *mut sys::SDL_KeyboardEvent, 1);
                    Some(ret.assume_init())
                }
            }
            Event::KeyUp {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
            } => {
                let keysym = mk_keysym(scancode, keycode, keymod);
                let event = sys::SDL_KeyboardEvent {
                    type_: SDL_EventType::SDL_KEYUP as u32,
                    timestamp,
                    windowID: window_id,
                    state: sys::SDL_RELEASED as u8,
                    repeat: repeat as u8,
                    padding2: 0,
                    padding3: 0,
                    keysym,
                };
                unsafe {
                    ptr::copy(&event, ret.as_mut_ptr() as *mut sys::SDL_KeyboardEvent, 1);
                    Some(ret.assume_init())
                }
            }
            Event::MouseMotion {
                timestamp,
                window_id,
                which,
                mousestate,
                x,
                y,
                xrel,
                yrel,
            } => {
                let state = mousestate.to_sdl_state();
                let event = sys::SDL_MouseMotionEvent {
                    type_: SDL_EventType::SDL_MOUSEMOTION as u32,
                    timestamp,
                    windowID: window_id,
                    which,
                    state,
                    x,
                    y,
                    xrel,
                    yrel,
                };
                unsafe {
                    ptr::copy(
                        &event,
                        ret.as_mut_ptr() as *mut sys::SDL_MouseMotionEvent,
                        1,
                    );
                    Some(ret.assume_init())
                }
            }
            Event::MouseButtonDown {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => {
                let event = sys::SDL_MouseButtonEvent {
                    type_: SDL_EventType::SDL_MOUSEBUTTONDOWN as u32,
                    timestamp,
                    windowID: window_id,
                    which,
                    button: mouse_btn as u8,
                    state: sys::SDL_PRESSED as u8,
                    clicks,
                    padding1: 0,
                    x,
                    y,
                };
                unsafe {
                    ptr::copy(
                        &event,
                        ret.as_mut_ptr() as *mut sys::SDL_MouseButtonEvent,
                        1,
                    );
                    Some(ret.assume_init())
                }
            }
            Event::MouseButtonUp {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => {
                let event = sys::SDL_MouseButtonEvent {
                    type_: SDL_EventType::SDL_MOUSEBUTTONUP as u32,
                    timestamp,
                    windowID: window_id,
                    which,
                    button: mouse_btn as u8,
                    state: sys::SDL_RELEASED as u8,
                    clicks,
                    padding1: 0,
                    x,
                    y,
                };
                unsafe {
                    ptr::copy(
                        &event,
                        ret.as_mut_ptr() as *mut sys::SDL_MouseButtonEvent,
                        1,
                    );
                    Some(ret.assume_init())
                }
            }

            Event::MouseWheel {
                timestamp,
                window_id,
                which,
                x,
                y,
                direction,
                precise_x,
                precise_y,
            } => {
                let event = sys::SDL_MouseWheelEvent {
                    type_: SDL_EventType::SDL_MOUSEWHEEL as u32,
                    timestamp,
                    windowID: window_id,
                    which,
                    x,
                    y,
                    direction: direction.to_ll(),
                    preciseX: precise_x,
                    preciseY: precise_y,
                };
                unsafe {
                    ptr::copy(&event, ret.as_mut_ptr() as *mut sys::SDL_MouseWheelEvent, 1);
                    Some(ret.assume_init())
                }
            }
            Event::JoyAxisMotion {
                timestamp,
                which,
                axis_idx,
                value,
            } => {
                let event = sys::SDL_JoyAxisEvent {
                    type_: SDL_EventType::SDL_JOYAXISMOTION as u32,
                    timestamp,
                    which: which as i32,
                    axis: axis_idx,
                    value,
                    padding1: 0,
                    padding2: 0,
                    padding3: 0,
                    padding4: 0,
                };
                unsafe {
                    ptr::copy(&event, ret.as_mut_ptr() as *mut sys::SDL_JoyAxisEvent, 1);
                    Some(ret.assume_init())
                }
            }
            Event::JoyBallMotion {
                timestamp,
                which,
                ball_idx,
                xrel,
                yrel,
            } => {
                let event = sys::SDL_JoyBallEvent {
                    type_: SDL_EventType::SDL_JOYBALLMOTION as u32,
                    timestamp,
                    which: which as i32,
                    ball: ball_idx,
                    xrel,
                    yrel,
                    padding1: 0,
                    padding2: 0,
                    padding3: 0,
                };
                unsafe {
                    ptr::copy(&event, ret.as_mut_ptr() as *mut sys::SDL_JoyBallEvent, 1);
                    Some(ret.assume_init())
                }
            }
            Event::JoyHatMotion {
                timestamp,
                which,
                hat_idx,
                state,
            } => {
                let hatvalue = state.to_raw();
                let event = sys::SDL_JoyHatEvent {
                    type_: SDL_EventType::SDL_JOYHATMOTION as u32,
                    timestamp,
                    which: which as i32,
                    hat: hat_idx,
                    value: hatvalue,
                    padding1: 0,
                    padding2: 0,
                };
                unsafe {
                    ptr::copy(&event, ret.as_mut_ptr() as *mut sys::SDL_JoyHatEvent, 1);
                    Some(ret.assume_init())
                }
            }
            Event::JoyButtonDown {
                timestamp,
                which,
                button_idx,
            } => {
                let event = sys::SDL_JoyButtonEvent {
                    type_: SDL_EventType::SDL_JOYBUTTONDOWN as u32,
                    timestamp,
                    which: which as i32,
                    button: button_idx,
                    state: sys::SDL_PRESSED as u8,
                    padding1: 0,
                    padding2: 0,
                };
                unsafe {
                    ptr::copy(&event, ret.as_mut_ptr() as *mut sys::SDL_JoyButtonEvent, 1);
                    Some(ret.assume_init())
                }
            }

            Event::JoyButtonUp {
                timestamp,
                which,
                button_idx,
            } => {
                let event = sys::SDL_JoyButtonEvent {
                    type_: SDL_EventType::SDL_JOYBUTTONUP as u32,
                    timestamp,
                    which: which as i32,
                    button: button_idx,
                    state: sys::SDL_RELEASED as u8,
                    padding1: 0,
                    padding2: 0,
                };
                unsafe {
                    ptr::copy(&event, ret.as_mut_ptr() as *mut sys::SDL_JoyButtonEvent, 1);
                    Some(ret.assume_init())
                }
            }

            Event::JoyDeviceAdded { timestamp, which } => {
                let event = sys::SDL_JoyDeviceEvent {
                    type_: SDL_EventType::SDL_JOYDEVICEADDED as u32,
                    timestamp,
                    which: which as i32,
                };
                unsafe {
                    ptr::copy(&event, ret.as_mut_ptr() as *mut sys::SDL_JoyDeviceEvent, 1);
                    Some(ret.assume_init())
                }
            }

            Event::JoyDeviceRemoved { timestamp, which } => {
                let event = sys::SDL_JoyDeviceEvent {
                    type_: SDL_EventType::SDL_JOYDEVICEREMOVED as u32,
                    timestamp,
                    which: which as i32,
                };
                unsafe {
                    ptr::copy(&event, ret.as_mut_ptr() as *mut sys::SDL_JoyDeviceEvent, 1);
                    Some(ret.assume_init())
                }
            }
            Event::ControllerAxisMotion {
                timestamp,
                which,
                axis,
                value,
            } => {
                let axisval = axis.to_ll();
                let event = sys::SDL_ControllerAxisEvent {
                    type_: SDL_EventType::SDL_CONTROLLERAXISMOTION as u32,
                    timestamp,
                    which: which as i32,
                    axis: axisval as u8,
                    value,
                    padding1: 0,
                    padding2: 0,
                    padding3: 0,
                    padding4: 0,
                };
                unsafe {
                    ptr::copy(
                        &event,
                        ret.as_mut_ptr() as *mut sys::SDL_ControllerAxisEvent,
                        1,
                    );
                    Some(ret.assume_init())
                }
            }
            Event::ControllerButtonDown {
                timestamp,
                which,
                button,
            } => {
                let buttonval = button.to_ll();
                let event = sys::SDL_ControllerButtonEvent {
                    type_: SDL_EventType::SDL_CONTROLLERBUTTONDOWN as u32,
                    timestamp,
                    which: which as i32,
                    // This conversion turns an i32 into a u8; signed-to-unsigned conversions
                    // are a bit of a code smell, but that appears to be how SDL defines it.
                    button: buttonval as u8,
                    state: sys::SDL_PRESSED as u8,
                    padding1: 0,
                    padding2: 0,
                };
                unsafe {
                    ptr::copy(
                        &event,
                        ret.as_mut_ptr() as *mut sys::SDL_ControllerButtonEvent,
                        1,
                    );
                    Some(ret.assume_init())
                }
            }

            Event::ControllerButtonUp {
                timestamp,
                which,
                button,
            } => {
                let buttonval = button.to_ll();
                let event = sys::SDL_ControllerButtonEvent {
                    type_: SDL_EventType::SDL_CONTROLLERBUTTONUP as u32,
                    timestamp,
                    which: which as i32,
                    button: buttonval as u8,
                    state: sys::SDL_RELEASED as u8,
                    padding1: 0,
                    padding2: 0,
                };
                unsafe {
                    ptr::copy(
                        &event,
                        ret.as_mut_ptr() as *mut sys::SDL_ControllerButtonEvent,
                        1,
                    );
                    Some(ret.assume_init())
                }
            }

            Event::ControllerDeviceAdded { timestamp, which } => {
                let event = sys::SDL_ControllerDeviceEvent {
                    type_: SDL_EventType::SDL_CONTROLLERDEVICEADDED as u32,
                    timestamp,
                    which: which as i32,
                };
                unsafe {
                    ptr::copy(
                        &event,
                        ret.as_mut_ptr() as *mut sys::SDL_ControllerDeviceEvent,
                        1,
                    );
                    Some(ret.assume_init())
                }
            }

            Event::ControllerDeviceRemoved { timestamp, which } => {
                let event = sys::SDL_ControllerDeviceEvent {
                    type_: SDL_EventType::SDL_CONTROLLERDEVICEREMOVED as u32,
                    timestamp,
                    which: which as i32,
                };
                unsafe {
                    ptr::copy(
                        &event,
                        ret.as_mut_ptr() as *mut sys::SDL_ControllerDeviceEvent,
                        1,
                    );
                    Some(ret.assume_init())
                }
            }

            Event::ControllerDeviceRemapped { timestamp, which } => {
                let event = sys::SDL_ControllerDeviceEvent {
                    type_: SDL_EventType::SDL_CONTROLLERDEVICEREMAPPED as u32,
                    timestamp,
                    which: which as i32,
                };
                unsafe {
                    ptr::copy(
                        &event,
                        ret.as_mut_ptr() as *mut sys::SDL_ControllerDeviceEvent,
                        1,
                    );
                    Some(ret.assume_init())
                }
            }

            Event::FingerDown { .. }
            | Event::FingerUp { .. }
            | Event::FingerMotion { .. }
            | Event::DollarGesture { .. }
            | Event::DollarRecord { .. }
            | Event::MultiGesture { .. }
            | Event::ClipboardUpdate { .. }
            | Event::DropFile { .. }
            | Event::TextEditing { .. }
            | Event::TextInput { .. }
            | Event::Unknown { .. }
            | _ => {
                // don't know how to convert!
                None
            }
        }
    }

    pub fn from_ll(raw: sys::SDL_Event) -> Event {
        let raw_type = unsafe { raw.type_ };

        // if event type has not been defined, treat it as a UserEvent
        let event_type: EventType = EventType::try_from(raw_type as u32).unwrap_or(EventType::User);
        unsafe {
            match event_type {
                EventType::Quit => {
                    let event = raw.quit;
                    Event::Quit {
                        timestamp: event.timestamp,
                    }
                }
                EventType::AppTerminating => {
                    let event = raw.common;
                    Event::AppTerminating {
                        timestamp: event.timestamp,
                    }
                }
                EventType::AppLowMemory => {
                    let event = raw.common;
                    Event::AppLowMemory {
                        timestamp: event.timestamp,
                    }
                }
                EventType::AppWillEnterBackground => {
                    let event = raw.common;
                    Event::AppWillEnterBackground {
                        timestamp: event.timestamp,
                    }
                }
                EventType::AppDidEnterBackground => {
                    let event = raw.common;
                    Event::AppDidEnterBackground {
                        timestamp: event.timestamp,
                    }
                }
                EventType::AppWillEnterForeground => {
                    let event = raw.common;
                    Event::AppWillEnterForeground {
                        timestamp: event.timestamp,
                    }
                }
                EventType::AppDidEnterForeground => {
                    let event = raw.common;
                    Event::AppDidEnterForeground {
                        timestamp: event.timestamp,
                    }
                }

                EventType::Display => {
                    let event = raw.display;

                    Event::Display {
                        timestamp: event.timestamp,
                        display_index: event.display as i32,
                        display_event: DisplayEvent::from_ll(event.event, event.data1),
                    }
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
                        keymod: keyboard::Mod::from_bits_truncate(event.keysym.mod_),
                        repeat: event.repeat != 0,
                    }
                }
                EventType::KeyUp => {
                    let event = raw.key;

                    Event::KeyUp {
                        timestamp: event.timestamp,
                        window_id: event.windowID,
                        keycode: Keycode::from_i32(event.keysym.sym as i32),
                        scancode: Scancode::from_i32(event.keysym.scancode as i32),
                        keymod: keyboard::Mod::from_bits_truncate(event.keysym.mod_),
                        repeat: event.repeat != 0,
                    }
                }
                EventType::TextEditing => {
                    let event = raw.edit;

                    let text = String::from_utf8(
                        event
                            .text
                            .iter()
                            .take_while(|&b| (*b) != 0)
                            .map(|&b| b as u8)
                            .collect::<Vec<u8>>(),
                    )
                    .expect("Invalid TextEditing string");
                    Event::TextEditing {
                        timestamp: event.timestamp,
                        window_id: event.windowID,
                        text,
                        start: event.start,
                        length: event.length,
                    }
                }
                EventType::TextInput => {
                    let event = raw.text;

                    let text = String::from_utf8(
                        event
                            .text
                            .iter()
                            .take_while(|&b| (*b) != 0)
                            .map(|&b| b as u8)
                            .collect::<Vec<u8>>(),
                    )
                    .expect("Invalid TextInput string");
                    Event::TextInput {
                        timestamp: event.timestamp,
                        window_id: event.windowID,
                        text,
                    }
                }

                EventType::MouseMotion => {
                    let event = raw.motion;

                    Event::MouseMotion {
                        timestamp: event.timestamp,
                        window_id: event.windowID,
                        which: event.which as u32,
                        mousestate: mouse::MouseState::from_sdl_state(event.state),
                        x: event.x,
                        y: event.y,
                        xrel: event.xrel,
                        yrel: event.yrel,
                    }
                }
                EventType::MouseButtonDown => {
                    let event = raw.button;

                    Event::MouseButtonDown {
                        timestamp: event.timestamp,
                        window_id: event.windowID,
                        which: event.which as u32,
                        mouse_btn: mouse::MouseButton::from_ll(event.button),
                        clicks: event.clicks,
                        x: event.x,
                        y: event.y,
                    }
                }
                EventType::MouseButtonUp => {
                    let event = raw.button;

                    Event::MouseButtonUp {
                        timestamp: event.timestamp,
                        window_id: event.windowID,
                        which: event.which as u32,
                        mouse_btn: mouse::MouseButton::from_ll(event.button),
                        clicks: event.clicks,
                        x: event.x,
                        y: event.y,
                    }
                }
                EventType::MouseWheel => {
                    let event = raw.wheel;

                    Event::MouseWheel {
                        timestamp: event.timestamp,
                        window_id: event.windowID,
                        which: event.which as u32,
                        x: event.x,
                        y: event.y,
                        direction: mouse::MouseWheelDirection::from_ll(event.direction),
                        precise_x: event.preciseX,
                        precise_y: event.preciseY,
                    }
                }

                EventType::JoyAxisMotion => {
                    let event = raw.jaxis;
                    Event::JoyAxisMotion {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                        axis_idx: event.axis,
                        value: event.value,
                    }
                }
                EventType::JoyBallMotion => {
                    let event = raw.jball;
                    Event::JoyBallMotion {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                        ball_idx: event.ball,
                        xrel: event.xrel,
                        yrel: event.yrel,
                    }
                }
                EventType::JoyHatMotion => {
                    let event = raw.jhat;
                    Event::JoyHatMotion {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                        hat_idx: event.hat,
                        state: joystick::HatState::from_raw(event.value),
                    }
                }
                EventType::JoyButtonDown => {
                    let event = raw.jbutton;
                    Event::JoyButtonDown {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                        button_idx: event.button,
                    }
                }
                EventType::JoyButtonUp => {
                    let event = raw.jbutton;
                    Event::JoyButtonUp {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                        button_idx: event.button,
                    }
                }
                EventType::JoyDeviceAdded => {
                    let event = raw.jdevice;
                    Event::JoyDeviceAdded {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                    }
                }
                EventType::JoyDeviceRemoved => {
                    let event = raw.jdevice;
                    Event::JoyDeviceRemoved {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                    }
                }

                EventType::ControllerAxisMotion => {
                    let event = raw.caxis;
                    let axis = controller::Axis::from_ll(transmute(event.axis as i32)).unwrap();

                    Event::ControllerAxisMotion {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                        axis,
                        value: event.value,
                    }
                }
                EventType::ControllerButtonDown => {
                    let event = raw.cbutton;
                    let button =
                        controller::Button::from_ll(transmute(event.button as i32)).unwrap();

                    Event::ControllerButtonDown {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                        button,
                    }
                }
                EventType::ControllerButtonUp => {
                    let event = raw.cbutton;
                    let button =
                        controller::Button::from_ll(transmute(event.button as i32)).unwrap();

                    Event::ControllerButtonUp {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                        button,
                    }
                }
                EventType::ControllerDeviceAdded => {
                    let event = raw.cdevice;
                    Event::ControllerDeviceAdded {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                    }
                }
                EventType::ControllerDeviceRemoved => {
                    let event = raw.cdevice;
                    Event::ControllerDeviceRemoved {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                    }
                }
                EventType::ControllerDeviceRemapped => {
                    let event = raw.cdevice;
                    Event::ControllerDeviceRemapped {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                    }
                }
                EventType::ControllerTouchpadDown => {
                    let event = raw.ctouchpad;
                    Event::ControllerTouchpadDown {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                        touchpad: event.touchpad as u32,
                        finger: event.finger as u32,
                        x: event.x,
                        y: event.y,
                        pressure: event.pressure,
                    }
                }
                EventType::ControllerTouchpadMotion => {
                    let event = raw.ctouchpad;
                    Event::ControllerTouchpadMotion {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                        touchpad: event.touchpad as u32,
                        finger: event.finger as u32,
                        x: event.x,
                        y: event.y,
                        pressure: event.pressure,
                    }
                }
                EventType::ControllerTouchpadUp => {
                    let event = raw.ctouchpad;
                    Event::ControllerTouchpadUp {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                        touchpad: event.touchpad as u32,
                        finger: event.finger as u32,
                        x: event.x,
                        y: event.y,
                        pressure: event.pressure,
                    }
                }
                #[cfg(feature = "hidapi")]
                EventType::ControllerSensorUpdated => {
                    let event = raw.csensor;
                    Event::ControllerSensorUpdated {
                        timestamp: event.timestamp,
                        which: event.which as u32,
                        sensor: crate::sensor::SensorType::from_ll(event.sensor),
                        data: event.data,
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
                        pressure: event.pressure,
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
                        pressure: event.pressure,
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
                        pressure: event.pressure,
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
                        y: event.y,
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
                        y: event.y,
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
                        num_fingers: event.numFingers,
                    }
                }

                EventType::ClipboardUpdate => {
                    let event = raw.common;
                    Event::ClipboardUpdate {
                        timestamp: event.timestamp,
                    }
                }
                EventType::DropFile => {
                    let event = raw.drop;

                    let buf = CStr::from_ptr(event.file as *const _).to_bytes();
                    let text = String::from_utf8_lossy(buf).to_string();
                    sys::SDL_free(event.file as *mut c_void);

                    Event::DropFile {
                        timestamp: event.timestamp,
                        window_id: event.windowID,
                        filename: text,
                    }
                }
                EventType::DropText => {
                    let event = raw.drop;

                    let buf = CStr::from_ptr(event.file as *const _).to_bytes();
                    let text = String::from_utf8_lossy(buf).to_string();
                    sys::SDL_free(event.file as *mut c_void);

                    Event::DropText {
                        timestamp: event.timestamp,
                        window_id: event.windowID,
                        filename: text,
                    }
                }
                EventType::DropBegin => {
                    let event = raw.drop;

                    Event::DropBegin {
                        timestamp: event.timestamp,
                        window_id: event.windowID,
                    }
                }
                EventType::DropComplete => {
                    let event = raw.drop;

                    Event::DropComplete {
                        timestamp: event.timestamp,
                        window_id: event.windowID,
                    }
                }
                EventType::AudioDeviceAdded => {
                    let event = raw.adevice;
                    Event::AudioDeviceAdded {
                        timestamp: event.timestamp,
                        which: event.which,
                        // zero if an audio output device, non-zero if an audio capture device
                        iscapture: event.iscapture != 0,
                    }
                }
                EventType::AudioDeviceRemoved => {
                    let event = raw.adevice;
                    Event::AudioDeviceRemoved {
                        timestamp: event.timestamp,
                        which: event.which,
                        // zero if an audio output device, non-zero if an audio capture device
                        iscapture: event.iscapture != 0,
                    }
                }

                EventType::RenderTargetsReset => Event::RenderTargetsReset {
                    timestamp: raw.common.timestamp,
                },
                EventType::RenderDeviceReset => Event::RenderDeviceReset {
                    timestamp: raw.common.timestamp,
                },

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
                            type_: event.type_,
                        }
                    } else {
                        let event = raw.user;

                        Event::User {
                            timestamp: event.timestamp,
                            window_id: event.windowID,
                            type_: raw_type,
                            code: event.code,
                            data1: event.data1,
                            data2: event.data2,
                        }
                    }
                }
            }
        } // close unsafe & match
    }

    pub fn is_user_event(&self) -> bool {
        match *self {
            Event::User { .. } => true,
            _ => false,
        }
    }

    pub fn as_user_event_type<T: ::std::any::Any>(&self) -> Option<T> {
        use std::any::TypeId;
        let type_id = TypeId::of::<Box<T>>();

        let (event_id, event_box_ptr) = match *self {
            Event::User { type_, data1, .. } => (type_, data1),
            _ => return None,
        };

        let cet = CUSTOM_EVENT_TYPES.lock().unwrap();

        let event_type_id = match cet.sdl_id_to_type_id.get(&event_id) {
            Some(id) => id,
            None => {
                panic!("internal error; could not find typeid")
            }
        };

        if &type_id != event_type_id {
            return None;
        }

        let event_box: Box<T> = unsafe { Box::from_raw(event_box_ptr as *mut T) };

        Some(*event_box)
    }

    /// Returns `true` if they are the same "kind" of events.
    ///
    /// # Example:
    ///
    /// ```
    /// use sdl2::event::Event;
    ///
    /// let ev1 = Event::JoyButtonDown {
    ///     timestamp: 0,
    ///     which: 0,
    ///     button_idx: 0,
    /// };
    /// let ev2 = Event::JoyButtonDown {
    ///     timestamp: 1,
    ///     which: 1,
    ///     button_idx: 1,
    /// };
    ///
    /// assert!(ev1 != ev2); // The events aren't equal (they contain different values).
    /// assert!(ev1.is_same_kind_as(&ev2)); // But they are of the same kind!
    /// ```
    pub fn is_same_kind_as(&self, other: &Event) -> bool {
        match (self, other) {
            (Self::Quit { .. }, Self::Quit { .. })
            | (Self::AppTerminating { .. }, Self::AppTerminating { .. })
            | (Self::AppLowMemory { .. }, Self::AppLowMemory { .. })
            | (Self::AppWillEnterBackground { .. }, Self::AppWillEnterBackground { .. })
            | (Self::AppDidEnterBackground { .. }, Self::AppDidEnterBackground { .. })
            | (Self::AppWillEnterForeground { .. }, Self::AppWillEnterForeground { .. })
            | (Self::AppDidEnterForeground { .. }, Self::AppDidEnterForeground { .. })
            | (Self::Display { .. }, Self::Display { .. })
            | (Self::Window { .. }, Self::Window { .. })
            | (Self::KeyDown { .. }, Self::KeyDown { .. })
            | (Self::KeyUp { .. }, Self::KeyUp { .. })
            | (Self::TextEditing { .. }, Self::TextEditing { .. })
            | (Self::TextInput { .. }, Self::TextInput { .. })
            | (Self::MouseMotion { .. }, Self::MouseMotion { .. })
            | (Self::MouseButtonDown { .. }, Self::MouseButtonDown { .. })
            | (Self::MouseButtonUp { .. }, Self::MouseButtonUp { .. })
            | (Self::MouseWheel { .. }, Self::MouseWheel { .. })
            | (Self::JoyAxisMotion { .. }, Self::JoyAxisMotion { .. })
            | (Self::JoyBallMotion { .. }, Self::JoyBallMotion { .. })
            | (Self::JoyHatMotion { .. }, Self::JoyHatMotion { .. })
            | (Self::JoyButtonDown { .. }, Self::JoyButtonDown { .. })
            | (Self::JoyButtonUp { .. }, Self::JoyButtonUp { .. })
            | (Self::JoyDeviceAdded { .. }, Self::JoyDeviceAdded { .. })
            | (Self::JoyDeviceRemoved { .. }, Self::JoyDeviceRemoved { .. })
            | (Self::ControllerAxisMotion { .. }, Self::ControllerAxisMotion { .. })
            | (Self::ControllerButtonDown { .. }, Self::ControllerButtonDown { .. })
            | (Self::ControllerButtonUp { .. }, Self::ControllerButtonUp { .. })
            | (Self::ControllerDeviceAdded { .. }, Self::ControllerDeviceAdded { .. })
            | (Self::ControllerDeviceRemoved { .. }, Self::ControllerDeviceRemoved { .. })
            | (Self::ControllerDeviceRemapped { .. }, Self::ControllerDeviceRemapped { .. })
            | (Self::FingerDown { .. }, Self::FingerDown { .. })
            | (Self::FingerUp { .. }, Self::FingerUp { .. })
            | (Self::FingerMotion { .. }, Self::FingerMotion { .. })
            | (Self::DollarGesture { .. }, Self::DollarGesture { .. })
            | (Self::DollarRecord { .. }, Self::DollarRecord { .. })
            | (Self::MultiGesture { .. }, Self::MultiGesture { .. })
            | (Self::ClipboardUpdate { .. }, Self::ClipboardUpdate { .. })
            | (Self::DropFile { .. }, Self::DropFile { .. })
            | (Self::DropText { .. }, Self::DropText { .. })
            | (Self::DropBegin { .. }, Self::DropBegin { .. })
            | (Self::DropComplete { .. }, Self::DropComplete { .. })
            | (Self::AudioDeviceAdded { .. }, Self::AudioDeviceAdded { .. })
            | (Self::AudioDeviceRemoved { .. }, Self::AudioDeviceRemoved { .. })
            | (Self::RenderTargetsReset { .. }, Self::RenderTargetsReset { .. })
            | (Self::RenderDeviceReset { .. }, Self::RenderDeviceReset { .. })
            | (Self::User { .. }, Self::User { .. })
            | (Self::Unknown { .. }, Self::Unknown { .. }) => true,
            #[cfg(feature = "hidapi")]
            (Self::ControllerSensorUpdated { .. }, Self::ControllerSensorUpdated { .. }) => true,
            _ => false,
        }
    }

    /// Returns the `timestamp` field of the event.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::event::Event;
    ///
    /// let ev = Event::JoyButtonDown {
    ///     timestamp: 12,
    ///     which: 0,
    ///     button_idx: 0,
    /// };
    /// assert!(ev.get_timestamp() == 12);
    /// ```
    pub fn get_timestamp(&self) -> u32 {
        *match self {
            Self::Quit { timestamp, .. } => timestamp,
            Self::AppTerminating { timestamp, .. } => timestamp,
            Self::AppLowMemory { timestamp, .. } => timestamp,
            Self::AppWillEnterBackground { timestamp, .. } => timestamp,
            Self::AppDidEnterBackground { timestamp, .. } => timestamp,
            Self::AppWillEnterForeground { timestamp, .. } => timestamp,
            Self::AppDidEnterForeground { timestamp, .. } => timestamp,
            Self::Display { timestamp, .. } => timestamp,
            Self::Window { timestamp, .. } => timestamp,
            Self::KeyDown { timestamp, .. } => timestamp,
            Self::KeyUp { timestamp, .. } => timestamp,
            Self::TextEditing { timestamp, .. } => timestamp,
            Self::TextInput { timestamp, .. } => timestamp,
            Self::MouseMotion { timestamp, .. } => timestamp,
            Self::MouseButtonDown { timestamp, .. } => timestamp,
            Self::MouseButtonUp { timestamp, .. } => timestamp,
            Self::MouseWheel { timestamp, .. } => timestamp,
            Self::JoyAxisMotion { timestamp, .. } => timestamp,
            Self::JoyBallMotion { timestamp, .. } => timestamp,
            Self::JoyHatMotion { timestamp, .. } => timestamp,
            Self::JoyButtonDown { timestamp, .. } => timestamp,
            Self::JoyButtonUp { timestamp, .. } => timestamp,
            Self::JoyDeviceAdded { timestamp, .. } => timestamp,
            Self::JoyDeviceRemoved { timestamp, .. } => timestamp,
            Self::ControllerAxisMotion { timestamp, .. } => timestamp,
            Self::ControllerButtonDown { timestamp, .. } => timestamp,
            Self::ControllerButtonUp { timestamp, .. } => timestamp,
            Self::ControllerDeviceAdded { timestamp, .. } => timestamp,
            Self::ControllerDeviceRemoved { timestamp, .. } => timestamp,
            Self::ControllerDeviceRemapped { timestamp, .. } => timestamp,
            Self::ControllerTouchpadDown { timestamp, .. } => timestamp,
            Self::ControllerTouchpadMotion { timestamp, .. } => timestamp,
            Self::ControllerTouchpadUp { timestamp, .. } => timestamp,
            #[cfg(feature = "hidapi")]
            Self::ControllerSensorUpdated { timestamp, .. } => timestamp,
            Self::FingerDown { timestamp, .. } => timestamp,
            Self::FingerUp { timestamp, .. } => timestamp,
            Self::FingerMotion { timestamp, .. } => timestamp,
            Self::DollarGesture { timestamp, .. } => timestamp,
            Self::DollarRecord { timestamp, .. } => timestamp,
            Self::MultiGesture { timestamp, .. } => timestamp,
            Self::ClipboardUpdate { timestamp, .. } => timestamp,
            Self::DropFile { timestamp, .. } => timestamp,
            Self::DropText { timestamp, .. } => timestamp,
            Self::DropBegin { timestamp, .. } => timestamp,
            Self::DropComplete { timestamp, .. } => timestamp,
            Self::AudioDeviceAdded { timestamp, .. } => timestamp,
            Self::AudioDeviceRemoved { timestamp, .. } => timestamp,
            Self::RenderTargetsReset { timestamp, .. } => timestamp,
            Self::RenderDeviceReset { timestamp, .. } => timestamp,
            Self::User { timestamp, .. } => timestamp,
            Self::Unknown { timestamp, .. } => timestamp,
        }
    }

    /// Returns the `window_id` field of the event if it's present (not all events have it!).
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::event::Event;
    ///
    /// let ev = Event::JoyButtonDown {
    ///     timestamp: 0,
    ///     which: 0,
    ///     button_idx: 0,
    /// };
    /// assert!(ev.get_window_id() == None);
    ///
    /// let another_ev = Event::DropBegin {
    ///     timestamp: 0,
    ///     window_id: 3,
    /// };
    /// assert!(another_ev.get_window_id() == Some(3));
    /// ```
    pub fn get_window_id(&self) -> Option<u32> {
        match self {
            Self::Window { window_id, .. } => Some(*window_id),
            Self::KeyDown { window_id, .. } => Some(*window_id),
            Self::KeyUp { window_id, .. } => Some(*window_id),
            Self::TextEditing { window_id, .. } => Some(*window_id),
            Self::TextInput { window_id, .. } => Some(*window_id),
            Self::MouseMotion { window_id, .. } => Some(*window_id),
            Self::MouseButtonDown { window_id, .. } => Some(*window_id),
            Self::MouseButtonUp { window_id, .. } => Some(*window_id),
            Self::MouseWheel { window_id, .. } => Some(*window_id),
            Self::DropFile { window_id, .. } => Some(*window_id),
            Self::DropText { window_id, .. } => Some(*window_id),
            Self::DropBegin { window_id, .. } => Some(*window_id),
            Self::DropComplete { window_id, .. } => Some(*window_id),
            Self::User { window_id, .. } => Some(*window_id),
            _ => None,
        }
    }

    /// Returns `true` if this is a window event.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::event::Event;
    ///
    /// let ev = Event::Quit {
    ///     timestamp: 0,
    /// };
    /// assert!(ev.is_window());
    ///
    /// let ev = Event::AppLowMemory {
    ///     timestamp: 0,
    /// };
    /// assert!(ev.is_window());
    ///
    /// let another_ev = Event::TextInput {
    ///     timestamp: 0,
    ///     window_id: 0,
    ///     text: String::new(),
    /// };
    /// assert!(another_ev.is_window() == false); // Not a window event!
    /// ```
    pub fn is_window(&self) -> bool {
        match self {
            Self::Quit { .. }
            | Self::AppTerminating { .. }
            | Self::AppLowMemory { .. }
            | Self::AppWillEnterBackground { .. }
            | Self::AppDidEnterBackground { .. }
            | Self::AppWillEnterForeground { .. }
            | Self::AppDidEnterForeground { .. }
            | Self::Window { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` if this is a keyboard event.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::event::Event;
    /// use sdl2::keyboard::Mod;
    ///
    /// let ev = Event::KeyDown {
    ///     timestamp: 0,
    ///     window_id: 0,
    ///     keycode: None,
    ///     scancode: None,
    ///     keymod: Mod::empty(),
    ///     repeat: false,
    /// };
    /// assert!(ev.is_keyboard());
    ///
    /// let another_ev = Event::Quit {
    ///     timestamp: 0,
    /// };
    /// assert!(another_ev.is_keyboard() == false); // Not a keyboard event!
    /// ```
    pub fn is_keyboard(&self) -> bool {
        match self {
            Self::KeyDown { .. } | Self::KeyUp { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` if this is a text event.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::event::Event;
    ///
    /// let ev = Event::TextInput {
    ///     timestamp: 0,
    ///     window_id: 0,
    ///     text: String::new(),
    /// };
    /// assert!(ev.is_text());
    ///
    /// let another_ev = Event::Quit {
    ///     timestamp: 0,
    /// };
    /// assert!(another_ev.is_text() == false); // Not a text event!
    /// ```
    pub fn is_text(&self) -> bool {
        match self {
            Self::TextEditing { .. } | Self::TextInput { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` if this is a mouse event.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::event::Event;
    /// use sdl2::mouse::MouseWheelDirection;
    ///
    /// let ev = Event::MouseWheel {
    ///     timestamp: 0,
    ///     window_id: 0,
    ///     which: 0,
    ///     precise_x: 0.0,
    ///     precise_y: 0.0,
    ///     x: 0,
    ///     y: 0,
    ///     direction: MouseWheelDirection::Normal,
    /// };
    /// assert!(ev.is_mouse());
    ///
    /// let another_ev = Event::Quit {
    ///     timestamp: 0,
    /// };
    /// assert!(another_ev.is_mouse() == false); // Not a mouse event!
    /// ```
    pub fn is_mouse(&self) -> bool {
        match self {
            Self::MouseMotion { .. }
            | Self::MouseButtonDown { .. }
            | Self::MouseButtonUp { .. }
            | Self::MouseWheel { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` if this is a controller event.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::event::Event;
    ///
    /// let ev = Event::ControllerDeviceAdded {
    ///     timestamp: 0,
    ///     which: 0,
    /// };
    /// assert!(ev.is_controller());
    ///
    /// let another_ev = Event::Quit {
    ///     timestamp: 0,
    /// };
    /// assert!(another_ev.is_controller() == false); // Not a controller event!
    /// ```
    pub fn is_controller(&self) -> bool {
        match self {
            Self::ControllerAxisMotion { .. }
            | Self::ControllerButtonDown { .. }
            | Self::ControllerButtonUp { .. }
            | Self::ControllerDeviceAdded { .. }
            | Self::ControllerDeviceRemoved { .. }
            | Self::ControllerDeviceRemapped { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` if this is a joy event.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::event::Event;
    ///
    /// let ev = Event::JoyButtonUp {
    ///     timestamp: 0,
    ///     which: 0,
    ///     button_idx: 0,
    /// };
    /// assert!(ev.is_joy());
    ///
    /// let another_ev = Event::Quit {
    ///     timestamp: 0,
    /// };
    /// assert!(another_ev.is_joy() == false); // Not a joy event!
    /// ```
    pub fn is_joy(&self) -> bool {
        match self {
            Self::JoyAxisMotion { .. }
            | Self::JoyBallMotion { .. }
            | Self::JoyHatMotion { .. }
            | Self::JoyButtonDown { .. }
            | Self::JoyButtonUp { .. }
            | Self::JoyDeviceAdded { .. }
            | Self::JoyDeviceRemoved { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` if this is a finger event.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::event::Event;
    ///
    /// let ev = Event::FingerMotion {
    ///     timestamp: 0,
    ///     touch_id: 0,
    ///     finger_id: 0,
    ///     x: 0.,
    ///     y: 0.,
    ///     dx: 0.,
    ///     dy: 0.,
    ///     pressure: 0.,
    /// };
    /// assert!(ev.is_finger());
    ///
    /// let another_ev = Event::Quit {
    ///     timestamp: 0,
    /// };
    /// assert!(another_ev.is_finger() == false); // Not a finger event!
    /// ```
    pub fn is_finger(&self) -> bool {
        match self {
            Self::FingerDown { .. } | Self::FingerUp { .. } | Self::FingerMotion { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` if this is a dollar event.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::event::Event;
    ///
    /// let ev = Event::DollarGesture {
    ///     timestamp: 0,
    ///     touch_id: 0,
    ///     gesture_id: 0,
    ///     num_fingers: 0,
    ///     error: 0.,
    ///     x: 0.,
    ///     y: 0.,
    /// };
    /// assert!(ev.is_dollar());
    ///
    /// let another_ev = Event::Quit {
    ///     timestamp: 0,
    /// };
    /// assert!(another_ev.is_dollar() == false); // Not a dollar event!
    /// ```
    pub fn is_dollar(&self) -> bool {
        match self {
            Self::DollarGesture { .. } | Self::DollarRecord { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` if this is a drop event.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::event::Event;
    ///
    /// let ev = Event::DropBegin {
    ///     timestamp: 0,
    ///     window_id: 3,
    /// };
    /// assert!(ev.is_drop());
    ///
    /// let another_ev = Event::Quit {
    ///     timestamp: 0,
    /// };
    /// assert!(another_ev.is_drop() == false); // Not a drop event!
    /// ```
    pub fn is_drop(&self) -> bool {
        match self {
            Self::DropFile { .. }
            | Self::DropText { .. }
            | Self::DropBegin { .. }
            | Self::DropComplete { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` if this is an audio event.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::event::Event;
    ///
    /// let ev = Event::AudioDeviceAdded {
    ///     timestamp: 0,
    ///     which: 3,
    ///     iscapture: false,
    /// };
    /// assert!(ev.is_audio());
    ///
    /// let another_ev = Event::Quit {
    ///     timestamp: 0,
    /// };
    /// assert!(another_ev.is_audio() == false); // Not an audio event!
    /// ```
    pub fn is_audio(&self) -> bool {
        match self {
            Self::AudioDeviceAdded { .. } | Self::AudioDeviceRemoved { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` if this is a render event.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::event::Event;
    ///
    /// let ev = Event::RenderTargetsReset {
    ///     timestamp: 0,
    /// };
    /// assert!(ev.is_render());
    ///
    /// let another_ev = Event::Quit {
    ///     timestamp: 0,
    /// };
    /// assert!(another_ev.is_render() == false); // Not a render event!
    /// ```
    pub fn is_render(&self) -> bool {
        match self {
            Self::RenderTargetsReset { .. } | Self::RenderDeviceReset { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` if this is a user event.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::event::Event;
    ///
    /// let ev = Event::User {
    ///     timestamp: 0,
    ///     window_id: 0,
    ///     type_: 0,
    ///     code: 0,
    ///     data1: ::std::ptr::null_mut(),
    ///     data2: ::std::ptr::null_mut(),
    /// };
    /// assert!(ev.is_user());
    ///
    /// let another_ev = Event::Quit {
    ///     timestamp: 0,
    /// };
    /// assert!(another_ev.is_user() == false); // Not a user event!
    /// ```
    pub fn is_user(&self) -> bool {
        match self {
            Self::User { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` if this is an unknown event.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::event::Event;
    ///
    /// let ev = Event::Unknown {
    ///     timestamp: 0,
    ///     type_: 0,
    /// };
    /// assert!(ev.is_unknown());
    ///
    /// let another_ev = Event::Quit {
    ///     timestamp: 0,
    /// };
    /// assert!(another_ev.is_unknown() == false); // Not an unknown event!
    /// ```
    pub fn is_unknown(&self) -> bool {
        match self {
            Self::Unknown { .. } => true,
            _ => false,
        }
    }
}

unsafe fn poll_event() -> Option<Event> {
    let mut raw = mem::MaybeUninit::uninit();
    let has_pending = sys::SDL_PollEvent(raw.as_mut_ptr()) == 1;

    if has_pending {
        Some(Event::from_ll(raw.assume_init()))
    } else {
        None
    }
}

unsafe fn wait_event() -> Event {
    let mut raw = mem::MaybeUninit::uninit();
    let success = sys::SDL_WaitEvent(raw.as_mut_ptr()) == 1;

    if success {
        Event::from_ll(raw.assume_init())
    } else {
        panic!("{}", get_error())
    }
}

unsafe fn wait_event_timeout(timeout: u32) -> Option<Event> {
    let mut raw = mem::MaybeUninit::uninit();
    let success = sys::SDL_WaitEventTimeout(raw.as_mut_ptr(), timeout as c_int) == 1;

    if success {
        Some(Event::from_ll(raw.assume_init()))
    } else {
        None
    }
}

impl crate::EventPump {
    /// Query if an event type is enabled.
    #[doc(alias = "SDL_EventState")]
    pub fn is_event_enabled(&self, event_type: EventType) -> bool {
        let result = unsafe { sys::SDL_EventState(event_type as u32, sys::SDL_QUERY) };

        result != sys::SDL_DISABLE as u8
    }

    /// Enable an event type. Returns if the event type was enabled before the call.
    #[doc(alias = "SDL_EventState")]
    pub fn enable_event(&mut self, event_type: EventType) -> bool {
        let result = unsafe { sys::SDL_EventState(event_type as u32, sys::SDL_ENABLE as c_int) };

        result != sys::SDL_DISABLE as u8
    }

    /// Disable an event type. Returns if the event type was enabled before the call.
    #[doc(alias = "SDL_EventState")]
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
    ///         Event::KeyDown {..} => { /*...*/ }
    ///         _ => ()
    ///     }
    /// }
    /// ```
    pub fn poll_iter(&mut self) -> EventPollIterator {
        EventPollIterator {
            _marker: PhantomData,
        }
    }

    /// Pumps the event loop, gathering events from the input devices.
    #[doc(alias = "SDL_PumpEvents")]
    pub fn pump_events(&mut self) {
        unsafe {
            sys::SDL_PumpEvents();
        };
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
            _marker: PhantomData,
        }
    }

    /// Returns a waiting iterator that calls `wait_event_timeout()`.
    ///
    /// Note: The iterator will never terminate, unless waiting for an event
    /// exceeds the specified timeout.
    pub fn wait_timeout_iter(&mut self, timeout: u32) -> EventWaitTimeoutIterator {
        EventWaitTimeoutIterator {
            _marker: PhantomData,
            timeout,
        }
    }

    #[inline]
    pub fn keyboard_state(&self) -> crate::keyboard::KeyboardState {
        crate::keyboard::KeyboardState::new(self)
    }

    #[inline]
    pub fn mouse_state(&self) -> crate::mouse::MouseState {
        crate::mouse::MouseState::new(self)
    }

    #[inline]
    pub fn relative_mouse_state(&self) -> crate::mouse::RelativeMouseState {
        crate::mouse::RelativeMouseState::new(self)
    }
}

/// An iterator that calls `EventPump::poll_event()`.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EventPollIterator<'a> {
    _marker: PhantomData<&'a ()>,
}

impl<'a> Iterator for EventPollIterator<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        unsafe { poll_event() }
    }
}

/// An iterator that calls `EventPump::wait_event()`.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EventWaitIterator<'a> {
    _marker: PhantomData<&'a ()>,
}

impl<'a> Iterator for EventWaitIterator<'a> {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        unsafe { Some(wait_event()) }
    }
}

/// An iterator that calls `EventPump::wait_event_timeout()`.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EventWaitTimeoutIterator<'a> {
    _marker: PhantomData<&'a ()>,
    timeout: u32,
}

impl<'a> Iterator for EventWaitTimeoutIterator<'a> {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        unsafe { wait_event_timeout(self.timeout) }
    }
}

#[cfg(test)]
mod test {
    use super::super::controller::{Axis, Button};
    use super::super::joystick::HatState;
    use super::super::keyboard::{Keycode, Mod, Scancode};
    use super::super::mouse::{MouseButton, MouseState, MouseWheelDirection};
    use super::super::video::Orientation;
    use super::DisplayEvent;
    use super::Event;
    use super::WindowEvent;

    // Tests a round-trip conversion from an Event type to
    // the SDL event type and back, to make sure it's sane.
    #[test]
    fn test_to_from_ll() {
        {
            let e = Event::Quit { timestamp: 0 };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::Display {
                timestamp: 0,
                display_index: 1,
                display_event: DisplayEvent::Orientation(Orientation::LandscapeFlipped),
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::Window {
                timestamp: 0,
                window_id: 0,
                win_event: WindowEvent::Resized(1, 2),
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::KeyDown {
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
            let e = Event::KeyUp {
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
            let e = Event::MouseMotion {
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
            let e = Event::MouseButtonDown {
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
            let e = Event::MouseButtonUp {
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
            let e = Event::MouseWheel {
                timestamp: 1,
                window_id: 0,
                which: 32,
                x: 23,
                y: 91,
                direction: MouseWheelDirection::Flipped,
                precise_x: 1.6,
                precise_y: 2.7,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::JoyAxisMotion {
                timestamp: 0,
                which: 1,
                axis_idx: 1,
                value: 12,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::JoyBallMotion {
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
            let e = Event::JoyHatMotion {
                timestamp: 0,
                which: 3,
                hat_idx: 1,
                state: HatState::Left,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::JoyButtonDown {
                timestamp: 0,
                which: 0,
                button_idx: 3,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::JoyButtonUp {
                timestamp: 9876,
                which: 1,
                button_idx: 2,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::JoyDeviceAdded {
                timestamp: 0,
                which: 1,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::JoyDeviceRemoved {
                timestamp: 0,
                which: 2,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::ControllerAxisMotion {
                timestamp: 53,
                which: 0,
                axis: Axis::LeftX,
                value: 3,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::ControllerButtonDown {
                timestamp: 0,
                which: 1,
                button: Button::Guide,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::ControllerButtonUp {
                timestamp: 654214,
                which: 0,
                button: Button::DPadRight,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::ControllerDeviceAdded {
                timestamp: 543,
                which: 3,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::ControllerDeviceRemoved {
                timestamp: 555,
                which: 3,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
        {
            let e = Event::ControllerDeviceRemapped {
                timestamp: 654,
                which: 0,
            };
            let e2 = Event::from_ll(e.clone().to_ll().unwrap());
            assert_eq!(e, e2);
        }
    }

    #[test]
    fn test_from_ll_keymod_keydown_unknown_bits() {
        let mut raw_event = Event::KeyDown {
            timestamp: 0,
            window_id: 1,
            keycode: None,
            scancode: Some(Scancode::Q),
            keymod: Mod::empty(),
            repeat: false,
        }
        .to_ll()
        .unwrap();

        // Simulate SDL setting bits unknown to us, see PR #780
        raw_event.key.keysym.mod_ = 0xffff;

        if let Event::KeyDown { keymod, .. } = Event::from_ll(raw_event) {
            assert_eq!(keymod, Mod::all());
        } else {
            panic!()
        }
    }

    #[test]
    fn test_from_ll_keymod_keyup_unknown_bits() {
        let mut raw_event = Event::KeyUp {
            timestamp: 0,
            window_id: 1,
            keycode: None,
            scancode: Some(Scancode::Q),
            keymod: Mod::empty(),
            repeat: false,
        }
        .to_ll()
        .unwrap();

        // Simulate SDL setting bits unknown to us, see PR #780
        raw_event.key.keysym.mod_ = 0xffff;

        if let Event::KeyUp { keymod, .. } = Event::from_ll(raw_event) {
            assert_eq!(keymod, Mod::all());
        } else {
            panic!()
        }
    }
}

/// A sendible type that can push events to the event queue.
pub struct EventSender {
    _priv: (),
}

impl EventSender {
    /// Pushes an event to the event queue.
    #[doc(alias = "SDL_PushEvent")]
    pub fn push_event(&self, event: Event) -> Result<(), String> {
        match event.to_ll() {
            Some(mut raw_event) => {
                let ok = unsafe { sys::SDL_PushEvent(&mut raw_event) == 1 };
                if ok {
                    Ok(())
                } else {
                    Err(get_error())
                }
            }
            None => Err("Cannot push unsupported event type to the queue".to_owned()),
        }
    }

    /// Push a custom event
    ///
    /// If the event type ``T`` was not registered using
    /// [EventSubsystem::register_custom_event]
    /// (../struct.EventSubsystem.html#method.register_custom_event),
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
    pub fn push_custom_event<T: ::std::any::Any>(&self, event: T) -> Result<(), String> {
        use std::any::TypeId;
        let cet = CUSTOM_EVENT_TYPES.lock().unwrap();
        let type_id = TypeId::of::<Box<T>>();

        let user_event_id = *match cet.type_id_to_sdl_id.get(&type_id) {
            Some(id) => id,
            None => {
                return Err("Type is not registered as a custom event type!".to_owned());
            }
        };

        let event_box = Box::new(event);
        let event = Event::User {
            timestamp: 0,
            window_id: 0,
            type_: user_event_id,
            code: 0,
            data1: Box::into_raw(event_box) as *mut c_void,
            data2: ::std::ptr::null_mut(),
        };
        drop(cet);

        self.push_event(event)?;

        Ok(())
    }
}

/// A callback trait for [`EventSubsystem::add_event_watch`].
pub trait EventWatchCallback {
    fn callback(&mut self, event: Event) -> ();
}

/// An handler for the event watch callback.
/// One must bind this struct in a variable as long as you want to keep the callback active.
/// For further information, see [`EventSubsystem::add_event_watch`].
pub struct EventWatch<'a, CB: EventWatchCallback + 'a> {
    activated: bool,
    callback: Box<CB>,
    _phantom: PhantomData<&'a CB>,
}

impl<'a, CB: EventWatchCallback + 'a> EventWatch<'a, CB> {
    fn add(callback: CB) -> EventWatch<'a, CB> {
        let f = Box::new(callback);
        let mut watch = EventWatch {
            activated: false,
            callback: f,
            _phantom: PhantomData,
        };
        watch.activate();
        watch
    }

    /// Activates the event watch.
    /// Does nothing if it is already activated.
    pub fn activate(&mut self) {
        if !self.activated {
            self.activated = true;
            unsafe { sys::SDL_AddEventWatch(self.filter(), self.callback()) };
        }
    }

    /// Deactivates the event watch.
    /// Does nothing if it is already activated.
    pub fn deactivate(&mut self) {
        if self.activated {
            self.activated = false;
            unsafe { sys::SDL_DelEventWatch(self.filter(), self.callback()) };
        }
    }

    /// Returns if the event watch is activated.
    pub fn activated(&self) -> bool {
        self.activated
    }

    /// Set the activation state of the event watch.
    pub fn set_activated(&mut self, activate: bool) {
        if activate {
            self.activate();
        } else {
            self.deactivate();
        }
    }

    fn filter(&self) -> SDL_EventFilter {
        Some(event_callback_marshall::<CB> as _)
    }

    fn callback(&mut self) -> *mut c_void {
        &mut *self.callback as *mut _ as *mut c_void
    }
}

impl<'a, CB: EventWatchCallback + 'a> Drop for EventWatch<'a, CB> {
    fn drop(&mut self) {
        self.deactivate();
    }
}

extern "C" fn event_callback_marshall<CB: EventWatchCallback>(
    user_data: *mut c_void,
    event: *mut sdl2_sys::SDL_Event,
) -> i32 {
    let f: &mut CB = unsafe { &mut *(user_data as *mut _) };
    let event = Event::from_ll(unsafe { *event });
    f.callback(event);
    0
}

impl<F: FnMut(Event) -> ()> EventWatchCallback for F {
    fn callback(&mut self, event: Event) -> () {
        self(event)
    }
}
