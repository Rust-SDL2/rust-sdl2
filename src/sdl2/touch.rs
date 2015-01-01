use std::ptr;

pub use sys::touch as ll;

pub type Finger = ll::Finger;
pub type TouchDevice = ll::TouchDevice;

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
