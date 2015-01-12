use std::ptr;

pub use sys::touch as ll;

pub type Finger = ll::Finger;
pub type TouchDevice = ll::TouchDevice;

pub fn get_num_touch_devices() -> isize {
    unsafe { ll::SDL_GetNumTouchDevices() as isize }
}

pub fn get_touch_device(index: isize) -> TouchDevice {
    unsafe { ll::SDL_GetTouchDevice(index as i32) }
}

pub fn get_num_touch_fingers(touch: TouchDevice) -> isize {
    unsafe { ll::SDL_GetNumTouchFingers(touch) as isize }
}

pub fn get_touch_finger(touch: TouchDevice, index: isize) -> Option<Finger> {
    let raw = unsafe { ll::SDL_GetTouchFinger(touch, index as i32) };

    if raw == ptr::null() {
        None
    } else {
        unsafe { Some(*raw) }
    }
}
