
pub mod ll {
	use std::libc::{c_int, int64_t};
	use rwops::ll::SDL_RWops;
	use touch::ll::SDL_TouchID;

	pub type SDL_GestureID = int64_t;

	externfn!(fn SDL_RecordGesture(touchId: SDL_TouchID) -> c_int)
	externfn!(fn SDL_SaveAllDollarTemplates(src: *SDL_RWops) -> c_int)
	externfn!(fn SDL_SaveDollarTemplate(gestureId: SDL_GestureID,
		                                src: *SDL_RWops) -> c_int)
	externfn!(fn SDL_LoadDollarTemplates(touchId: SDL_TouchID, src: *SDL_RWops)
		      -> c_int)
}
