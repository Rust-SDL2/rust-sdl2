use std::os::raw::{c_int,c_char};
use video::SDL_Window;

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum SDL_MessageBoxFlags {
    SDL_MESSAGEBOX_ERROR = 16,
    SDL_MESSAGEBOX_WARNING = 32,
    SDL_MESSAGEBOX_INFORMATION = 64,
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum SDL_MessageBoxButtonFlags {
    SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT = 1,
    SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SDL_MessageBoxButtonData {
    pub flags: u32,
    pub buttonid: c_int,
    pub text: *const c_char,
}
impl ::std::default::Default for SDL_MessageBoxButtonData {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SDL_MessageBoxColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum SDL_MessageBoxColorType {
    SDL_MESSAGEBOX_COLOR_BACKGROUND = 0,
    SDL_MESSAGEBOX_COLOR_TEXT = 1,
    SDL_MESSAGEBOX_COLOR_BUTTON_BORDER = 2,
    SDL_MESSAGEBOX_COLOR_BUTTON_BACKGROUND = 3,
    SDL_MESSAGEBOX_COLOR_BUTTON_SELECTED = 4,
    SDL_MESSAGEBOX_COLOR_MAX = 5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SDL_MessageBoxColorScheme {
    pub colors: [SDL_MessageBoxColor; 5usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SDL_MessageBoxData {
    pub flags: u32,
    pub window: *mut SDL_Window,
    pub title: *const c_char,
    pub message: *const c_char,
    pub numbuttons: c_int,
    pub buttons: *const SDL_MessageBoxButtonData,
    pub color_scheme: *const SDL_MessageBoxColorScheme,
}

extern "C" {
    pub fn SDL_ShowMessageBox(messageboxdata: *const SDL_MessageBoxData,
                              buttonid: *mut c_int)
        -> c_int;

    pub fn SDL_ShowSimpleMessageBox(flags: u32,
                                    title: *const c_char,
                                    message: *const c_char,
                                    window: *mut SDL_Window)
        -> c_int;
}
