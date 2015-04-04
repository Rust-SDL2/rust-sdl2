use rect::Rect;
use surface::SDL_Surface;

#[cfg(feature = "no_std")]
use core::prelude::*;
use libc::{c_void, c_int, c_float, c_char, uint16_t, uint32_t};

pub type SDL_Rect = Rect;
pub type SDL_bool = c_int;

//SDL_video.h
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_Window;

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_DisplayMode {
    pub format: uint32_t,
    pub w: c_int,
    pub h: c_int,
    pub refresh_rate: c_int,
    pub driverdata: *const c_void
}

pub type SDL_WindowPos = c_int;
pub const SDL_WINDOWPOS_CENTERED: SDL_WindowPos = 0x2FFF0000;
pub const SDL_WINDOWPOS_UNDEFINED: SDL_WindowPos = 0x1FFF0000;

#[derive(Copy, Clone)]
pub enum SDL_WindowFlags {
    SDL_WINDOW_FULLSCREEN = 0x00000001,
    SDL_WINDOW_OPENGL = 0x00000002,
    SDL_WINDOW_SHOWN = 0x00000004,
    SDL_WINDOW_HIDDEN = 0x00000008,
    SDL_WINDOW_BORDERLESS = 0x00000010,
    SDL_WINDOW_RESIZABLE = 0x00000020,
    SDL_WINDOW_MINIMIZED = 0x00000040,
    SDL_WINDOW_MAXIMIZED = 0x00000080,
    SDL_WINDOW_INPUT_GRABBED = 0x00000100,
    SDL_WINDOW_INPUT_FOCUS = 0x00000200,
    SDL_WINDOW_MOUSE_FOCUS = 0x00000400,
    SDL_WINDOW_FULLSCREEN_DESKTOP = 0x00001001,
    SDL_WINDOW_FOREIGN = 0x00000800,
    SDL_WINDOW_ALLOW_HIGHDPI = 0x00002000
}

#[derive(Copy, Clone)]
pub enum SDL_WindowEventID {
    SDL_WINDOWEVENT_NONE,
    SDL_WINDOWEVENT_SHOWN,
    SDL_WINDOWEVENT_HIDDEN,
    SDL_WINDOWEVENT_EXPOSED,
    SDL_WINDOWEVENT_MOVED,
    SDL_WINDOWEVENT_RESIZED,
    SDL_WINDOWEVENT_SIZE_CHANGED,
    SDL_WINDOWEVENT_MINIMIZED,
    SDL_WINDOWEVENT_MAXIMIZED,
    SDL_WINDOWEVENT_RESTORED,
    SDL_WINDOWEVENT_ENTER,
    SDL_WINDOWEVENT_LEAVE,
    SDL_WINDOWEVENT_FOCUS_GAINED,
    SDL_WINDOWEVENT_FOCUS_LOST,
    SDL_WINDOWEVENT_CLOSE
}

pub type SDL_GLContext = *const c_void;

#[derive(Copy, Clone, FromPrimitive)]
#[repr(C)]
pub enum SDL_GLattr {
    SDL_GL_RED_SIZE = 0,
    SDL_GL_GREEN_SIZE = 1,
    SDL_GL_BLUE_SIZE = 2,
    SDL_GL_ALPHA_SIZE = 3,
    SDL_GL_BUFFER_SIZE = 4,
    SDL_GL_DOUBLEBUFFER = 5,
    SDL_GL_DEPTH_SIZE = 6,
    SDL_GL_STENCIL_SIZE = 7,
    SDL_GL_ACCUM_RED_SIZE = 8,
    SDL_GL_ACCUM_GREEN_SIZE = 9,
    SDL_GL_ACCUM_BLUE_SIZE = 10,
    SDL_GL_ACCUM_ALPHA_SIZE = 11,
    SDL_GL_STEREO = 12,
    SDL_GL_MULTISAMPLEBUFFERS = 13,
    SDL_GL_MULTISAMPLESAMPLES = 14,
    SDL_GL_ACCELERATED_VISUAL = 15,
    SDL_GL_RETAINED_BACKING = 16,
    SDL_GL_CONTEXT_MAJOR_VERSION = 17,
    SDL_GL_CONTEXT_MINOR_VERSION = 18,
    SDL_GL_CONTEXT_EGL = 19,
    SDL_GL_CONTEXT_FLAGS = 20,
    SDL_GL_CONTEXT_PROFILE_MASK = 21,
    SDL_GL_SHARE_WITH_CURRENT_CONTEXT = 22,
    SDL_GL_FRAMEBUFFER_SRGB_CAPABLE = 23
}

#[derive(Copy, Clone)]
pub enum SDL_GLprofile {
    SDL_GL_CONTEXT_PROFILE_CORE = 0x0001,
    SDL_GL_CONTEXT_PROFILE_COMPATIBILITY = 0x0002,
    SDL_GL_CONTEXT_PROFILE_ES = 0x0004
}

//SDL_video.h
extern "C" {
    pub fn SDL_GetNumVideoDrivers() -> c_int;
    pub fn SDL_GetVideoDriver(index: c_int) -> *const c_char;
    pub fn SDL_VideoInit(driver_name: *const c_char) -> c_int;
    pub fn SDL_VideoQuit();
    pub fn SDL_GetCurrentVideoDriver() -> *const c_char;
    pub fn SDL_GetNumVideoDisplays() -> c_int;
    pub fn SDL_GetDisplayName(displayIndex: c_int) -> *const c_char;
    pub fn SDL_GetDisplayBounds(displayIndex: c_int, rect: *const SDL_Rect) -> c_int;
    pub fn SDL_GetNumDisplayModes(displayIndex: c_int) -> c_int;
    pub fn SDL_GetDisplayMode(displayIndex: c_int, modeIndex: c_int, mode: *const SDL_DisplayMode) -> c_int;
    pub fn SDL_GetDesktopDisplayMode(displayIndex: c_int, mode: *const SDL_DisplayMode) -> c_int;
    pub fn SDL_GetCurrentDisplayMode(displayIndex: c_int, mode: *const SDL_DisplayMode) -> c_int;
    pub fn SDL_GetClosestDisplayMode(displayIndex: c_int, mode: *const SDL_DisplayMode, closest: *const SDL_DisplayMode) -> *const SDL_DisplayMode;
    pub fn SDL_GetWindowDisplayIndex(window: *const SDL_Window) -> c_int;
    pub fn SDL_SetWindowDisplayMode(window: *const SDL_Window, mode: *const SDL_DisplayMode) -> c_int;
    pub fn SDL_GetWindowDisplayMode(window: *const SDL_Window, mode: *const SDL_DisplayMode) -> c_int;
    pub fn SDL_GetWindowPixelFormat(window: *const SDL_Window) -> uint32_t;
    pub fn SDL_CreateWindow(title: *const c_char, x: c_int, y: c_int, w: c_int, h: c_int, flags: uint32_t) -> *const SDL_Window;
    pub fn SDL_CreateWindowFrom(data: *const c_void) -> *const SDL_Window;
    pub fn SDL_GetWindowID(window: *const SDL_Window) -> uint32_t;
    pub fn SDL_GetWindowFromID(id: uint32_t) -> *const SDL_Window;
    pub fn SDL_GetWindowFlags(window: *const SDL_Window) -> uint32_t;
    pub fn SDL_SetWindowTitle(window: *const SDL_Window, title: *const c_char);
    pub fn SDL_GetWindowTitle(window: *const SDL_Window) -> *const c_char;
    pub fn SDL_SetWindowIcon(window: *const SDL_Window, icon: *const SDL_Surface);
    pub fn SDL_SetWindowData(window: *const SDL_Window, name: *const c_char, userdata: *const c_void) -> *const c_void;
    pub fn SDL_GetWindowData(window: *const SDL_Window, name: *const c_char) -> *const c_void;
    pub fn SDL_SetWindowPosition(window: *const SDL_Window, x: c_int, y: c_int);
    pub fn SDL_GetWindowPosition(window: *const SDL_Window, x: *const c_int, y: *const c_int);
    pub fn SDL_SetWindowSize(window: *const SDL_Window, w: c_int, h: c_int);
    pub fn SDL_GetWindowSize(window: *const SDL_Window, w: *const c_int, h: *const c_int);
    pub fn SDL_SetWindowMinimumSize(window: *const SDL_Window, min_w: c_int, min_h: c_int);
    pub fn SDL_GetWindowMinimumSize(window: *const SDL_Window, w: *const c_int, h: *const c_int);
    pub fn SDL_SetWindowMaximumSize(window: *const SDL_Window, max_w: c_int, max_h: c_int);
    pub fn SDL_GetWindowMaximumSize(window: *const SDL_Window, w: *const c_int, h: *const c_int);
    pub fn SDL_SetWindowBordered(window: *const SDL_Window, bordered: SDL_bool);
    pub fn SDL_ShowWindow(window: *const SDL_Window);
    pub fn SDL_HideWindow(window: *const SDL_Window);
    pub fn SDL_RaiseWindow(window: *const SDL_Window);
    pub fn SDL_MaximizeWindow(window: *const SDL_Window);
    pub fn SDL_MinimizeWindow(window: *const SDL_Window);
    pub fn SDL_RestoreWindow(window: *const SDL_Window);
    pub fn SDL_SetWindowFullscreen(window: *const SDL_Window, flags: uint32_t) -> c_int;
    pub fn SDL_GetWindowSurface(window: *const SDL_Window) -> *const SDL_Surface;
    pub fn SDL_UpdateWindowSurface(window: *const SDL_Window) -> c_int;
    pub fn SDL_UpdateWindowSurfaceRects(window: *const SDL_Window, rects: *const SDL_Rect, numrects: c_int) -> c_int;
    pub fn SDL_SetWindowGrab(window: *const SDL_Window, grabbed: SDL_bool);
    pub fn SDL_GetWindowGrab(window: *const SDL_Window) -> SDL_bool;
    pub fn SDL_SetWindowBrightness(window: *const SDL_Window, brightness: c_float) -> c_int;
    pub fn SDL_GetWindowBrightness(window: *const SDL_Window) -> c_float;
    pub fn SDL_SetWindowGammaRamp(window: *const SDL_Window, red: *const uint16_t, green: *const uint16_t, blue: *const uint16_t) -> c_int;
    pub fn SDL_GetWindowGammaRamp(window: *const SDL_Window, red: *const uint16_t, green: *const uint16_t, blue: *const uint16_t) -> c_int;
    pub fn SDL_DestroyWindow(window: *const SDL_Window);
    pub fn SDL_IsScreenSaverEnabled() -> SDL_bool;
    pub fn SDL_EnableScreenSaver();
    pub fn SDL_DisableScreenSaver();
    pub fn SDL_GL_GetDrawableSize(window: *const SDL_Window, w: *const c_int, h: *const c_int);
    pub fn SDL_GL_LoadLibrary(path: *const c_char) -> c_int;
    pub fn SDL_GL_GetProcAddress(procname: *const c_char) -> Option<extern "system" fn()>;
    pub fn SDL_GL_UnloadLibrary();
    pub fn SDL_GL_ExtensionSupported(extension: *const c_char) -> SDL_bool;
    pub fn SDL_GL_SetAttribute(attr: SDL_GLattr, value: c_int) -> c_int;
    pub fn SDL_GL_GetAttribute(attr: SDL_GLattr, value: *const c_int) -> c_int;
    pub fn SDL_GL_CreateContext(window: *const SDL_Window) -> SDL_GLContext;
    pub fn SDL_GL_MakeCurrent(window: *const SDL_Window, context: SDL_GLContext) -> c_int;
    pub fn SDL_GL_GetCurrentWindow() -> *const SDL_Window;
    pub fn SDL_GL_GetCurrentContext() -> SDL_GLContext;
    pub fn SDL_GL_SetSwapInterval(interval: c_int) -> c_int;
    pub fn SDL_GL_GetSwapInterval() -> c_int;
    pub fn SDL_GL_SwapWindow(window: *const SDL_Window);
    pub fn SDL_GL_DeleteContext(context: SDL_GLContext);
}
