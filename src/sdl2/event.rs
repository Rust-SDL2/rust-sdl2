/*!
Event Handling
 */

use std::mem;
use libc::{c_int, c_void, uint32_t};
use std::num::FromPrimitive;
use std::string;
use std::ptr;

use controller;
use controller::{ControllerAxis, ControllerButton};
use joystick;
use joystick::HatState;
use keyboard;
use keyboard::Mod;
use keyboard::ll::SDL_Keymod;
use keycode::KeyCode;
use mouse;
use mouse::{Mouse, MouseState};
use scancode::ScanCode;
use video;
use get_error;
use SdlResult;

#[doc(hidden)]
#[allow(non_camel_case_types, non_snake_case)]
pub mod ll {
    use libc::{c_float, c_int, c_char, c_uint, c_void, int16_t,
               int32_t, uint8_t, uint16_t, uint32_t};
    use gesture::ll::SDL_GestureID;
    use keyboard::ll::SDL_Keysym;
    use touch::ll::SDL_FingerID;
    use touch::ll::SDL_TouchID;

    pub type SDL_bool = c_int;

    // SDL_events.h
    pub type SDL_EventState = uint8_t;
    pub const SDL_DISABLE: SDL_EventState = 0;
    pub const SDL_ENABLE: SDL_EventState = 1;
    pub const SDL_QUERY: SDL_EventState = -1;

    pub type SDL_SysWMmsg = c_void;

    pub type SDL_EventType = c_uint;
    pub const SDL_FIRSTEVENT: SDL_EventType = 0;
    pub const SDL_QUIT: SDL_EventType = 256;
    pub const SDL_APP_TERMINATING: SDL_EventType = 257;
    pub const SDL_APP_LOWMEMORY: SDL_EventType = 258;
    pub const SDL_APP_WILLENTERBACKGROUND: SDL_EventType = 259;
    pub const SDL_APP_DIDENTERBACKGROUND: SDL_EventType = 260;
    pub const SDL_APP_WILLENTERFOREGROUND: SDL_EventType = 261;
    pub const SDL_APP_DIDENTERFOREGROUND: SDL_EventType = 262;
    pub const SDL_WINDOWEVENT: SDL_EventType = 512;
    pub const SDL_SYSWMEVENT: SDL_EventType = 513;
    pub const SDL_KEYDOWN: SDL_EventType = 768;
    pub const SDL_KEYUP: SDL_EventType = 769;
    pub const SDL_TEXTEDITING: SDL_EventType = 770;
    pub const SDL_TEXTINPUT: SDL_EventType = 771;
    pub const SDL_MOUSEMOTION: SDL_EventType = 1024;
    pub const SDL_MOUSEBUTTONDOWN: SDL_EventType = 1025;
    pub const SDL_MOUSEBUTTONUP: SDL_EventType = 1026;
    pub const SDL_MOUSEWHEEL: SDL_EventType = 1027;
    pub const SDL_JOYAXISMOTION: SDL_EventType = 1536;
    pub const SDL_JOYBALLMOTION: SDL_EventType = 1537;
    pub const SDL_JOYHATMOTION: SDL_EventType = 1538;
    pub const SDL_JOYBUTTONDOWN: SDL_EventType = 1539;
    pub const SDL_JOYBUTTONUP: SDL_EventType = 1540;
    pub const SDL_JOYDEVICEADDED: SDL_EventType = 1541;
    pub const SDL_JOYDEVICEREMOVED: SDL_EventType = 1542;
    pub const SDL_CONTROLLERAXISMOTION: SDL_EventType = 1616;
    pub const SDL_CONTROLLERBUTTONDOWN: SDL_EventType = 1617;
    pub const SDL_CONTROLLERBUTTONUP: SDL_EventType = 1618;
    pub const SDL_CONTROLLERDEVICEADDED: SDL_EventType = 1619;
    pub const SDL_CONTROLLERDEVICEREMOVED: SDL_EventType = 1620;
    pub const SDL_CONTROLLERDEVICEREMAPPED: SDL_EventType = 1621;
    pub const SDL_FINGERDOWN: SDL_EventType = 1792;
    pub const SDL_FINGERUP: SDL_EventType = 1793;
    pub const SDL_FINGERMOTION: SDL_EventType = 1794;
    pub const SDL_DOLLARGESTURE: SDL_EventType = 2048;
    pub const SDL_DOLLARRECORD: SDL_EventType = 2049;
    pub const SDL_MULTIGESTURE: SDL_EventType = 2050;
    pub const SDL_CLIPBOARDUPDATE: SDL_EventType = 2304;
    pub const SDL_DROPFILE: SDL_EventType = 4096;
    pub const SDL_USEREVENT: SDL_EventType = 32768;
    pub const SDL_LASTEVENT: SDL_EventType = 65535;

    #[repr(C)]
    pub struct SDL_CommonEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
    }

    #[repr(C)]
    pub struct SDL_WindowEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub windowID: uint32_t,
        pub event: uint8_t,
        pub padding1: uint8_t,
        pub padding2: uint8_t,
        pub padding3: uint8_t,
        pub data1: int32_t,
        pub data2: int32_t,
    }

    #[repr(C)]
    pub struct SDL_KeyboardEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub windowID: uint32_t,
        pub state: uint8_t,
        pub repeat: uint8_t,
        pub padding2: uint8_t,
        pub padding3: uint8_t,
        pub keysym: SDL_Keysym,
    }

    #[repr(C)]
    pub struct SDL_TextEditingEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub windowID: uint32_t,
        pub text: [c_char, ..32u],
        pub start: int32_t,
        pub length: int32_t,
    }

    #[repr(C)]
    pub struct SDL_TextInputEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub windowID: uint32_t,
        pub text: [c_char, ..32u],
    }

    #[repr(C)]
    pub struct SDL_MouseMotionEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub windowID: uint32_t,
        pub which: uint32_t,
        pub state: uint32_t,
        pub x: int32_t,
        pub y: int32_t,
        pub xrel: int32_t,
        pub yrel: int32_t,
    }

    #[repr(C)]
    pub struct SDL_MouseButtonEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub windowID: uint32_t,
        pub which: uint32_t,
        pub button: uint8_t,
        pub state: uint8_t,
        pub padding1: uint8_t,
        pub padding2: uint8_t,
        pub x: int32_t,
        pub y: int32_t,
    }

    #[repr(C)]
    pub struct SDL_MouseWheelEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub windowID: uint32_t,
        pub which: uint32_t,
        pub x: int32_t,
        pub y: int32_t,
    }

    #[repr(C)]
    pub struct SDL_JoyAxisEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub which: int32_t,
        pub axis: uint8_t,
        pub padding1: uint8_t,
        pub padding2: uint8_t,
        pub padding3: uint8_t,
        pub value: int16_t,
        pub padding4: uint16_t,
    }

    #[repr(C)]
    pub struct SDL_JoyBallEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub which: int32_t,
        pub ball: uint8_t,
        pub padding1: uint8_t,
        pub padding2: uint8_t,
        pub padding3: uint8_t,
        pub xrel: int16_t,
        pub yrel: int16_t,
    }

    #[repr(C)]
    pub struct SDL_JoyHatEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub which: int32_t,
        pub hat: uint8_t,
        pub value: uint8_t,
        pub padding1: uint8_t,
        pub padding2: uint8_t,
    }

    #[repr(C)]
    pub struct SDL_JoyButtonEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub which: int32_t,
        pub button: uint8_t,
        pub state: uint8_t,
        pub padding1: uint8_t,
        pub padding2: uint8_t,
    }

    #[repr(C)]
    pub struct SDL_JoyDeviceEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub which: int32_t,
    }

    #[repr(C)]
    pub struct SDL_ControllerAxisEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub which: int32_t,
        pub axis: uint8_t,
        pub padding1: uint8_t,
        pub padding2: uint8_t,
        pub padding3: uint8_t,
        pub value: int16_t,
        pub padding4: uint16_t,
    }

    #[repr(C)]
    pub struct SDL_ControllerButtonEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub which: int32_t,
        pub button: uint8_t,
        pub state: uint8_t,
        pub padding1: uint8_t,
        pub padding2: uint8_t,
    }

    #[repr(C)]
    pub struct SDL_ControllerDeviceEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub which: int32_t,
    }

    #[repr(C)]
    pub struct SDL_TouchFingerEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub touchId: SDL_TouchID,
        pub fingerId: SDL_FingerID,
        pub x: c_float,
        pub y: c_float,
        pub dx: c_float,
        pub dy: c_float,
        pub pressure: c_float,
    }

    #[repr(C)]
    pub struct SDL_MultiGestureEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub touchId: SDL_TouchID,
        pub dTheta: c_float,
        pub dDist: c_float,
        pub x: c_float,
        pub y: c_float,
        pub numFingers: uint16_t,
        pub padding: uint16_t,
    }

    #[repr(C)]
    pub struct SDL_DollarGestureEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub touchId: SDL_TouchID,
        pub gestureId: SDL_GestureID,
        pub numFingers: uint32_t,
        pub error: c_float,
        pub x: c_float,
        pub y: c_float,
    }

    #[repr(C)]
    pub struct SDL_DropEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub file: *const c_char,
    }

    #[repr(C)]
    pub struct SDL_QuitEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
    }

    #[repr(C)]
    pub struct SDL_OSEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
    }

    #[repr(C)]
    pub struct SDL_UserEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub windowID: uint32_t,
        pub code: int32_t,
        pub data1: *const c_void,
        pub data2: *const c_void,
    }

    #[repr(C)]
    pub struct SDL_SysWMEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub msg: *const SDL_SysWMmsg,
    }

    #[repr(C)]
    pub struct SDL_Event {
        pub data: [uint8_t, ..56u],
    }

    impl SDL_Event {
        pub fn _type(&self) -> *const uint32_t {
            self.data.as_ptr() as *const _
        }

        pub fn common(&self) -> *const SDL_CommonEvent {
            self.data.as_ptr() as *const _
        }

        pub fn window(&self) -> *const SDL_WindowEvent {
            self.data.as_ptr() as *const _
        }

        pub fn key(&self) -> *const SDL_KeyboardEvent {
            self.data.as_ptr() as *const _
        }

        pub fn edit(&self) -> *const SDL_TextEditingEvent {
            self.data.as_ptr() as *const _
        }

        pub fn text(&self) -> *const SDL_TextInputEvent {
            self.data.as_ptr() as *const _
        }

        pub fn motion(&self) -> *const SDL_MouseMotionEvent {
            self.data.as_ptr() as *const _
        }

        pub fn button(&self) -> *const SDL_MouseButtonEvent {
            self.data.as_ptr() as *const _
        }

        pub fn wheel(&self) -> *const SDL_MouseWheelEvent {
            self.data.as_ptr() as *const _
        }

        pub fn jaxis(&self) -> *const SDL_JoyAxisEvent {
            self.data.as_ptr() as *const _
        }

        pub fn jball(&self) -> *const SDL_JoyBallEvent {
            self.data.as_ptr() as *const _
        }

        pub fn jhat(&self) -> *const SDL_JoyHatEvent {
            self.data.as_ptr() as *const _
        }

        pub fn jbutton(&self) -> *const SDL_JoyButtonEvent {
            self.data.as_ptr() as *const _
        }

        pub fn jdevice(&self) -> *const SDL_JoyDeviceEvent {
            self.data.as_ptr() as *const _
        }

        pub fn caxis(&self) -> *const SDL_ControllerAxisEvent {
            self.data.as_ptr() as *const _
        }

        pub fn cbutton(&self) -> *const SDL_ControllerButtonEvent {
            self.data.as_ptr() as *const _
        }

        pub fn cdevice(&self) -> *const SDL_ControllerDeviceEvent {
            self.data.as_ptr() as *const _
        }

        pub fn quit(&self) -> *const SDL_QuitEvent {
            self.data.as_ptr() as *const _
        }

        pub fn user(&self) -> *const SDL_UserEvent {
            self.data.as_ptr() as *const _
        }

        pub fn syswm(&self) -> *const SDL_SysWMEvent {
            self.data.as_ptr() as *const _
        }

        pub fn tfinger(&self) -> *const SDL_TouchFingerEvent {
            self.data.as_ptr() as *const _
        }

        pub fn mgesture(&self) -> *const SDL_MultiGestureEvent {
            self.data.as_ptr() as *const _
        }

        pub fn dgesture(&self) -> *const SDL_DollarGestureEvent {
            self.data.as_ptr() as *const _
        }

        pub fn drop(&self) -> *const SDL_DropEvent {
            self.data.as_ptr() as *const _
        }
    }

    pub type SDL_eventaction = c_uint;
    pub const SDL_ADDEVENT: SDL_eventaction = 0;
    pub const SDL_PEEKEVENT: SDL_eventaction = 1;
    pub const SDL_GETEVENT: SDL_eventaction = 2;
    pub type SDL_EventFilter =
        extern "C" fn(userdata: *const c_void, event: *const SDL_Event) -> c_int;

    extern "C" {
        pub fn SDL_free(mem: *const c_void);
        pub fn SDL_PumpEvents();
        /*pub fn SDL_PeepEvents(events: &[SDL_Event], numevents: c_int,
                                    action: SDL_eventaction, minType: uint32_t,
                                    maxType: uint32_t) -> c_int;*/
        pub fn SDL_HasEvent(_type: uint32_t) -> SDL_bool;
        pub fn SDL_HasEvents(minType: uint32_t, maxType: uint32_t) ->
                  SDL_bool;
        pub fn SDL_FlushEvent(_type: uint32_t);
        pub fn SDL_FlushEvents(minType: uint32_t, maxType: uint32_t);
        pub fn SDL_PollEvent(event: *const SDL_Event) -> c_int;
        pub fn SDL_WaitEvent(event: *const SDL_Event) -> c_int;
        pub fn SDL_WaitEventTimeout(event: *const SDL_Event, timeout: c_int) ->
                  c_int;
        pub fn SDL_PushEvent(event: *const SDL_Event) -> c_int;
        pub fn SDL_SetEventFilter(filter: SDL_EventFilter,
                                        userdata: *const c_void);
        /*pub fn SDL_GetEventFilter(filter: *SDL_EventFilter,
                                        userdata: **c_void) -> SDL_bool;*/
        pub fn SDL_AddEventWatch(filter: SDL_EventFilter, userdata: *const c_void);
        pub fn SDL_DelEventWatch(filter: SDL_EventFilter, userdata: *const c_void);
        pub fn SDL_FilterEvents(filter: SDL_EventFilter, userdata: *const c_void);
        pub fn SDL_EventState(_type: uint32_t, state: SDL_EventState) -> SDL_EventState;
        pub fn SDL_RegisterEvents(numevents: c_int) -> uint32_t;
    }
}

/// Types of events that can be delivered.
#[deriving(FromPrimitive)]
#[repr(u32)]
pub enum EventType {
    First = ll::SDL_FIRSTEVENT,

    Quit = ll::SDL_QUIT,
    AppTerminating = ll::SDL_APP_TERMINATING,
    AppLowMemory = ll::SDL_APP_LOWMEMORY,
    AppWillEnterBackground = ll::SDL_APP_WILLENTERBACKGROUND,
    AppDidEnterBackground = ll::SDL_APP_DIDENTERBACKGROUND,
    AppWillEnterForeground = ll::SDL_APP_WILLENTERFOREGROUND,
    AppDidEnterForeground = ll::SDL_APP_DIDENTERFOREGROUND,

    Window = ll::SDL_WINDOWEVENT,
    // TODO: SysWM = ll::SDL_SYSWMEVENT,

    KeyDown = ll::SDL_KEYDOWN,
    KeyUp = ll::SDL_KEYUP,
    TextEditing = ll::SDL_TEXTEDITING,
    TextInput = ll::SDL_TEXTINPUT,

    MouseMotion = ll::SDL_MOUSEMOTION,
    MouseButtonDown = ll::SDL_MOUSEBUTTONDOWN,
    MouseButtonUp = ll::SDL_MOUSEBUTTONUP,
    MouseWheel = ll::SDL_MOUSEWHEEL,

    JoyAxisMotion = ll::SDL_JOYAXISMOTION,
    JoyBallMotion = ll::SDL_JOYBALLMOTION,
    JoyHatMotion = ll::SDL_JOYHATMOTION,
    JoyButtonDown = ll::SDL_JOYBUTTONDOWN,
    JoyButtonUp = ll::SDL_JOYBUTTONUP,
    JoyDeviceAdded = ll::SDL_JOYDEVICEADDED,
    JoyDeviceRemoved = ll::SDL_JOYDEVICEREMOVED,

    ControllerAxisMotion = ll::SDL_CONTROLLERAXISMOTION,
    ControllerButtonDown = ll::SDL_CONTROLLERBUTTONDOWN,
    ControllerButtonUp = ll::SDL_CONTROLLERBUTTONUP,
    ControllerDeviceAdded = ll::SDL_CONTROLLERDEVICEADDED,
    ControllerDeviceRemoved = ll::SDL_CONTROLLERDEVICEREMOVED,
    ControllerDeviceRemapped = ll::SDL_CONTROLLERDEVICEREMAPPED,

    FingerDown = ll::SDL_FINGERDOWN,
    FingerUp = ll::SDL_FINGERUP,
    FingerMotion = ll::SDL_FINGERMOTION,
    DollarGesture = ll::SDL_DOLLARGESTURE,
    DollarRecord = ll::SDL_DOLLARRECORD,
    MultiGesture = ll::SDL_MULTIGESTURE,

    ClipboardUpdate = ll::SDL_CLIPBOARDUPDATE,
    DropFile = ll::SDL_DROPFILE,

    User = ll::SDL_USEREVENT,
    Last = ll::SDL_LASTEVENT,
}

#[deriving(Show)]
/// An enum of window events.
pub enum WindowEventId {
    None,
    Shown,
    Hidden,
    Exposed,
    Moved,
    Resized,
    SizeChanged,
    Minimized,
    Maximized,
    Restored,
    Enter,
    Leave,
    FocusGained,
    FocusLost,
    Close,
}

impl WindowEventId {
    fn from_ll(id: u8) -> WindowEventId {
        match id {
            1  => WindowEventId::Shown,
            2  => WindowEventId::Hidden,
            3  => WindowEventId::Exposed,
            4  => WindowEventId::Moved,
            5  => WindowEventId::Resized,
            6  => WindowEventId::SizeChanged,
            7  => WindowEventId::Minimized,
            8  => WindowEventId::Maximized,
            9  => WindowEventId::Restored,
            10 => WindowEventId::Enter,
            11 => WindowEventId::Leave,
            12 => WindowEventId::FocusGained,
            13 => WindowEventId::FocusLost,
            14 => WindowEventId::Close,
            _  => WindowEventId::None
        }
    }
}

/// Different event types.
pub enum Event {
    NoEvent,

    /// (timestamp)
    QuitEvent(uint),
    AppTerminatingEvent(uint),
    AppLowMemoryEvent(uint),
    AppWillEnterBackgroundEvent(uint),
    AppDidEnterBackgroundEvent(uint),
    AppWillEnterForegroundEvent(uint),
    AppDidEnterForegroundEvent(uint),

    /// (timestamp, window, winEventId, data1, data2)
    WindowEvent(uint, video::Window, WindowEventId, int, int),
    // TODO: SysWMEvent

    /// (timestamp, window, keycode, scancode, keymod)
    KeyDownEvent(uint, video::Window, KeyCode, ScanCode, Mod),
    KeyUpEvent(uint, video::Window, KeyCode, ScanCode, Mod),
    /// (timestamp, window, text, start, length)
    TextEditingEvent(uint, video::Window, String, int, int),
    /// (timestamp, window, text)
    TextInputEvent(uint, video::Window, String),

    /// (timestamp, window, which, [MouseState], x, y, xrel, yrel)
    MouseMotionEvent(uint, video::Window, uint, MouseState, int, int,
                     int, int),
    /// (timestamp, window, which, MouseBtn, x, y)
    MouseButtonDownEvent(uint, video::Window, uint, Mouse, int, int),
    MouseButtonUpEvent(uint, video::Window, uint, Mouse, int, int),
    /// (timestamp, window, whichId, x, y)
    MouseWheelEvent(uint, video::Window, uint, int, int),

    /// (timestamp, whichId, axisIdx, value)
    JoyAxisMotionEvent(uint, int, int, i16),
    /// (timestamp, whichId, ballIdx, xrel, yrel)
    JoyBallMotionEvent(uint, int, int, i16, i16),
    /// (timestamp, whichId, hatIdx, state)
    JoyHatMotionEvent(uint, int, int, HatState),
    /// (timestamp, whichId, buttonIdx)
    JoyButtonDownEvent(uint, int, int),
    JoyButtonUpEvent(uint, int, int),
    /// (timestamp, whichId)
    JoyDeviceAddedEvent(uint, int),
    JoyDeviceRemovedEvent(uint, int),

    /// (timestamp, whichId, axis, value)
    ControllerAxisMotionEvent(uint, int, ControllerAxis, i16),
    /// (timestamp, whichId, button)
    ControllerButtonDownEvent(uint, int, ControllerButton),
    ControllerButtonUpEvent(uint, int, ControllerButton),
    /// (timestamp, whichIdx)
    ControllerDeviceAddedEvent(uint, int),
    ControllerDeviceRemovedEvent(uint, int),
    ControllerDeviceRemappedEvent(uint, int),

    /// (timestamp, touchId, fingerId, x, y, dx, dy, pressure)
    FingerDownEvent(uint, i64, i64, f64, f64, f64, f64, f64),
    FingerUpEvent(uint, i64, i64, f64, f64, f64, f64, f64),
    FingerMotionEvent(uint, i64, i64, f64, f64, f64, f64, f64),

    /// (timestamp, touchId, gestureId, numFingers, error, x, y)
    DollarGestureEvent(uint, i64, i64, uint, f64, f64, f64),
    DollarRecordEvent(uint, i64, i64, uint, f64, f64, f64),
    /// (timestamp, touchId, dTheta, dDist, x, y, numFingers)
    MultiGestureEvent(uint, i64, f64, f64, f64, f64, uint),

    /// (timestamp)
    ClipboardUpdateEvent(uint),

    /// (timestamp, filename)
    DropFileEvent(uint, String),

    /// (timestamp, Window, type, code)
    UserEvent(uint, video::Window, uint, int),
}

impl ::std::fmt::Show for Event {
    fn fmt(&self, out: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        out.write(match *self {
            Event::NoEvent => "NoEvent",
            Event::QuitEvent(..) => "QuitEvent",
            Event::AppTerminatingEvent(..) => "AppTerminatingEvent",
            Event::AppLowMemoryEvent(..) => "AppLowMemoryEvent",
            Event::AppWillEnterBackgroundEvent(..) => "AppWillEnterBackgroundEvent",
            Event::AppDidEnterBackgroundEvent(..) => "AppDidEnterBackgroundEvent",
            Event::AppWillEnterForegroundEvent(..) => "AppWillEnterForegroundEvent",
            Event::AppDidEnterForegroundEvent(..) => "AppDidEnterForegroundEvent",
            Event::WindowEvent(..) => "WindowEvent",
            Event::KeyDownEvent(..) => "KeyDownEvent",
            Event::KeyUpEvent(..) => "KeyUpEvent",
            Event::TextEditingEvent(..) => "TextEditingEvent",
            Event::TextInputEvent(..) => "TextInputEvent",
            Event::MouseMotionEvent(..) => "MouseMotionEvent",
            Event::MouseButtonDownEvent(..) => "MouseButtonDownEvent",
            Event::MouseButtonUpEvent(..) => "MouseButtonUpEvent",
            Event::MouseWheelEvent(..) => "MouseWheelEvent",
            Event::JoyAxisMotionEvent(..) => "JoyAxisMotionEvent",
            Event::JoyBallMotionEvent(..) => "JoyBallMotionEvent",
            Event::JoyHatMotionEvent(..) => "JoyHatMotionEvent",
            Event::JoyButtonDownEvent(..) => "JoyButtonDownEvent",
            Event::JoyButtonUpEvent(..) => "JoyButtonUpEvent",
            Event::JoyDeviceAddedEvent(..) => "JoyDeviceAddedEvent",
            Event::JoyDeviceRemovedEvent(..) => "JoyDeviceRemovedEvent",
            Event::ControllerAxisMotionEvent(..) => "ControllerAxisMotionEvent",
            Event::ControllerButtonDownEvent(..) => "ControllerButtonDownEvent",
            Event::ControllerButtonUpEvent(..) => "ControllerButtonUpEvent",
            Event::ControllerDeviceAddedEvent(..) => "ControllerDeviceAddedEvent",
            Event::ControllerDeviceRemovedEvent(..) => "ControllerDeviceRemovedEvent",
            Event::ControllerDeviceRemappedEvent(..) => "ControllerDeviceRemappedEvent",
            Event::FingerDownEvent(..) => "FingerDownEvent",
            Event::FingerUpEvent(..) => "FingerUpEvent",
            Event::FingerMotionEvent(..) => "FingerMotionEvent",
            Event::DollarGestureEvent(..) => "DollarGestureEvent",
            Event::DollarRecordEvent(..) => "DollarRecordEvent",
            Event::MultiGestureEvent(..) => "MultiGestureEvent",
            Event::ClipboardUpdateEvent(..) => "ClipboardUpdateEvent",
            Event::DropFileEvent(..) => "DropFileEvent",
            Event::UserEvent(..) => "UserEvent",
        }.as_bytes())
    }
}

// TODO: Remove this when from_utf8 is updated in Rust
impl Event {
    fn to_ll(self) -> Option<ll::SDL_Event> {
        let ret = null_event();
        match self {
            // just ignore timestamp
            Event::UserEvent(_, ref win, typ, code) => {
                let event = ll::SDL_UserEvent {
                    _type: typ as uint32_t,
                    timestamp: 0,
                    windowID: win.get_id(),
                    code: code as i32,
                    data1: ptr::null(),
                    data2: ptr::null(),
                };
                unsafe {
                    ptr::copy_memory(mem::transmute::<_,*mut ll::SDL_UserEvent>(&ret), &event, 1);
                }
                Some(ret)
            },
            _ => {
                // don't know how to convert!
                None
            }
        }
    }

    fn from_ll(raw: &ll::SDL_Event) -> Event {
        let raw_type = raw._type();
        let raw_type = if raw_type.is_null() {
            return Event::NoEvent;
        } else {
            unsafe { *raw_type }
        };

        // if event type has not been defined, treat it as a UserEvent
        let event_type: EventType = FromPrimitive::from_uint(raw_type as uint).unwrap_or(EventType::User);
        unsafe { match event_type {
            EventType::Quit => {
                let event = *raw.quit();
                Event::QuitEvent(event.timestamp as uint)
            }
            EventType::AppTerminating => {
                let event = *raw.common();
                Event::AppTerminatingEvent(event.timestamp as uint)
            }
            EventType::AppLowMemory => {
                let event = *raw.common();
                Event::AppLowMemoryEvent(event.timestamp as uint)
            }
            EventType::AppWillEnterBackground => {
                let event = *raw.common();
                Event::AppWillEnterBackgroundEvent(event.timestamp as uint)
            }
            EventType::AppDidEnterBackground => {
                let event = *raw.common();
                Event::AppDidEnterBackgroundEvent(event.timestamp as uint)
            }
            EventType::AppWillEnterForeground => {
                let event = *raw.common();
                Event::AppWillEnterForegroundEvent(event.timestamp as uint)
            }
            EventType::AppDidEnterForeground => {
                let event = *raw.common();
                Event::AppDidEnterForegroundEvent(event.timestamp as uint)
            }

            EventType::Window => {
                let event = *raw.window();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return Event::NoEvent,
                    Ok(window) => window,
                };

                Event::WindowEvent(event.timestamp as uint, window,
                                   WindowEventId::from_ll(event.event),
                                   event.data1 as int, event.data2 as int)
            }
            // TODO: SysWMEventType

            EventType::KeyDown => {
                let event = *raw.key();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return Event::NoEvent,
                    Ok(window) => window,
                };

                Event::KeyDownEvent(event.timestamp as uint, window,
                                    FromPrimitive::from_int(event.keysym.sym as int)
                                       .unwrap_or(KeyCode::UnknownKey),
                                    FromPrimitive::from_int(event.keysym.scancode as int)
                                       .unwrap_or(ScanCode::UnknownScanCode),
                                    keyboard::Mod::from_bits(event.keysym._mod as SDL_Keymod).unwrap())
            }
            EventType::KeyUp => {
                let event = *raw.key();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return Event::NoEvent,
                    Ok(window) => window,
                };

                Event::KeyUpEvent(event.timestamp as uint, window,
                                  FromPrimitive::from_int(event.keysym.sym as int)
                                       .unwrap_or(KeyCode::UnknownKey),
                                  FromPrimitive::from_int(event.keysym.scancode as int)
                                       .unwrap_or(ScanCode::UnknownScanCode),
                                  keyboard::Mod::from_bits(event.keysym._mod as SDL_Keymod).unwrap())
            }
            EventType::TextEditing => {
                let event = *raw.edit();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return Event::NoEvent,
                    Ok(window) => window,
                };

                let text = String::from_utf8_lossy(event.text.iter().take_while(|&b| (*b) != 0i8).map(|&b| b as u8).collect::<Vec<u8>>().as_slice()).into_string();
                Event::TextEditingEvent(event.timestamp as uint, window, text,
                                        event.start as int, event.length as int)
            }
            EventType::TextInput => {
                let event = *raw.text();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return Event::NoEvent,
                    Ok(window) => window,
                };

                let text = String::from_utf8_lossy(event.text.iter().take_while(|&b| (*b) != 0i8).map(|&b| b as u8).collect::<Vec<u8>>().as_slice()).into_string();
                Event::TextInputEvent(event.timestamp as uint, window, text)
            }

            EventType::MouseMotion => {
                let event = *raw.motion();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return Event::NoEvent,
                    Ok(window) => window,
                };

                Event::MouseMotionEvent(event.timestamp as uint, window,
                                        event.which as uint,
                                        mouse::MouseState::from_bits(event.state).unwrap(),
                                        event.x as int, event.y as int,
                                        event.xrel as int, event.yrel as int)
            }
            EventType::MouseButtonDown => {
                let event = *raw.button();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return Event::NoEvent,
                    Ok(window) => window,
                };

                Event::MouseButtonDownEvent(event.timestamp as uint, window,
                                            event.which as uint,
                                            mouse::wrap_mouse(event.button),
                                            event.x as int, event.y as int)
            }
            EventType::MouseButtonUp => {
                let event = *raw.button();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return Event::NoEvent,
                    Ok(window) => window,
                };

                Event::MouseButtonUpEvent(event.timestamp as uint, window,
                                          event.which as uint,
                                          mouse::wrap_mouse(event.button),
                                          event.x as int, event.y as int)
            }
            EventType::MouseWheel => {
                let event = *raw.wheel();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return Event::NoEvent,
                    Ok(window) => window,
                };

                Event::MouseWheelEvent(event.timestamp as uint, window,
                                       event.which as uint, event.x as int,
                                       event.y as int)
            }

            EventType::JoyAxisMotion => {
                let event = *raw.jaxis();
                Event::JoyAxisMotionEvent(event.timestamp as uint,
                                          event.which as int, event.axis as int,
                                          event.value)
            }
            EventType::JoyBallMotion => {
                let event = *raw.jball();
                Event::JoyBallMotionEvent(event.timestamp as uint,
                                          event.which as int, event.ball as int,
                                          event.xrel, event.yrel)
            }
            EventType::JoyHatMotion => {
                let event = *raw.jhat();
                Event::JoyHatMotionEvent(event.timestamp as uint,
                                         event.which as int, event.hat as int,
                                         joystick::HatState::from_bits(event.value).unwrap())
            }
            EventType::JoyButtonDown => {
                let event = *raw.jbutton();
                Event::JoyButtonDownEvent(event.timestamp as uint,
                                          event.which as int,
                                          event.button as int)
            }
            EventType::JoyButtonUp => {
                let event = *raw.jbutton();
                Event::JoyButtonUpEvent(event.timestamp as uint,
                                        event.which as int,
                                        event.button as int)
            }
            EventType::JoyDeviceAdded => {
                let event = *raw.jdevice();
                Event::JoyDeviceAddedEvent(event.timestamp as uint,
                                           event.which as int)
            }
            EventType::JoyDeviceRemoved => {
                let event = *raw.jdevice();
                Event::JoyDeviceRemovedEvent(event.timestamp as uint,
                                             event.which as int)
            }

            EventType::ControllerAxisMotion => {
                let event = *raw.caxis();
                let axis = controller::wrap_controller_axis(event.axis);

                Event::ControllerAxisMotionEvent(event.timestamp as uint,
                                                 event.which as int,
                                                 axis, event.value)
            }
            EventType::ControllerButtonDown => {
                let event = *raw.cbutton();
                let button = controller::wrap_controller_button(event.button);

                Event::ControllerButtonDownEvent(event.timestamp as uint,
                                                 event.which as int, button)
            }
            EventType::ControllerButtonUp => {
                let event = *raw.cbutton();
                let button = controller::wrap_controller_button(event.button);

                Event::ControllerButtonUpEvent(event.timestamp as uint,
                                               event.which as int, button)
            }
            EventType::ControllerDeviceAdded => {
                let event = *raw.cdevice();
                Event::ControllerDeviceAddedEvent(event.timestamp as uint,
                                                  event.which as int)
            }
            EventType::ControllerDeviceRemoved => {
                let event = *raw.cdevice();
                Event::ControllerDeviceRemovedEvent(event.timestamp as uint,
                                                    event.which as int)
            }
            EventType::ControllerDeviceRemapped => {
                let event = *raw.cdevice();
                Event::ControllerDeviceRemappedEvent(event.timestamp as uint,
                                                     event.which as int)
            }

            EventType::FingerDown => {
                let event = *raw.tfinger();
                Event::FingerDownEvent(event.timestamp as uint,
                                       event.touchId as i64,
                                       event.fingerId as i64,
                                       event.x as f64, event.y as f64,
                                       event.dx as f64, event.dy as f64,
                                       event.pressure as f64)
            }
            EventType::FingerUp => {
                let event = *raw.tfinger();
                Event::FingerUpEvent(event.timestamp as uint,
                                     event.touchId as i64,
                                     event.fingerId as i64, event.x as f64,
                                     event.y as f64, event.dx as f64,
                                     event.dy as f64, event.pressure as f64)
            }
            EventType::FingerMotion => {
                let event = *raw.tfinger();
                Event::FingerMotionEvent(event.timestamp as uint,
                                         event.touchId as i64,
                                         event.fingerId as i64, event.x as f64,
                                         event.y as f64, event.dx as f64,
                                         event.dy as f64, event.pressure as f64)
            }
            EventType::DollarGesture => {
                let event = *raw.dgesture();
                Event::DollarGestureEvent(event.timestamp as uint,
                                          event.touchId as i64,
                                          event.gestureId as i64,
                                          event.numFingers as uint,
                                          event.error as f64, event.x as f64,
                                          event.y as f64)
            }
            EventType::DollarRecord => {
                let event = *raw.dgesture();
                Event::DollarRecordEvent(event.timestamp as uint,
                                         event.touchId as i64,
                                         event.gestureId as i64,
                                         event.numFingers as uint,
                                         event.error as f64, event.x as f64,
                                         event.y as f64)
            }
            EventType::MultiGesture => {
                let event = *raw.mgesture();
                Event::MultiGestureEvent(event.timestamp as uint,
                                         event.touchId as i64,
                                         event.dTheta as f64,
                                         event.dDist as f64, event.x as f64,
                                         event.y as f64,
                                         event.numFingers as uint)
            }

            EventType::ClipboardUpdate => {
                let event = *raw.common();
                Event::ClipboardUpdateEvent(event.timestamp as uint)
            }
            EventType::DropFile => {
                let event = *raw.drop();

                let text = string::raw::from_buf(event.file as *const u8);
                ll::SDL_free(event.file as *const c_void);

                Event::DropFileEvent(event.timestamp as uint, text)
            }

            EventType::First | EventType::Last => Event::NoEvent,

            // If we have no other match and the event type is >= 32768
            // this is a user event
            EventType::User => {
                if raw_type < 32768 {
                    return Event::NoEvent;
                }

                let event = *raw.user();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return Event::NoEvent,
                    Ok(window) => window,
                };

                Event::UserEvent(event.timestamp as uint, window,
                                 raw_type as uint, event.code as int)
            }
        }}                      // close unsafe & match


    }
}

fn null_event() -> ll::SDL_Event {
    ll::SDL_Event { data: [0, ..56] }
}

/// Pump the event loop, gathering events from the input devices.
pub fn pump_events() {
    unsafe { ll::SDL_PumpEvents(); }
}

/// Check for the existence of certain event types in the event queue.
pub fn has_event(_type: EventType) -> bool {
    unsafe { ll::SDL_HasEvent(_type as uint32_t ) == 1 }
}

/// Check for the existence of a range of event types in the event queue.
pub fn has_events(min: EventType, max: EventType) -> bool {
    unsafe { ll::SDL_HasEvents(min as uint32_t, max as uint32_t) == 1 }
}

/// Clear events from the event queue.
pub fn flush_event(_type: EventType) {
    unsafe { ll::SDL_FlushEvent(_type as uint32_t) }
}

/// Clear events from the event queue of a range of event types.
pub fn flush_events(min: EventType, max: EventType) {
    unsafe { ll::SDL_FlushEvents(min as uint32_t, max as uint32_t) }
}

/// Poll for currently pending events.
pub fn poll_event() -> Event {
    pump_events();

    let raw = null_event();
    let success = unsafe { ll::SDL_PollEvent(&raw) == 1 as c_int };

    if success { Event::from_ll(&raw) }
    else { Event::NoEvent }
}

/// Wait indefinitely for the next available event.
pub fn wait_event() -> SdlResult<Event> {
    let raw = null_event();
    let success = unsafe { ll::SDL_WaitEvent(&raw) == 1 as c_int };

    if success { Ok(Event::from_ll(&raw)) }
    else { Err(get_error()) }
}

/// Wait until the specified timeout (in milliseconds) for the next available event.
pub fn wait_event_timeout(timeout: int) -> SdlResult<Event> {
    let raw = null_event();
    let success = unsafe { ll::SDL_WaitEventTimeout(&raw, timeout as c_int) ==
                           1 as c_int };

    if success { Ok(Event::from_ll(&raw)) }
    else { Err(get_error()) }
}

extern "C" fn event_filter_wrapper(userdata: *const c_void, event: *const ll::SDL_Event) -> c_int {
    let filter: extern fn(event: Event) -> bool = unsafe { mem::transmute(userdata) };
    if event.is_null() { 1 }
    else { filter(Event::from_ll(unsafe { &*event })) as c_int }
}

/// Set up a filter to process all events before they change internal state and are posted to the internal event queue.
pub fn set_event_filter(filter_func: extern fn(event: Event) -> bool) {
    unsafe { ll::SDL_SetEventFilter(event_filter_wrapper,
                                    filter_func as *const _) }
}

/// Add a callback to be triggered when an event is added to the event queue.
pub fn add_event_watch(filter_func: extern fn(event: Event) -> bool) {
    unsafe { ll::SDL_AddEventWatch(event_filter_wrapper,
                                   filter_func as *const _) }
}

/// Remove an event watch callback added.
pub fn delete_event_watch(filter_func: extern fn(event: Event) -> bool) {
    unsafe { ll::SDL_DelEventWatch(event_filter_wrapper,
                                   filter_func as *const _) }
}

/// Run a specific filter function on the current event queue, removing any events for which the filter returns 0.
pub fn filter_events(filter_func: extern fn(event: Event) -> bool) {
    unsafe { ll::SDL_FilterEvents(event_filter_wrapper,
                                  filter_func as *const _) }
}

/// Set the state of processing events.
pub fn set_event_state(_type: EventType, state: bool) {
    unsafe { ll::SDL_EventState(_type as uint32_t,
                                state as ll::SDL_EventState); }
}

/// Get the state of processing events.
pub fn get_event_state(_type: EventType) -> bool {
    unsafe { ll::SDL_EventState(_type as uint32_t, ll::SDL_QUERY)
             == ll::SDL_ENABLE }
}

/// allocate a set of user-defined events, and return the beginning event number for that set of events
pub fn register_events(num: int) -> Option<uint> {
    let ret = unsafe { ll::SDL_RegisterEvents(num as c_int) };
    if ret == (-1 as uint32_t) {
        None
    } else {
        Some(ret as uint)
    }
}

/// add an event to the event queue
pub fn push_event(event: Event) -> SdlResult<()> {
    match event.to_ll() {
        Some(raw_event) => {
            let ok = unsafe { ll::SDL_PushEvent(&raw_event) == 1 };
            if ok { Ok(()) }
            else { Err(get_error()) }
        },
        None => {
            Err("Unsupport event type to push back to queue.".into_string())
        }
    }
}
