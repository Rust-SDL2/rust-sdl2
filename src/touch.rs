
pub mod ll {
	use std::libc::{c_float, c_int, int64_t};

	pub type SDL_TouchID = int64_t;
	pub type SDL_FingerID = int64_t;

	pub struct SDL_Finger {
	    id: SDL_FingerID,
	    x: c_float,
	    y: c_float,
	    pressure: c_float,
	}

	externfn!(fn SDL_GetNumTouchDevices() -> c_int)
	externfn!(fn SDL_GetTouchDevice(index: c_int) -> SDL_TouchID)
	externfn!(fn SDL_GetNumTouchFingers(touchID: SDL_TouchID) -> c_int)
	externfn!(fn SDL_GetTouchFinger(touchID: SDL_TouchID, index: c_int))
}
