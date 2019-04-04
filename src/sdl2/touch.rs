use crate::sys;

pub type Finger = sys::SDL_Finger;
pub type TouchDevice = sys::SDL_TouchID;

pub fn num_touch_devices() -> i32 {
    unsafe { sys::SDL_GetNumTouchDevices() }
}

pub fn touch_device(index: i32) -> TouchDevice {
    unsafe { sys::SDL_GetTouchDevice(index) }
}

pub fn num_touch_fingers(touch: TouchDevice) -> i32 {
    unsafe { sys::SDL_GetNumTouchFingers(touch) }
}

pub fn touch_finger(touch: TouchDevice, index: i32) -> Option<Finger> {
    let raw = unsafe { sys::SDL_GetTouchFinger(touch, index) };

    if raw.is_null() {
        None
    } else {
        unsafe { Some(*raw) }
    }
}
