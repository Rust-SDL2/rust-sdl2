use std::ptr;

use sys::touch as ll;

pub type Finger = ll::Finger;
pub type TouchDevice = ll::TouchDevice;

pub fn get_num_touch_devices() -> i32 {
    unsafe { ll::SDL_GetNumTouchDevices() }
}

pub fn get_touch_device(index: i32) -> TouchDevice {
    unsafe { ll::SDL_GetTouchDevice(index) }
}

pub fn get_num_touch_fingers(touch: TouchDevice) -> i32 {
    unsafe { ll::SDL_GetNumTouchFingers(touch) }
}

pub fn get_touch_finger(touch: TouchDevice, index: i32) -> Option<Finger> {
    let raw = unsafe { ll::SDL_GetTouchFinger(touch, index) };

    if raw == ptr::null() {
        None
    } else {
        unsafe { Some(*raw) }
    }
}
