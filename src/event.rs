use std::cast;
use std::libc::{c_int, uint32_t};

pub mod ll {
    use std::cast;
    use std::libc::{c_float, c_int, c_uint, c_void, int16_t, int32_t, int64_t, uint8_t, uint16_t, uint32_t};
    use std::ptr;

    pub type SDLScancode = c_uint;
    pub type SDLKeycode = c_uint;
    pub type SDLMod = uint16_t;
    pub type SDL_SysWMmsg = c_void;
    pub type SDL_JoystickID = int32_t;
    pub type SDL_TouchID = int64_t;
    pub type SDL_FingerID = int64_t;
    pub type SDL_GestureID = int64_t;

    pub static SDL_DISABLE: c_int = 0;
    pub static SDL_ENABLE: c_int = 1;
    pub static SDL_QUERY: c_int = -1;

    pub static SDL_TEXTEDITINGEVENT_TEXT_SIZE: c_int = 32;
    pub static SDL_TEXTINPUTEVENT_TEXT_SIZE: c_int = 32;

    pub struct SDL_keysym {
        scancode: SDLScancode,
        sym: SDLKeycode,
        _mod: SDLMod,
        unused: c_uint,
    }

    pub struct SDL_Event {
        data: [uint8_t, ..56],
    }

    pub struct SDL_WindowEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        windowID: uint32_t,
        event: uint8_t,
        padding: [uint8_t, ..3],
        data1: int32_t,
        data2: int32_t,
    }

    pub struct SDL_KeyboardEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        windowID: uint32_t,
        state: uint8_t,
        repeat: uint8_t,
        padding: [uint8_t, ..2],
        keysym: SDL_keysym,
    }

    pub struct SDL_TextEditingEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        windowID: uint32_t,
        text: [u8, ..32],
        start: int32_t,
        length: int32_t,
    }

    pub struct SDL_TextInputEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        windowID: uint32_t,
        text: [u8, ..32],
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
        padding: [uint8_t, ..2],
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
        which: SDL_JoystickID,
        axis: uint8_t,
        padding: [uint8_t, ..3],
        value: int16_t,
        padding2: int16_t,
    }

    pub struct SDL_JoyBallEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        which: SDL_JoystickID,
        ball: uint8_t,
        padding: [uint8_t, ..3],
        xrel: int16_t,
        yrel: int16_t,
    }

    pub struct SDL_JoyHatEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        which: SDL_JoystickID,
        hat: uint8_t,
        value: uint8_t,
        padding: [uint8_t, ..2],
    }

    pub struct SDL_JoyButtonEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        which: SDL_JoystickID,
        button: uint8_t,
        state: uint8_t,
        padding: [uint8_t, ..2],
    }

    pub struct SDL_JoyDeviceEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        which: int32_t,
    }

    pub struct SDL_ControllerAxisEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        which: SDL_JoystickID,
        axis: uint8_t,
        padding: [uint8_t, ..3],
        value: int16_t,
        padding2: int16_t,
    }

    pub struct SDL_ControllerButtonEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        which: SDL_JoystickID,
        button: uint8_t,
        state: uint8_t,
        padding: [uint8_t, ..2],
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

    pub struct SDL_DropString {
        data: *u8,
    }

    impl Drop for SDL_DropString {
        fn drop(&self) {
            if !self.data.is_null() {
                unsafe { SDL_free(self.data as *c_void); }
            }
        }
    }

    pub struct SDL_DropEvent {
        _type: uint32_t,
        timestamp: uint32_t,
        file: SDL_DropString,
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

    impl SDL_Event {
        pub fn _type(&self) -> *uint32_t {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn window(&self) -> *SDL_WindowEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn keyboard(&self) -> *SDL_KeyboardEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn text_edit(&self) -> *SDL_TextEditingEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn text_input(&self) -> *SDL_TextInputEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn mouse_motion(&self) -> *SDL_MouseMotionEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn mouse_button(&self) -> *SDL_MouseButtonEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn mouse_wheel(&self) -> *SDL_MouseWheelEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn joy_axis(&self) -> *SDL_JoyAxisEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn joy_ball(&self) -> *SDL_JoyBallEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn joy_hat(&self) -> *SDL_JoyHatEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn joy_button(&self) -> *SDL_JoyButtonEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn joy_device(&self) -> *SDL_JoyDeviceEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn controller_axis(&self) -> *SDL_ControllerAxisEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn controller_button(&self) -> *SDL_ControllerButtonEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn controller_device(&self) -> *SDL_ControllerDeviceEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn touch(&self) -> *SDL_TouchFingerEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn gesture(&self) -> *SDL_MultiGestureEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn dollar(&self) -> *SDL_DollarGestureEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn drop(&self) -> *SDL_DropEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn quit(&self) -> *SDL_QuitEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn os(&self) -> *SDL_OSEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn user(&self) -> *SDL_UserEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn syswm(&self) -> *SDL_SysWMEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }
    }

    externfn!(fn SDL_free(mem: *c_void))
    externfn!(fn SDL_PumpEvents())
    externfn!(fn SDL_HasEvent(_type: uint32_t) -> c_int)
    externfn!(fn SDL_HasEvents(min_type: uint32_t, max_type: uint32_t) -> c_int)
    externfn!(fn SDL_FlushEvent(_type: uint32_t) -> c_int)
    externfn!(fn SDL_FlushEvents(min_type: uint32_t, max_type: uint32_t) -> c_int)
    externfn!(fn SDL_PollEvent(event: *SDL_Event) -> c_int)
    externfn!(fn SDL_WaitEvent(event: *SDL_Event) -> c_int)
    externfn!(fn SDL_WaitEventTimeout(event: *SDL_Event, timeout: c_int) -> c_int)
    externfn!(fn SDL_EventState(_type: uint32_t, state: c_int) -> uint8_t)
    externfn!(fn SDL_GetModState() -> SDLMod)
    externfn!(fn SDL_SetModState(modstate: SDLMod))
}

#[deriving(Eq)]
pub enum Mod {
     NoMod = 0x0000,
     LShiftMod = 0x0001,
     RShiftMod = 0x0002,
     LCtrlMod = 0x0040,
     RCtrlMod = 0x0080,
     LAltMod = 0x0100,
     RAltMod = 0x0200,
     LGuiMod = 0x0400,
     RGuiMod = 0x0800,
     NumMod = 0x1000,
     CapsMod = 0x2000,
     ModeMod = 0x4000,
     ReservedMod = 0x8000
}

fn wrap_mod_state(bitflags: ll::SDLMod) -> ~[Mod] {
    let flags = [NoMod,
        LShiftMod,
        RShiftMod,
        LCtrlMod,
        RCtrlMod,
        LAltMod,
        RAltMod,
        LGuiMod,
        RGuiMod,
        NumMod,
        CapsMod,
        ModeMod,
        ReservedMod];

    do flags.iter().filter_map |&flag| {
        if bitflags & (flag as ll::SDLMod) != 0 { Some(flag) }
        else { None }
    }.collect()
}

#[deriving(Eq)]
pub enum HatState {
    CenteredHatState,
    UpHatState,
    RightHatState,
    DownHatState,
    LeftHatState
}

fn wrap_hat_state(bitflags: u8) -> ~[HatState] {
    let flags = [CenteredHatState,
        UpHatState,
        RightHatState,
        DownHatState,
        LeftHatState];

    do flags.iter().filter_map |&flag| {
        if bitflags & (flag as u8) != 0 { Some(flag) }
        else { None }
    }.collect()
}

#[deriving(Eq)]
pub enum Mouse {
    LeftMouse,
    MiddleMouse,
    RightMouse,
}

fn wrap_mouse(bitflags: u8) -> Mouse {
    match bitflags {
        1 => LeftMouse,
        2 => MiddleMouse,
        3 => RightMouse,
        _ => fail!(~"unhandled mouse type")
    }
}

#[deriving(Eq)]
pub enum MouseState {
    LeftMouseState = 1,
    MiddleMouseState,
    RightMouseState,
    X1MouseState,
    X2MouseState
}

fn wrap_mouse_state(bitflags: u32) -> ~[MouseState] {
    let flags = [LeftMouseState,
        MiddleMouseState,
        RightMouseState,
        X1MouseState,
        X2MouseState];

    do flags.iter().filter_map |&flag| {
        if bitflags & (flag as u32) != 0 { Some(flag) }
        else { None }
    }.collect()
}

pub enum Event {
    NoEvent,
    WindowEvent(uint, u8, int, int),
    // TODO: KeyboardEvent
    // TODO: TextEditingEvent
    // TODO: TextInputEvent
    MouseMotionEvent(uint, uint, ~[MouseState], int, int, int, int),
    MouseButtonEvent(uint, uint, Mouse, bool, int, int),
    MouseWheelEvent(uint, uint, int, int),
    JoyAxisEvent(int, u8, i16),
    JoyBallEvent(int, u8, i16, i16),
    JoyHatEvent(int, u8, ~[HatState]),
    JoyButtonEvent(int, u8, bool),
    JoyDeviceEvent(int),
    ControllerAxisEvent(int, u8, i16),
    ControllerButtonEvent(int, u8, bool),
    ControllerDeviceEvent(int),
    TouchFingerEvent(i64, i64, float, float, float, float, float),
    MultiGestureEvent(i64, float, float, float, float, u16),
    DollarGestureEvent(i64, i64, uint, float, float, float),
    DropEvent(~str),
    QuitEvent(),
    OSEvent(),
    // TODO: UserEvent
    // TODO: SysWMEvent
}

fn null_event() -> ll::SDL_Event {
    ll::SDL_Event { data: [0, ..56] }
}

fn wrap_event(raw: ll::SDL_Event) -> Event {
    unsafe {
        let raw_type = raw._type();
        let raw_type = if raw_type.is_null() { return NoEvent; }
                       else { *raw_type };

        // FIXME: This is incredibly hacky
        let et: EventType = cast::transmute(raw_type as uint);

        match et {
            NoEventType | LastEventType => NoEvent,
            QuitEventType => QuitEvent,

            // TODO: Hook these up, requires event filter
            AppTerminatingEventType => NoEvent,
            AppLowMemoryEventType => NoEvent,
            AppWillEnterBackgroundEventType => NoEvent,
            AppDidEnterBackgroundEventType => NoEvent,
            AppWillEnterForegroundEventType => NoEvent,
            AppDidEnterForegroundEventType => NoEvent,

            WindowEventType => {
                let window = raw.window();
                let window = if window.is_null() { return NoEvent; }
                             else { *window };

                WindowEvent(window.windowID as uint, window.event,
                            window.data1 as int, window.data2 as int)
            }
            // TODO: Hook this up
            SysWMEventType => NoEvent,

            // TODO: Hook these up, requires key map
            KeyDownEventType | KeyUpEventType => NoEvent,
            TextEditingEventType => NoEvent,
            TextInputEventType => NoEvent,

            MouseMotionEventType => {
                let motion = raw.mouse_motion();
                let motion = if motion.is_null() { return NoEvent; }
                             else { *motion };

                MouseMotionEvent(motion.windowID as uint, motion.which as uint,
                                 wrap_mouse_state(motion.state),
                                 motion.x as int, motion.y as int,
                                 motion.xrel as int, motion.yrel as int)
            }
            MouseButtonDownEventType | MouseButtonUpEventType => {
                let button = raw.mouse_button();
                let button = if button.is_null() { return NoEvent; }
                             else { *button };

                MouseButtonEvent(button.windowID as uint, button.which as uint,
                                 wrap_mouse(button.button), button.state == 1,
                                 button.x as int, button.y as int)
            }
            MouseWheelEventType => {
                let wheel = raw.mouse_wheel();
                let wheel = if wheel.is_null() { return NoEvent }
                            else { *wheel };

                MouseWheelEvent(wheel.windowID as uint, wheel.which as uint,
                                wheel.x as int, wheel.y as int)
            }

            // TODO: Hook the remaining event types up
            JoyAxisMotionEventType => NoEvent,
            JoyBallMotionEventType => NoEvent,
            JoyHatMotionEventType => NoEvent,
            JoyButtonDownEventType | JoyButtonUpEventType => NoEvent,
            JoyDeviceAddedEventType => NoEvent,
            JoyDeviceRemovedEventType => NoEvent,

            ControllerAxisMotionEventType => NoEvent,
            ControllerButtonDownEventType | ControllerButtonUpEventType => {
                NoEvent
            }
            ControllerDeviceAddedEventType => NoEvent,
            ControllerDeviceRemovedEventType => NoEvent,
            ControllerDeviceRemappedEventType => NoEvent,

            FingerDownEventType | FingerUpEventType => NoEvent,
            FingerMotionEventType => NoEvent,

            DollarGestureEventType => NoEvent,
            DollarRecordEventType => NoEvent,
            MultiGestureEventType => NoEvent,

            ClipboardUpdateEventType => NoEvent,

            DropFileEventType => NoEvent,

            UserEventType => NoEvent,
        }
    }
}

pub enum EventType {
    NoEventType,

    QuitEventType = 0x100,
    AppTerminatingEventType,
    AppLowMemoryEventType,
    AppWillEnterBackgroundEventType,
    AppDidEnterBackgroundEventType,
    AppWillEnterForegroundEventType,
    AppDidEnterForegroundEventType,

    WindowEventType = 0x200,
    SysWMEventType,

    KeyDownEventType = 0x300,
    KeyUpEventType,
    TextEditingEventType,
    TextInputEventType,

    MouseMotionEventType = 0x400,
    MouseButtonDownEventType,
    MouseButtonUpEventType,
    MouseWheelEventType,

    JoyAxisMotionEventType = 0x600,
    JoyBallMotionEventType,
    JoyHatMotionEventType,
    JoyButtonDownEventType,
    JoyButtonUpEventType,
    JoyDeviceAddedEventType,
    JoyDeviceRemovedEventType,

    ControllerAxisMotionEventType = 0x650,
    ControllerButtonDownEventType,
    ControllerButtonUpEventType,
    ControllerDeviceAddedEventType,
    ControllerDeviceRemovedEventType,
    ControllerDeviceRemappedEventType,

    FingerDownEventType = 0x700,
    FingerUpEventType,
    FingerMotionEventType,

    DollarGestureEventType = 0x800,
    DollarRecordEventType,
    MultiGestureEventType,

    ClipboardUpdateEventType = 0x900,

    DropFileEventType = 0x1000,

    UserEventType = 0x8000,

    LastEventType = 0xffff,
}

impl EventType {
    pub fn get_state(&self) -> bool { get_event_state(*self) }
    pub fn set_state(&self, state: bool) { set_event_state(*self, state) }
}

pub fn pump_events() {
    unsafe { ll::SDL_PumpEvents(); }
}

pub fn wait_event() -> Event {
    wait_event_timeout(-1)
}

pub fn wait_event_timeout(timeout: int) -> Event {
    let raw = null_event();
    let success = if (timeout > 0) {
        unsafe { ll::SDL_WaitEventTimeout(&raw, timeout as i32) == 1 }
    } else {
        unsafe { ll::SDL_WaitEvent(&raw) == 1 as c_int }
    };

    if success { wrap_event(raw) }
    else { NoEvent }
}

pub fn poll_event() -> Event {
    pump_events();

    let raw = null_event();
    let success = unsafe { ll::SDL_PollEvent(&raw) == 1 as c_int };

    if success { wrap_event(raw) }
    else { NoEvent }
}

pub fn set_event_state(et: EventType, state: bool) {
    unsafe { ll::SDL_EventState(et as uint32_t, state as c_int); }
}

pub fn get_event_state(et: EventType) -> bool {
    unsafe { ll::SDL_EventState(et as uint32_t, ll::SDL_QUERY as c_int)
             == ll::SDL_ENABLE as u8 }
}

// TODO: get_key_state

pub fn get_mod_state() -> ~[Mod] {
    unsafe { wrap_mod_state(ll::SDL_GetModState()) }
}

pub fn set_mod_state(states: &[Mod]) {
    unsafe {
        ll::SDL_SetModState(do states.iter().fold(0u16) |states, &state| {
            states | state as ll::SDLMod
        });
    }
}

// TODO: get_key_name
// TODO: joysticks, mice
