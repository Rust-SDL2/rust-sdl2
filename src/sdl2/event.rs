/*!
Event Handling
 */

use std::mem;
use libc::{c_int, c_void, uint32_t};
use std::num::FromPrimitive;
use std::str;
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

#[doc(hidden)]
#[allow(non_camel_case_types)]
pub mod ll {
    use std::mem;
    use libc::{c_float, c_int, c_char, c_uint, c_void, int16_t,
               int32_t, uint8_t, uint16_t, uint32_t};
    use gesture::ll::SDL_GestureID;
    use keyboard::ll::SDL_Keysym;
    use touch::ll::SDL_FingerID;
    use touch::ll::SDL_TouchID;

    pub type SDL_bool = c_int;

    // SDL_events.h
    pub type SDL_EventState = uint8_t;
    pub static SDL_DISABLE: SDL_EventState = 0;
    pub static SDL_ENABLE: SDL_EventState = 1;
    pub static SDL_QUERY: SDL_EventState = -1;

    pub type SDL_SysWMmsg = c_void;

    pub type SDL_EventType = c_uint;
    pub static SDL_FIRSTEVENT: SDL_EventType = 0;
    pub static SDL_QUIT: SDL_EventType = 256;
    pub static SDL_APP_TERMINATING: SDL_EventType = 257;
    pub static SDL_APP_LOWMEMORY: SDL_EventType = 258;
    pub static SDL_APP_WILLENTERBACKGROUND: SDL_EventType = 259;
    pub static SDL_APP_DIDENTERBACKGROUND: SDL_EventType = 260;
    pub static SDL_APP_WILLENTERFOREGROUND: SDL_EventType = 261;
    pub static SDL_APP_DIDENTERFOREGROUND: SDL_EventType = 262;
    pub static SDL_WINDOWEVENT: SDL_EventType = 512;
    pub static SDL_SYSWMEVENT: SDL_EventType = 513;
    pub static SDL_KEYDOWN: SDL_EventType = 768;
    pub static SDL_KEYUP: SDL_EventType = 769;
    pub static SDL_TEXTEDITING: SDL_EventType = 770;
    pub static SDL_TEXTINPUT: SDL_EventType = 771;
    pub static SDL_MOUSEMOTION: SDL_EventType = 1024;
    pub static SDL_MOUSEBUTTONDOWN: SDL_EventType = 1025;
    pub static SDL_MOUSEBUTTONUP: SDL_EventType = 1026;
    pub static SDL_MOUSEWHEEL: SDL_EventType = 1027;
    pub static SDL_JOYAXISMOTION: SDL_EventType = 1536;
    pub static SDL_JOYBALLMOTION: SDL_EventType = 1537;
    pub static SDL_JOYHATMOTION: SDL_EventType = 1538;
    pub static SDL_JOYBUTTONDOWN: SDL_EventType = 1539;
    pub static SDL_JOYBUTTONUP: SDL_EventType = 1540;
    pub static SDL_JOYDEVICEADDED: SDL_EventType = 1541;
    pub static SDL_JOYDEVICEREMOVED: SDL_EventType = 1542;
    pub static SDL_CONTROLLERAXISMOTION: SDL_EventType = 1616;
    pub static SDL_CONTROLLERBUTTONDOWN: SDL_EventType = 1617;
    pub static SDL_CONTROLLERBUTTONUP: SDL_EventType = 1618;
    pub static SDL_CONTROLLERDEVICEADDED: SDL_EventType = 1619;
    pub static SDL_CONTROLLERDEVICEREMOVED: SDL_EventType = 1620;
    pub static SDL_CONTROLLERDEVICEREMAPPED: SDL_EventType = 1621;
    pub static SDL_FINGERDOWN: SDL_EventType = 1792;
    pub static SDL_FINGERUP: SDL_EventType = 1793;
    pub static SDL_FINGERMOTION: SDL_EventType = 1794;
    pub static SDL_DOLLARGESTURE: SDL_EventType = 2048;
    pub static SDL_DOLLARRECORD: SDL_EventType = 2049;
    pub static SDL_MULTIGESTURE: SDL_EventType = 2050;
    pub static SDL_CLIPBOARDUPDATE: SDL_EventType = 2304;
    pub static SDL_DROPFILE: SDL_EventType = 4096;
    pub static SDL_USEREVENT: SDL_EventType = 32768;
    pub static SDL_LASTEVENT: SDL_EventType = 65535;

    pub struct SDL_CommonEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
    }

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

    pub struct SDL_TextEditingEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub windowID: uint32_t,
        pub text: [c_char, ..32u],
        pub start: int32_t,
        pub length: int32_t,
    }

    pub struct SDL_TextInputEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub windowID: uint32_t,
        pub text: [c_char, ..32u],
    }

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

    pub struct SDL_MouseWheelEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub windowID: uint32_t,
        pub which: uint32_t,
        pub x: int32_t,
        pub y: int32_t,
    }

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

    pub struct SDL_JoyHatEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub which: int32_t,
        pub hat: uint8_t,
        pub value: uint8_t,
        pub padding1: uint8_t,
        pub padding2: uint8_t,
    }

    pub struct SDL_JoyButtonEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub which: int32_t,
        pub button: uint8_t,
        pub state: uint8_t,
        pub padding1: uint8_t,
        pub padding2: uint8_t,
    }

    pub struct SDL_JoyDeviceEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub which: int32_t,
    }

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

    pub struct SDL_ControllerButtonEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub which: int32_t,
        pub button: uint8_t,
        pub state: uint8_t,
        pub padding1: uint8_t,
        pub padding2: uint8_t,
    }

    pub struct SDL_ControllerDeviceEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub which: int32_t,
    }

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

    pub struct SDL_DropEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub file: *c_char,
    }

    pub struct SDL_QuitEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
    }

    pub struct SDL_OSEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
    }

    pub struct SDL_UserEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub windowID: uint32_t,
        pub code: int32_t,
        pub data1: *c_void,
        pub data2: *c_void,
    }

    pub struct SDL_SysWMEvent {
        pub _type: uint32_t,
        pub timestamp: uint32_t,
        pub msg: *SDL_SysWMmsg,
    }

    pub struct SDL_Event {
        pub data: [uint8_t, ..56u],
    }

    impl SDL_Event {
        pub fn _type(&self) -> *uint32_t {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn common(&self) -> *SDL_CommonEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn window(&self) -> *SDL_WindowEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn key(&self) -> *SDL_KeyboardEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn edit(&self) -> *SDL_TextEditingEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn text(&self) -> *SDL_TextInputEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn motion(&self) -> *SDL_MouseMotionEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn button(&self) -> *SDL_MouseButtonEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn wheel(&self) -> *SDL_MouseWheelEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn jaxis(&self) -> *SDL_JoyAxisEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn jball(&self) -> *SDL_JoyBallEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn jhat(&self) -> *SDL_JoyHatEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn jbutton(&self) -> *SDL_JoyButtonEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn jdevice(&self) -> *SDL_JoyDeviceEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn caxis(&self) -> *SDL_ControllerAxisEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn cbutton(&self) -> *SDL_ControllerButtonEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn cdevice(&self) -> *SDL_ControllerDeviceEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn quit(&self) -> *SDL_QuitEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn user(&self) -> *SDL_UserEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn syswm(&self) -> *SDL_SysWMEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn tfinger(&self) -> *SDL_TouchFingerEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn mgesture(&self) -> *SDL_MultiGestureEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn dgesture(&self) -> *SDL_DollarGestureEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn drop(&self) -> *SDL_DropEvent {
            unsafe { mem::transmute_copy(&self) }
        }
    }

    pub type SDL_eventaction = c_uint;
    pub static SDL_ADDEVENT: SDL_eventaction = 0;
    pub static SDL_PEEKEVENT: SDL_eventaction = 1;
    pub static SDL_GETEVENT: SDL_eventaction = 2;
    pub type SDL_EventFilter =
        extern "C" fn(userdata: *c_void, event: *SDL_Event) -> c_int;

    extern "C" {
        pub fn SDL_free(mem: *c_void);
        pub fn SDL_PumpEvents();
        /*pub fn SDL_PeepEvents(events: &[SDL_Event], numevents: c_int,
                                    action: SDL_eventaction, minType: uint32_t,
                                    maxType: uint32_t) -> c_int;*/
        pub fn SDL_HasEvent(_type: uint32_t) -> SDL_bool;
        pub fn SDL_HasEvents(minType: uint32_t, maxType: uint32_t) ->
                  SDL_bool;
        pub fn SDL_FlushEvent(_type: uint32_t);
        pub fn SDL_FlushEvents(minType: uint32_t, maxType: uint32_t);
        pub fn SDL_PollEvent(event: *SDL_Event) -> c_int;
        pub fn SDL_WaitEvent(event: *SDL_Event) -> c_int;
        pub fn SDL_WaitEventTimeout(event: *SDL_Event, timeout: c_int) ->
                  c_int;
        pub fn SDL_PushEvent(event: *SDL_Event) -> c_int;
        pub fn SDL_SetEventFilter(filter: SDL_EventFilter,
                                        userdata: *c_void);
        /*pub fn SDL_GetEventFilter(filter: *SDL_EventFilter,
                                        userdata: **c_void) -> SDL_bool;*/
        pub fn SDL_AddEventWatch(filter: SDL_EventFilter, userdata: *c_void);
        pub fn SDL_DelEventWatch(filter: SDL_EventFilter, userdata: *c_void);
        pub fn SDL_FilterEvents(filter: SDL_EventFilter, userdata: *c_void);
        pub fn SDL_EventState(_type: uint32_t, state: SDL_EventState) -> SDL_EventState;
        pub fn SDL_RegisterEvents(numevents: c_int) -> uint32_t;
    }
}

/// Types of events that can be delivered.
#[deriving(FromPrimitive)]
pub enum EventType {
    FirstEventType = ll::SDL_FIRSTEVENT,

    QuitEventType = ll::SDL_QUIT,
    AppTerminatingEventType = ll::SDL_APP_TERMINATING,
    AppLowMemoryEventType = ll::SDL_APP_LOWMEMORY,
    AppWillEnterBackgroundEventType = ll::SDL_APP_WILLENTERBACKGROUND,
    AppDidEnterBackgroundEventType = ll::SDL_APP_DIDENTERBACKGROUND,
    AppWillEnterForegroundEventType = ll::SDL_APP_WILLENTERFOREGROUND,
    AppDidEnterForegroundEventType = ll::SDL_APP_DIDENTERFOREGROUND,

    WindowEventType = ll::SDL_WINDOWEVENT,
    // TODO: SysWMEventType = ll::SDL_SYSWMEVENT,

    KeyDownEventType = ll::SDL_KEYDOWN,
    KeyUpEventType = ll::SDL_KEYUP,
    TextEditingEventType = ll::SDL_TEXTEDITING,
    TextInputEventType = ll::SDL_TEXTINPUT,

    MouseMotionEventType = ll::SDL_MOUSEMOTION,
    MouseButtonDownEventType = ll::SDL_MOUSEBUTTONDOWN,
    MouseButtonUpEventType = ll::SDL_MOUSEBUTTONUP,
    MouseWheelEventType = ll::SDL_MOUSEWHEEL,

    JoyAxisMotionEventType = ll::SDL_JOYAXISMOTION,
    JoyBallMotionEventType = ll::SDL_JOYBALLMOTION,
    JoyHatMotionEventType = ll::SDL_JOYHATMOTION,
    JoyButtonDownEventType = ll::SDL_JOYBUTTONDOWN,
    JoyButtonUpEventType = ll::SDL_JOYBUTTONUP,
    JoyDeviceAddedEventType = ll::SDL_JOYDEVICEADDED,
    JoyDeviceRemovedEventType = ll::SDL_JOYDEVICEREMOVED,

    ControllerAxisMotionEventType = ll::SDL_CONTROLLERAXISMOTION,
    ControllerButtonDownEventType = ll::SDL_CONTROLLERBUTTONDOWN,
    ControllerButtonUpEventType = ll::SDL_CONTROLLERBUTTONUP,
    ControllerDeviceAddedEventType = ll::SDL_CONTROLLERDEVICEADDED,
    ControllerDeviceRemovedEventType = ll::SDL_CONTROLLERDEVICEREMOVED,
    ControllerDeviceRemappedEventType = ll::SDL_CONTROLLERDEVICEREMAPPED,

    FingerDownEventType = ll::SDL_FINGERDOWN,
    FingerUpEventType = ll::SDL_FINGERUP,
    FingerMotionEventType = ll::SDL_FINGERMOTION,
    DollarGestureEventType = ll::SDL_DOLLARGESTURE,
    DollarRecordEventType = ll::SDL_DOLLARRECORD,
    MultiGestureEventType = ll::SDL_MULTIGESTURE,

    ClipboardUpdateEventType = ll::SDL_CLIPBOARDUPDATE,
    DropFileEventType = ll::SDL_DROPFILE,

    UserEventType = ll::SDL_USEREVENT,
    LastEventType = ll::SDL_LASTEVENT,
}

#[deriving(Show)]
/// An enum of window events.
pub enum WindowEventId {
    NoneWindowEventId,
    ShownWindowEventId,
    HiddenWindowEventId,
    ExposedWindowEventId,
    MovedWindowEventId,
    ResizedWindowEventId,
    SizeChangedWindowEventId,
    MinimizedWindowEventId,
    MaximizedWindowEventId,
    RestoredWindowEventId,
    EnterWindowEventId,
    LeaveWindowEventId,
    FocusGainedWindowEventId,
    FocusLostWindowEventId,
    CloseWindowEventId,
}

impl WindowEventId {
    fn from_ll(id: u8) -> WindowEventId {
        match id {
            1  => ShownWindowEventId,
            2  => HiddenWindowEventId,
            3  => ExposedWindowEventId,
            4  => MovedWindowEventId,
            5  => ResizedWindowEventId,
            6  => SizeChangedWindowEventId,
            7  => MinimizedWindowEventId,
            8  => MaximizedWindowEventId,
            9  => RestoredWindowEventId,
            10 => EnterWindowEventId,
            11 => LeaveWindowEventId,
            12 => FocusGainedWindowEventId,
            13 => FocusLostWindowEventId,
            14 => CloseWindowEventId,
            _  => NoneWindowEventId
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
    TextEditingEvent(uint, video::Window, ~str, int, int),
    /// (timestamp, window, text)
    TextInputEvent(uint, video::Window, ~str),

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
    DropFileEvent(uint, ~str),

    /// (timestamp, Window, type, code)
    UserEvent(uint, video::Window, uint, int),
}

impl ::std::fmt::Show for Event {
    fn fmt(&self, out: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        out.write(match *self {
            NoEvent => "NoEvent",
            QuitEvent(..) => "QuitEvent",
            AppTerminatingEvent(..) => "AppTerminatingEvent",
            AppLowMemoryEvent(..) => "AppLowMemoryEvent",
            AppWillEnterBackgroundEvent(..) => "AppWillEnterBackgroundEvent",
            AppDidEnterBackgroundEvent(..) => "AppDidEnterBackgroundEvent",
            AppWillEnterForegroundEvent(..) => "AppWillEnterForegroundEvent",
            AppDidEnterForegroundEvent(..) => "AppDidEnterForegroundEvent",
            WindowEvent(..) => "WindowEvent",
            KeyDownEvent(..) => "KeyDownEvent",
            KeyUpEvent(..) => "KeyUpEvent",
            TextEditingEvent(..) => "TextEditingEvent",
            TextInputEvent(..) => "TextInputEvent",
            MouseMotionEvent(..) => "MouseMotionEvent",
            MouseButtonDownEvent(..) => "MouseButtonDownEvent",
            MouseButtonUpEvent(..) => "MouseButtonUpEvent",
            MouseWheelEvent(..) => "MouseWheelEvent",
            JoyAxisMotionEvent(..) => "JoyAxisMotionEvent",
            JoyBallMotionEvent(..) => "JoyBallMotionEvent",
            JoyHatMotionEvent(..) => "JoyHatMotionEvent",
            JoyButtonDownEvent(..) => "JoyButtonDownEvent",
            JoyButtonUpEvent(..) => "JoyButtonUpEvent",
            JoyDeviceAddedEvent(..) => "JoyDeviceAddedEvent",
            JoyDeviceRemovedEvent(..) => "JoyDeviceRemovedEvent",
            ControllerAxisMotionEvent(..) => "ControllerAxisMotionEvent",
            ControllerButtonDownEvent(..) => "ControllerButtonDownEvent",
            ControllerButtonUpEvent(..) => "ControllerButtonUpEvent",
            ControllerDeviceAddedEvent(..) => "ControllerDeviceAddedEvent",
            ControllerDeviceRemovedEvent(..) => "ControllerDeviceRemovedEvent",
            ControllerDeviceRemappedEvent(..) => "ControllerDeviceRemappedEvent",
            FingerDownEvent(..) => "FingerDownEvent",
            FingerUpEvent(..) => "FingerUpEvent",
            FingerMotionEvent(..) => "FingerMotionEvent",
            DollarGestureEvent(..) => "DollarGestureEvent",
            DollarRecordEvent(..) => "DollarRecordEvent",
            MultiGestureEvent(..) => "MultiGestureEvent",
            ClipboardUpdateEvent(..) => "ClipboardUpdateEvent",
            DropFileEvent(..) => "DropFileEvent",
            UserEvent(..) => "UserEvent",
        }.as_bytes())
    }
}

// TODO: Remove this when from_utf8 is updated in Rust
impl Event {
    fn to_ll(self) -> Option<ll::SDL_Event> {
        let ret = null_event();
        match self {
            // just ignore timestamp
            UserEvent(_, ref win, typ, code) => {
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
            return NoEvent;
        } else {
            unsafe { *raw_type }
        };

        // if event type has not been defined, treat it as a UserEvent
        let event_type: EventType = FromPrimitive::from_uint(raw_type as uint).unwrap_or(UserEventType);
        unsafe { match event_type {
            QuitEventType => {
                let event = *raw.quit();
                QuitEvent(event.timestamp as uint)
            }
            AppTerminatingEventType => {
                let event = *raw.common();
                AppTerminatingEvent(event.timestamp as uint)
            }
            AppLowMemoryEventType => {
                let event = *raw.common();
                AppLowMemoryEvent(event.timestamp as uint)
            }
            AppWillEnterBackgroundEventType => {
                let event = *raw.common();
                AppWillEnterBackgroundEvent(event.timestamp as uint)
            }
            AppDidEnterBackgroundEventType => {
                let event = *raw.common();
                AppDidEnterBackgroundEvent(event.timestamp as uint)
            }
            AppWillEnterForegroundEventType => {
                let event = *raw.common();
                AppWillEnterForegroundEvent(event.timestamp as uint)
            }
            AppDidEnterForegroundEventType => {
                let event = *raw.common();
                AppDidEnterForegroundEvent(event.timestamp as uint)
            }

            WindowEventType => {
                let event = *raw.window();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                WindowEvent(event.timestamp as uint, window,
                            WindowEventId::from_ll(event.event),
                            event.data1 as int, event.data2 as int)
            }
            // TODO: SysWMEventType

            KeyDownEventType => {
                let event = *raw.key();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                KeyDownEvent(event.timestamp as uint, window,
                             FromPrimitive::from_int(event.keysym.sym as int).unwrap(),
                             FromPrimitive::from_int(event.keysym.scancode as int).unwrap(),
                             keyboard::Mod::from_bits(event.keysym._mod as SDL_Keymod).unwrap())
            }
            KeyUpEventType => {
                let event = *raw.key();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                KeyUpEvent(event.timestamp as uint, window,
                           FromPrimitive::from_int(event.keysym.sym as int).unwrap(),
                           FromPrimitive::from_int(event.keysym.scancode as int).unwrap(),
                           keyboard::Mod::from_bits(event.keysym._mod as SDL_Keymod).unwrap())
            }
            TextEditingEventType => {
                let event = *raw.edit();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                let text = str::from_utf8_lossy(event.text.iter().take_while(|&b| (*b) != 0i8).map(|&b| b as u8).collect::<Vec<u8>>().as_slice()).into_owned();
                TextEditingEvent(event.timestamp as uint, window, text,
                                 event.start as int, event.length as int)
            }
            TextInputEventType => {
                let event = *raw.text();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                let text = str::from_utf8_lossy(event.text.iter().take_while(|&b| (*b) != 0i8).map(|&b| b as u8).collect::<Vec<u8>>().as_slice()).into_owned();
                TextInputEvent(event.timestamp as uint, window, text)
            }

            MouseMotionEventType => {
                let event = *raw.motion();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                MouseMotionEvent(event.timestamp as uint, window,
                                 event.which as uint,
                                 mouse::MouseState::from_bits(event.state).unwrap(),
                                 event.x as int, event.y as int,
                                 event.xrel as int, event.yrel as int)
            }
            MouseButtonDownEventType => {
                let event = *raw.button();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                MouseButtonDownEvent(event.timestamp as uint, window,
                                     event.which as uint,
                                     mouse::wrap_mouse(event.button),
                                     event.x as int, event.y as int)
            }
            MouseButtonUpEventType => {
                let event = *raw.button();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                MouseButtonUpEvent(event.timestamp as uint, window,
                                   event.which as uint,
                                   mouse::wrap_mouse(event.button),
                                   event.x as int, event.y as int)
            }
            MouseWheelEventType => {
                let event = *raw.button();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                MouseWheelEvent(event.timestamp as uint, window,
                                event.which as uint, event.x as int,
                                event.y as int)
            }

            JoyAxisMotionEventType => {
                let event = *raw.jaxis();
                JoyAxisMotionEvent(event.timestamp as uint, event.which as int,
                                   event.axis as int, event.value)
            }
            JoyBallMotionEventType => {
                let event = *raw.jball();
                JoyBallMotionEvent(event.timestamp as uint, event.which as int,
                                   event.ball as int,
                                   event.xrel, event.yrel)
            }
            JoyHatMotionEventType => {
                let event = *raw.jhat();
                JoyHatMotionEvent(event.timestamp as uint, event.which as int,
                                  event.hat as int,
                                  joystick::HatState::from_bits(event.value).unwrap())
            }
            JoyButtonDownEventType => {
                let event = *raw.jbutton();
                JoyButtonDownEvent(event.timestamp as uint, event.which as int,
                                   event.button as int)
            }
            JoyButtonUpEventType => {
                let event = *raw.jbutton();
                JoyButtonUpEvent(event.timestamp as uint, event.which as int,
                                 event.button as int)
            }
            JoyDeviceAddedEventType => {
                let event = *raw.jdevice();
                JoyDeviceAddedEvent(event.timestamp as uint,
                                    event.which as int)
            }
            JoyDeviceRemovedEventType => {
                let event = *raw.jdevice();
                JoyDeviceRemovedEvent(event.timestamp as uint,
                                      event.which as int)
            }

            ControllerAxisMotionEventType => {
                let event = *raw.caxis();
                let axis = controller::wrap_controller_axis(event.axis);

                ControllerAxisMotionEvent(event.timestamp as uint,
                                          event.which as int,
                                          axis, event.value)
            }
            ControllerButtonDownEventType => {
                let event = *raw.cbutton();
                let button = controller::wrap_controller_button(event.button);

                ControllerButtonDownEvent(event.timestamp as uint,
                                          event.which as int, button)
            }
            ControllerButtonUpEventType => {
                let event = *raw.cbutton();
                let button = controller::wrap_controller_button(event.button);

                ControllerButtonUpEvent(event.timestamp as uint,
                                        event.which as int, button)
            }
            ControllerDeviceAddedEventType => {
                let event = *raw.cdevice();
                ControllerDeviceAddedEvent(event.timestamp as uint,
                                           event.which as int)
            }
            ControllerDeviceRemovedEventType => {
                let event = *raw.cdevice();
                ControllerDeviceRemovedEvent(event.timestamp as uint,
                                             event.which as int)
            }
            ControllerDeviceRemappedEventType => {
                let event = *raw.cdevice();
                ControllerDeviceRemappedEvent(event.timestamp as uint,
                                              event.which as int)
            }

            FingerDownEventType => {
                let event = *raw.tfinger();
                FingerDownEvent(event.timestamp as uint, event.touchId as i64,
                                event.fingerId as i64, event.x as f64,
                                event.y as f64, event.dx as f64,
                                event.dy as f64, event.pressure as f64)
            }
            FingerUpEventType => {
                let event = *raw.tfinger();
                FingerUpEvent(event.timestamp as uint, event.touchId as i64,
                              event.fingerId as i64, event.x as f64,
                              event.y as f64, event.dx as f64,
                              event.dy as f64, event.pressure as f64)
            }
            FingerMotionEventType => {
                let event = *raw.tfinger();
                FingerMotionEvent(event.timestamp as uint,
                                  event.touchId as i64, event.fingerId as i64,
                                  event.x as f64, event.y as f64,
                                  event.dx as f64, event.dy as f64,
                                  event.pressure as f64)
            }
            DollarGestureEventType => {
                let event = *raw.dgesture();
                DollarGestureEvent(event.timestamp as uint,
                                   event.touchId as i64,
                                   event.gestureId as i64,
                                   event.numFingers as uint,
                                   event.error as f64, event.x as f64,
                                   event.y as f64)
            }
            DollarRecordEventType => {
                let event = *raw.dgesture();
                DollarRecordEvent(event.timestamp as uint,
                                  event.touchId as i64, event.gestureId as i64,
                                  event.numFingers as uint,
                                  event.error as f64, event.x as f64,
                                  event.y as f64)
            }
            MultiGestureEventType => {
                let event = *raw.mgesture();
                MultiGestureEvent(event.timestamp as uint,
                                  event.touchId as i64, event.dTheta as f64,
                                  event.dDist as f64, event.x as f64,
                                  event.y as f64, event.numFingers as uint)
            }

            ClipboardUpdateEventType => {
                let event = *raw.common();
                ClipboardUpdateEvent(event.timestamp as uint)
            }
            DropFileEventType => {
                let event = *raw.drop();

                let text = str::raw::from_c_str(event.file);
                ll::SDL_free(event.file as *c_void);

                DropFileEvent(event.timestamp as uint, text)
            }

            FirstEventType | LastEventType => NoEvent,

            // If we have no other match and the event type is >= 32768
            // this is a user event
            UserEventType => {
                if raw_type < 32768 {
                    return NoEvent;
                }

                let event = *raw.user();

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                UserEvent(event.timestamp as uint, window, raw_type as uint,
                          event.code as int)
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
    else { NoEvent }
}

/// Wait indefinitely for the next available event.
pub fn wait_event() -> Result<Event, ~str> {
    let raw = null_event();
    let success = unsafe { ll::SDL_WaitEvent(&raw) == 1 as c_int };

    if success { Ok(Event::from_ll(&raw)) }
    else { Err(get_error()) }
}

/// Wait until the specified timeout (in milliseconds) for the next available event.
pub fn wait_event_timeout(timeout: int) -> Result<Event, ~str> {
    let raw = null_event();
    let success = unsafe { ll::SDL_WaitEventTimeout(&raw, timeout as c_int) ==
                           1 as c_int };

    if success { Ok(Event::from_ll(&raw)) }
    else { Err(get_error()) }
}

extern "C" fn event_filter_wrapper(userdata: *c_void, event: *ll::SDL_Event) -> c_int {
    let filter: extern fn(event: Event) -> bool = unsafe { mem::transmute(userdata) };
    if event.is_null() { 1 }
    else { filter(Event::from_ll(unsafe { mem::transmute(event) })) as c_int }
}

/// Set up a filter to process all events before they change internal state and are posted to the internal event queue.
pub fn set_event_filter(filter_func: extern fn(event: Event) -> bool) {
    unsafe { ll::SDL_SetEventFilter(event_filter_wrapper,
                                    mem::transmute(filter_func)) }
}

/// Add a callback to be triggered when an event is added to the event queue.
pub fn add_event_watch(filter_func: extern fn(event: Event) -> bool) {
    unsafe { ll::SDL_AddEventWatch(event_filter_wrapper,
                                   mem::transmute(filter_func)) }
}

/// Remove an event watch callback added.
pub fn delete_event_watch(filter_func: extern fn(event: Event) -> bool) {
    unsafe { ll::SDL_DelEventWatch(event_filter_wrapper,
                                   mem::transmute(filter_func)) }
}

/// Run a specific filter function on the current event queue, removing any events for which the filter returns 0.
pub fn filter_events(filter_func: extern fn(event: Event) -> bool) {
    unsafe { ll::SDL_FilterEvents(event_filter_wrapper,
                                  mem::transmute(filter_func)) }
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
pub fn push_event(event: Event) -> Result<(), ~str> {
    match event.to_ll() {
        Some(raw_event) => {
            let ok = unsafe { ll::SDL_PushEvent(&raw_event) == 1 };
            if ok { Ok(()) }
            else { Err(get_error()) }
        },
        None => {
            Err("Unsupport event type to push back to queue.".to_owned())
        }
    }
}
