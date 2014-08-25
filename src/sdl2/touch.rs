use std::ptr;

pub type TouchDevice = ll::SDL_TouchID;

#[deriving(PartialEq)]
#[repr(C)]
pub struct Finger {
    id: TouchDevice,
    x: f32,
    y: f32,
    pressure: f32,
}

#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_int, int64_t};
    use touch::Finger;

    pub type SDL_TouchID = int64_t;
    pub type SDL_FingerID = int64_t;
    pub type SDL_Finger = Finger;

    extern "C" {
        pub fn SDL_GetNumTouchDevices() -> c_int;
        pub fn SDL_GetTouchDevice(index: c_int) -> SDL_TouchID;
        pub fn SDL_GetNumTouchFingers(touchID: SDL_TouchID) -> c_int;
        pub fn SDL_GetTouchFinger(touchID: SDL_TouchID, index: c_int) ->
                  *const SDL_Finger;
    }
}

pub fn get_num_touch_devices() -> int {
    unsafe { ll::SDL_GetNumTouchDevices() as int }
}

pub fn get_touch_device(index: int) -> TouchDevice {
    unsafe { ll::SDL_GetTouchDevice(index as i32) }
}

pub fn get_num_touch_fingers(touch: TouchDevice) -> int {
    unsafe { ll::SDL_GetNumTouchFingers(touch) as int }
}

pub fn get_touch_finger(touch: TouchDevice, index: int) -> Option<Finger> {
    let raw = unsafe { ll::SDL_GetTouchFinger(touch, index as i32) };

    if raw == ptr::null() {
        None
    } else {
        unsafe { Some(*raw) }
    }
}
