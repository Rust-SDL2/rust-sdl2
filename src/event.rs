use std::cast;
use std::libc::c_int;
use std::num::IntConvertible;
use std::str;
use std::vec;

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

pub mod ll {
    use std::cast;
    use std::libc::{c_float, c_int, c_schar, c_uint, c_void, int16_t,
                    int32_t, uint8_t, uint16_t, uint32_t};
    use std::ptr;
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
        _type: uint32_t,
        timestamp: uint32_t,
    }

    pub struct SDL_WindowEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        windowID: uint32_t,
        event: uint8_t,
        padding1: uint8_t,
        padding2: uint8_t,
        padding3: uint8_t,
        data1: int32_t,
        data2: int32_t,
    }

    pub struct SDL_KeyboardEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        windowID: uint32_t,
        state: uint8_t,
        repeat: uint8_t,
        padding2: uint8_t,
        padding3: uint8_t,
        keysym: SDL_Keysym,
    }

    pub struct SDL_TextEditingEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        windowID: uint32_t,
        text: [c_schar, ..32u],
        start: int32_t,
        length: int32_t,
    }

    pub struct SDL_TextInputEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        windowID: uint32_t,
        text: [c_schar, ..32u],
    }

    pub struct SDL_MouseMotionEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        windowID: uint32_t,
        which: uint32_t,
        state: uint32_t,
        x: int32_t,
        y: int32_t,
        xrel: int32_t,
        yrel: int32_t,
    }

    pub struct SDL_MouseButtonEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        windowID: uint32_t,
        which: uint32_t,
        button: uint8_t,
        state: uint8_t,
        padding1: uint8_t,
        padding2: uint8_t,
        x: int32_t,
        y: int32_t,
    }

    pub struct SDL_MouseWheelEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        windowID: uint32_t,
        which: uint32_t,
        x: int32_t,
        y: int32_t,
    }

    pub struct SDL_JoyAxisEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        which: int32_t,
        axis: uint8_t,
        padding1: uint8_t,
        padding2: uint8_t,
        padding3: uint8_t,
        value: int16_t,
        padding4: uint16_t,
    }

    pub struct SDL_JoyBallEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        which: int32_t,
        ball: uint8_t,
        padding1: uint8_t,
        padding2: uint8_t,
        padding3: uint8_t,
        xrel: int16_t,
        yrel: int16_t,
    }

    pub struct SDL_JoyHatEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        which: int32_t,
        hat: uint8_t,
        value: uint8_t,
        padding1: uint8_t,
        padding2: uint8_t,
    }

    pub struct SDL_JoyButtonEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        which: int32_t,
        button: uint8_t,
        state: uint8_t,
        padding1: uint8_t,
        padding2: uint8_t,
    }

    pub struct SDL_JoyDeviceEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        which: int32_t,
    }

    pub struct SDL_ControllerAxisEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        which: int32_t,
        axis: uint8_t,
        padding1: uint8_t,
        padding2: uint8_t,
        padding3: uint8_t,
        value: int16_t,
        padding4: uint16_t,
    }

    pub struct SDL_ControllerButtonEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        which: int32_t,
        button: uint8_t,
        state: uint8_t,
        padding1: uint8_t,
        padding2: uint8_t,
    }

    pub struct SDL_ControllerDeviceEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        which: int32_t,
    }

    pub struct SDL_TouchFingerEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        touchId: SDL_TouchID,
        fingerId: SDL_FingerID,
        x: c_float,
        y: c_float,
        dx: c_float,
        dy: c_float,
        pressure: c_float,
    }

    pub struct SDL_MultiGestureEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        touchId: SDL_TouchID,
        dTheta: c_float,
        dDist: c_float,
        x: c_float,
        y: c_float,
        numFingers: uint16_t,
        padding: uint16_t,
    }

    pub struct SDL_DollarGestureEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        touchId: SDL_TouchID,
        gestureId: SDL_GestureID,
        numFingers: uint32_t,
        error: c_float,
        x: c_float,
        y: c_float,
    }

    pub struct SDL_DropEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        file: *c_schar,
    }

    pub struct SDL_QuitEvent {
        _type: uint32_t,
        timestamp: uint32_t,
    }

    pub struct SDL_OSEvent {
        _type: uint32_t,
        timestamp: uint32_t,
    }

    pub struct SDL_UserEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        windowID: uint32_t,
        code: int32_t,
        data1: *c_void,
        data2: *c_void,
    }

    pub struct SDL_SysWMEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        msg: *SDL_SysWMmsg,
    }

    pub struct SDL_Event {
        data: [uint8_t, ..56u],
    }

    impl SDL_Event {
        pub fn _type(&self) -> *uint32_t {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn common(&self) -> *SDL_CommonEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn window(&self) -> *SDL_WindowEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn key(&self) -> *SDL_KeyboardEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn edit(&self) -> *SDL_TextEditingEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn text(&self) -> *SDL_TextInputEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn motion(&self) -> *SDL_MouseMotionEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn button(&self) -> *SDL_MouseButtonEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn wheel(&self) -> *SDL_MouseWheelEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn jaxis(&self) -> *SDL_JoyAxisEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn jball(&self) -> *SDL_JoyBallEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn jhat(&self) -> *SDL_JoyHatEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn jbutton(&self) -> *SDL_JoyButtonEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn jdevice(&self) -> *SDL_JoyDeviceEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn caxis(&self) -> *SDL_ControllerAxisEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn cbutton(&self) -> *SDL_ControllerButtonEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn cdevice(&self) -> *SDL_ControllerDeviceEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn quit(&self) -> *SDL_QuitEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn user(&self) -> *SDL_UserEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn syswm(&self) -> *SDL_SysWMEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn tfinger(&self) -> *SDL_TouchFingerEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn mgesture(&self) -> *SDL_MultiGestureEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn dgesture(&self) -> *SDL_DollarGestureEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn drop(&self) -> *SDL_DropEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn padding(&self) -> *[uint8_t, ..56u] {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }
    }

    pub type SDL_eventaction = c_uint;
    pub static SDL_ADDEVENT: SDL_eventaction = 0;
    pub static SDL_PEEKEVENT: SDL_eventaction = 1;
    pub static SDL_GETEVENT: SDL_eventaction = 2;
    pub type SDL_EventFilter =
        extern "C" fn(arg1: *c_void, arg2: *SDL_Event) -> c_int;

    externfn!(fn SDL_PumpEvents())
    externfn!(fn SDL_PeepEvents(events: &[SDL_Event], numevents: c_int,
                                action: SDL_eventaction, minType: uint32_t,
                                maxType: uint32_t) -> c_int)
    externfn!(fn SDL_HasEvent(_type: uint32_t) -> SDL_bool)
    externfn!(fn SDL_HasEvents(minType: uint32_t, maxType: uint32_t) ->
              SDL_bool)
    externfn!(fn SDL_FlushEvent(_type: uint32_t))
    externfn!(fn SDL_FlushEvents(minType: uint32_t, maxType: uint32_t))
    externfn!(fn SDL_PollEvent(event: *SDL_Event) -> c_int)
    externfn!(fn SDL_WaitEvent(event: *SDL_Event) -> c_int)
    externfn!(fn SDL_WaitEventTimeout(event: *SDL_Event, timeout: c_int) ->
              c_int)
    externfn!(fn SDL_PushEvent(event: *SDL_Event) -> c_int)
    externfn!(fn SDL_SetEventFilter(filter: SDL_EventFilter,
                                    userdata: *c_void))
    externfn!(fn SDL_GetEventFilter(filter: *SDL_EventFilter,
                                    userdata: **c_void) -> SDL_bool)
    externfn!(fn SDL_AddEventWatch(filter: SDL_EventFilter, userdata: *c_void))
    externfn!(fn SDL_DelEventWatch(filter: SDL_EventFilter, userdata: *c_void))
    externfn!(fn SDL_FilterEvents(filter: SDL_EventFilter, userdata: *c_void))
    externfn!(fn SDL_EventState(_type: uint32_t, state: SDL_EventState) ->
              SDL_EventState)
    externfn!(fn SDL_RegisterEvents(numevents: c_int) -> uint32_t)
}

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

    // TODO: ControllerAxisMotionEventType = ll::SDL_CONTROLLERAXISMOTION,
    // TODO: ControllerButtonDownEventType = ll::SDL_CONTROLLERBUTTONDOWN,
    // TODO: ControllerButtonUpEventType = ll::SDL_CONTROLLERBUTTONUP,
    // TODO: ControllerDeviceAddedEventType = ll::SDL_CONTROLLERDEVICEADDED,
    // TODO: ControllerDeviceRemovedEventType = ll::SDL_CONTROLLERDEVICEREMOVED,
    // TODO: ControllerDeviceRemappedEventType = ll::SDL_CONTROLLERDEVICEREMAPPED,

    // TODO: FingerDownEventType = ll:SDL_FINGERDOWN,
    // TODO: FingerUpEventType = ll::SDL_FINGERUP,
    // TODO: FingerMotionEventType = ll::SDL_FINGERMOTION,
    // TODO: DollarGestureEventType = ll::SDL_DOLLARGESTURE,
    // TODO: DollarRecordEventType = ll::SDL_DOLLARRECORD,
    // TODO: MultiGestureEventType = ll::SDL_MULTIGESTURE,

    // TODO: ClipboardUpdateEventType = ll::SDL_CLIPBOARDUPDATE,
    // TODO: DropFileEventType = ll::SDL_DROPFILE,

    UserEventType = ll::SDL_USEREVENT,
    LastEventType = ll::SDL_LASTEVENT,
}

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

pub enum Event {
    NoEvent,

    QuitEvent(uint),
    AppTerminatingEvent(uint),
    AppLowMemoryEvent(uint),
    AppWillEnterBackgroundEvent(uint),
    AppDidEnterBackgroundEvent(uint),
    AppWillEnterForegroundEvent(uint),
    AppDidEnterForegroundEvent(uint),

    WindowEvent(uint, ~video::Window, WindowEventId, int, int),
    // TODO: SysWMEvent

    KeyDownEvent(uint, ~video::Window, KeyCode, ScanCode, ~[Mod]),
    KeyUpEvent(uint, ~video::Window, KeyCode, ScanCode, ~[Mod]),
    TextEditingEvent(uint, ~video::Window, ~str, int, int),
    TextInputEvent(uint, ~video::Window, ~str),

    MouseMotionEvent(uint, ~video::Window, uint, ~[MouseState], int, int,
                     int, int),
    MouseButtonDownEvent(uint, ~video::Window, uint, Mouse, int, int),
    MouseButtonUpEvent(uint, ~video::Window, uint, Mouse, int, int),
    MouseWheelEvent(uint, ~video::Window, uint, int, int),

    JoyAxisMotionEvent(uint, int, int, i16),
    JoyBallMotionEvent(uint, int, i16, i16),
    JoyHatMotionEvent(uint, int, int, ~[HatState]),
    JoyButtonDownEvent(uint, int, int),
    JoyButtonUpEvent(uint, int, int),
    JoyDeviceAddedEvent(uint, int),
    JoyDeviceRemovedEvent(uint, int),

    // TODO: ControllerAxisMotionEvent
    // TODO: ControllerButtonDownEvent
    // TODO: ControllerButtonUpEvent
    // TODO: ControllerDeviceAddedEvent
    // TODO: ControllerDeviceRemovedEvent
    // TODO: ControllerDeviceRemappedEvent

    // TODO: FingerDownEvent
    // TODO: FingerUpEvent
    // TODO: FingerMotionEvent
    // TODO: DollarGestureEvent
    // TODO: DollarRecordEvent
    // TODO: MultiGestureEvent

    // TODO: ClipboardUpdateEvent
    // TODO: DropFileEvent

    UserEvent(uint, ~video::Window, int),
}

impl Event {
}

fn wrap_event(raw: ll::SDL_Event) -> Event {
    unsafe {
        let raw_type = raw._type();
        let raw_type = if raw_type.is_null() { return NoEvent; }
                       else { *raw_type };

        // FIXME: This is incredibly hacky
        let event_type: EventType = cast::transmute(raw_type as uint);
        match event_type {
            QuitEventType => {
                let event = raw.quit();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                QuitEvent(event.timestamp as uint)
            }
            AppTerminatingEventType => {
                let event = raw.common();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                AppTerminatingEvent(event.timestamp as uint)
            }
            AppLowMemoryEventType => {
                let event = raw.common();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                AppLowMemoryEvent(event.timestamp as uint)
            }
            AppWillEnterBackgroundEventType => {
                let event = raw.common();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                AppWillEnterBackgroundEvent(event.timestamp as uint)
            }
            AppDidEnterBackgroundEventType => {
                let event = raw.common();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                AppDidEnterBackgroundEvent(event.timestamp as uint)
            }
            AppWillEnterForegroundEventType => {
                let fore = raw.common();
                let fore = if fore.is_null() { return NoEvent; }
                           else { *fore };

                AppWillEnterForegroundEvent(fore.timestamp as uint)
            }
            AppDidEnterForegroundEventType => {
                let event = raw.common();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                AppDidEnterForegroundEvent(event.timestamp as uint)
            }

            WindowEventType => {
                let event = raw.window();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                WindowEvent(event.timestamp as uint, window,
                            wrap_window_event_id(event.event),
                            event.data1 as int, event.data2 as int)
            }
            // TODO: SysWMEventType

            KeyDownEventType => {
                let event = raw.key();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                KeyDownEvent(event.timestamp as uint, window,
                             IntConvertible::from_int(event.keysym.sym as int),
                             IntConvertible::from_int(event.keysym.scancode as int),
                             keyboard::wrap_mod_state(event.keysym._mod as SDL_Keymod))
            }
            KeyUpEventType => {
                let event = raw.key();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                KeyUpEvent(event.timestamp as uint, window,
                           IntConvertible::from_int(event.keysym.sym as int),
                           IntConvertible::from_int(event.keysym.scancode as int),
                           keyboard::wrap_mod_state(event.keysym._mod as SDL_Keymod))
            }
            TextEditingEventType => {
                let event = raw.edit();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                // FIXME: This seems really poorly done
                let mut text: ~[u8] = vec::with_capacity(32);
                for c in event.text.iter() {
                    if *c == 0 {
                        break;
                    }
                    text.push(c.clone() as u8);
                }

                TextEditingEvent(event.timestamp as uint, window,
                                 str::from_utf8(text),
                                 event.start as int, event.length as int)
            }
            TextInputEventType => {
                let event = raw.text();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                // FIXME: This seems really poorly done
                let mut text: ~[u8] = vec::with_capacity(32);
                for c in event.text.iter() {
                    if *c == 0 {
                        break;
                    }
                    text.push(c.clone() as u8);
                }

                TextInputEvent(event.timestamp as uint, window,
                               str::from_utf8(text))
            }

            MouseMotionEventType => {
                let event = raw.motion();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                MouseMotionEvent(event.timestamp as uint, window,
                                 event.which as uint,
                                 mouse::wrap_mouse_state(event.state),
                                 event.x as int, event.y as int,
                                 event.xrel as int, event.yrel as int)
            }
            MouseButtonDownEventType => {
                let event = raw.button();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

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
                let event = raw.button();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

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
                let event = raw.button();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

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
                let event = raw.jaxis();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                JoyAxisMotionEvent(event.timestamp as uint, event.which as int,
                                   event.axis as int, event.value)
            }
            JoyBallMotionEventType => {
                let event = raw.jball();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                JoyBallMotionEvent(event.timestamp as uint, event.which as int,
                                   event.xrel, event.yrel)
            }
            JoyHatMotionEventType => {
                let event = raw.jhat();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                JoyHatMotionEvent(event.timestamp as uint, event.which as int,
                                  event.hat as int,
                                  joystick::wrap_hat_state(event.value))
            }
            JoyButtonDownEventType => {
                let event = raw.jbutton();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                JoyButtonDownEvent(event.timestamp as uint, event.which as int,
                                   event.button as int)
            }
            JoyButtonUpEventType => {
                let event = raw.jbutton();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                JoyButtonUpEvent(event.timestamp as uint, event.which as int,
                                 event.button as int)
            }
            JoyDeviceAddedEventType => {
                let event = raw.jdevice();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                JoyDeviceAddedEvent(event.timestamp as uint,
                                    event.which as int)
            }
            JoyDeviceRemovedEventType => {
                let event = raw.jdevice();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                JoyDeviceRemovedEvent(event.timestamp as uint,
                                      event.which as int)
            }

            // TODO: All the controller and touch events

            UserEventType => {
                let event = raw.user();
                let event = if event.is_null() { return NoEvent; }
                            else { *event };

                let window = video::Window::from_id(event.windowID);
                let window = match window {
                    Err(_) => return NoEvent,
                    Ok(window) => window,
                };

                UserEvent(event.timestamp as uint, window, event.code as int)
            }

            FirstEventType => NoEvent,
            LastEventType => NoEvent,
            //_ => NoEvent
        }
    }
}

fn wrap_window_event_id(id: u8) -> WindowEventId {
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

fn null_event() -> ll::SDL_Event {
    ll::SDL_Event { data: [0, ..56] }
}

pub fn pump_events() {
    unsafe { ll::SDL_PumpEvents(); }
}

pub fn poll_event() -> Event {
    pump_events();

    let raw = null_event();
    let success = unsafe { ll::SDL_PollEvent(&raw) == 1 as c_int };

    if success { wrap_event(raw) }
    else { NoEvent }
}
