#![doc(hidden)]
#![allow(non_camel_case_types, non_snake_case)]
use libc::{c_float, c_int, c_char, c_uint, c_void, int16_t,
           int32_t, uint8_t, uint16_t, uint32_t};
use gesture::SDL_GestureID;
use keyboard::SDL_Keysym;
use touch::SDL_FingerID;
use touch::SDL_TouchID;
#[cfg(feature = "no_std")]
use core::prelude::*;

pub type SDL_bool = c_int;

// SDL_events.h
pub type SDL_EventState = uint8_t;
pub const SDL_DISABLE: SDL_EventState = 0;
pub const SDL_ENABLE: SDL_EventState = 1;
pub const SDL_QUERY: SDL_EventState = 0xFF;

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_CommonEvent {
    pub _type: uint32_t,
    pub timestamp: uint32_t,
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TextEditingEvent {
    pub _type: uint32_t,
    pub timestamp: uint32_t,
    pub windowID: uint32_t,
    pub text: [c_char; 32],
    pub start: int32_t,
    pub length: int32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TextInputEvent {
    pub _type: uint32_t,
    pub timestamp: uint32_t,
    pub windowID: uint32_t,
    pub text: [c_char; 32],
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseWheelEvent {
    pub _type: uint32_t,
    pub timestamp: uint32_t,
    pub windowID: uint32_t,
    pub which: uint32_t,
    pub x: int32_t,
    pub y: int32_t,
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyDeviceEvent {
    pub _type: uint32_t,
    pub timestamp: uint32_t,
    pub which: int32_t,
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerDeviceEvent {
    pub _type: uint32_t,
    pub timestamp: uint32_t,
    pub which: int32_t,
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_DropEvent {
    pub _type: uint32_t,
    pub timestamp: uint32_t,
    pub file: *const c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_QuitEvent {
    pub _type: uint32_t,
    pub timestamp: uint32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_OSEvent {
    pub _type: uint32_t,
    pub timestamp: uint32_t,
}

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_UserEvent {
    pub _type: uint32_t,
    pub timestamp: uint32_t,
    pub windowID: uint32_t,
    pub code: int32_t,
    pub data1: *const c_void,
    pub data2: *const c_void,
}

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_SysWMEvent {
    pub _type: uint32_t,
    pub timestamp: uint32_t,
    pub msg: *const SDL_SysWMmsg,
}

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_Event {
    pub data: [uint8_t; 56],
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
    pub fn SDL_PeepEvents(events: *mut SDL_Event, numevents: c_int,
                                action: SDL_eventaction,
                                minType: uint32_t, maxType: uint32_t) -> c_int;
    pub fn SDL_HasEvent(_type: uint32_t) -> SDL_bool;
    pub fn SDL_HasEvents(minType: uint32_t, maxType: uint32_t) ->
              SDL_bool;
    pub fn SDL_FlushEvent(_type: uint32_t);
    pub fn SDL_FlushEvents(minType: uint32_t, maxType: uint32_t);
    pub fn SDL_PollEvent(event: *mut SDL_Event) -> c_int;
    pub fn SDL_WaitEvent(event: *mut SDL_Event) -> c_int;
    pub fn SDL_WaitEventTimeout(event: *mut SDL_Event, timeout: c_int) ->
              c_int;
    pub fn SDL_PushEvent(event: *mut SDL_Event) -> c_int;
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
