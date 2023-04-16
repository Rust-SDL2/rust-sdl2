#![no_std]

use core::ptr::null_mut;
use sdl2_sys::*;

fn main() {
    unsafe {
        let mut _window: *mut SDL_Window = null_mut();
        let mut _surface: *mut SDL_Surface = null_mut();
        if SDL_Init(SDL_INIT_VIDEO) < 0 {
            panic!("failed to initialize sdl2 with video");
        };
        _window = SDL_CreateWindow(
            b"hello_sdl2" as *const _ as *const i8,
            SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            640,
            480,
            SDL_WindowFlags::SDL_WINDOW_SHOWN as u32,
        );

        if _window == null_mut() {
            panic!("failed to create window");
        }

        _surface = SDL_GetWindowSurface(_window);
        SDL_FillRect(
            _surface,
            null_mut(),
            SDL_MapRGB((*_surface).format, 0xFF, 0xFF, 0x00),
        );
        SDL_UpdateWindowSurface(_window);
        SDL_Delay(5000);
        SDL_DestroyWindow(_window);
        SDL_Quit();
    }
}
