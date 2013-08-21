use std::libc::{c_int, uint32_t};
use std::ptr;
use std::str;
use std::cast;

use rect::Rect;

use get_error;

pub mod ll {
    use rect::Rect;
    use surface::ll::SDL_Surface;

    use std::libc::{c_void, c_int, c_float, c_char, uint16_t, uint32_t};

    pub type SDL_Rect = Rect;
    pub type SDL_bool = c_int;

    //SDL_video.h
    pub struct SDL_Window;

    pub struct SDL_DisplayMode
    {
        format: uint32_t,
        w: c_int,
        h: c_int,
        refresh_rate: c_int,
        driverdata: *c_void
    }

    pub type SDL_WindowPos = c_int;
    pub static SDL_WINDOWPOS_CENTERED: SDL_WindowPos = 0x2FFF0000;
    pub static SDL_WINDOWPOS_UNDEFINED: SDL_WindowPos = 0x1FFF0000;

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
        SDL_WINDOW_FOREIGN = 0x00000800
    }

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
    
    pub type SDL_GLContext = *c_void;

    pub enum SDL_GLattr {
        SDL_GL_RED_SIZE,
        SDL_GL_GREEN_SIZE,
        SDL_GL_BLUE_SIZE,
        SDL_GL_ALPHA_SIZE,
        SDL_GL_BUFFER_SIZE,
        SDL_GL_DOUBLEBUFFER,
        SDL_GL_DEPTH_SIZE,
        SDL_GL_STENCIL_SIZE,
        SDL_GL_ACCUM_RED_SIZE,
        SDL_GL_ACCUM_GREEN_SIZE,
        SDL_GL_ACCUM_BLUE_SIZE,
        SDL_GL_ACCUM_ALPHA_SIZE,
        SDL_GL_STEREO,
        SDL_GL_MULTISAMPLEBUFFERS,
        SDL_GL_MULTISAMPLESAMPLES,
        SDL_GL_ACCELERATED_VISUAL,
        SDL_GL_RETAINED_BACKING,
        SDL_GL_CONTEXT_MAJOR_VERSION,
        SDL_GL_CONTEXT_MINOR_VERSION,
        SDL_GL_CONTEXT_EGL,
        SDL_GL_CONTEXT_FLAGS,
        SDL_GL_CONTEXT_PROFILE_MASK,
        SDL_GL_SHARE_WITH_CURRENT_CONTEXT
    }

    pub enum SDL_GLprofile {
        SDL_GL_CONTEXT_PROFILE_CORE = 0x0001,
        SDL_GL_CONTEXT_PROFILE_COMPATIBILITY = 0x0002,
        SDL_GL_CONTEXT_PROFILE_ES = 0x0004
    }

    //SDL_video.h
    externfn!(fn SDL_GetNumVideoDrivers() -> c_int)
    externfn!(fn SDL_GetVideoDriver(index: c_int) -> *c_char)
    externfn!(fn SDL_VideoInit(driver_name: *c_char) -> c_int)
    externfn!(fn SDL_VideoQuit())
    externfn!(fn SDL_GetCurrentVideoDriver() -> *c_char)
    externfn!(fn SDL_GetNumVideoDisplays() -> c_int)
    externfn!(fn SDL_GetDisplayName(displayIndex: c_int) -> *c_char)
    externfn!(fn SDL_GetDisplayBounds(displayIndex: c_int, rect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_GetNumDisplayModes(displayIndex: c_int) -> c_int)
    externfn!(fn SDL_GetDisplayMode(displayIndex: c_int, modeIndex: c_int, mode: *SDL_DisplayMode) -> c_int)
    externfn!(fn SDL_GetDesktopDisplayMode(displayIndex: c_int, mode: *SDL_DisplayMode) -> c_int)
    externfn!(fn SDL_GetCurrentDisplayMode(displayIndex: c_int, mode: *SDL_DisplayMode) -> c_int)
    externfn!(fn SDL_GetClosestDisplayMode(displayIndex: c_int, mode: *SDL_DisplayMode, closest: *SDL_DisplayMode) -> *SDL_DisplayMode)
    externfn!(fn SDL_GetWindowDisplayIndex(window: *SDL_Window) -> c_int)
    externfn!(fn SDL_SetWindowDisplayMode(window: *SDL_Window, mode: *SDL_DisplayMode) -> c_int)
    externfn!(fn SDL_GetWindowDisplayMode(window: *SDL_Window, mode: *SDL_DisplayMode) -> c_int)
    externfn!(fn SDL_GetWindowPixelFormat(window: *SDL_Window) -> uint32_t)
    externfn!(fn SDL_CreateWindow(title: *c_char, x: c_int, y: c_int, w: c_int, h: c_int, flags: uint32_t) -> *SDL_Window)
    externfn!(fn SDL_CreateWindowFrom(data: *c_void) -> *SDL_Window)
    externfn!(fn SDL_GetWindowID(window: *SDL_Window) -> uint32_t)
    externfn!(fn SDL_GetWindowFromID(id: uint32_t) -> *SDL_Window)
    externfn!(fn SDL_GetWindowFlags(window: *SDL_Window) -> uint32_t)
    externfn!(fn SDL_SetWindowTitle(window: *SDL_Window, title: *c_char))
    externfn!(fn SDL_GetWindowTitle(window: *SDL_Window) -> *c_char)
    externfn!(fn SDL_SetWindowIcon(window: *SDL_Window, icon: *SDL_Surface))
    externfn!(fn SDL_SetWindowData(window: *SDL_Window, name: *c_char, userdata: *c_void) -> *c_void)
    externfn!(fn SDL_GetWindowData(window: *SDL_Window, name: *c_char) -> *c_void)
    externfn!(fn SDL_SetWindowPosition(window: *SDL_Window, x: c_int, y: c_int))
    externfn!(fn SDL_GetWindowPosition(window: *SDL_Window, x: *c_int, y: *c_int))
    externfn!(fn SDL_SetWindowSize(window: *SDL_Window, w: c_int, h: c_int))
    externfn!(fn SDL_GetWindowSize(window: *SDL_Window, w: *c_int, h: *c_int))
    externfn!(fn SDL_SetWindowMinimumSize(window: *SDL_Window, min_w: c_int, min_h: c_int))
    externfn!(fn SDL_GetWindowMinimumSize(window: *SDL_Window, w: *c_int, h: *c_int))
    externfn!(fn SDL_SetWindowMaximumSize(window: *SDL_Window, max_w: c_int, max_h: c_int))
    externfn!(fn SDL_GetWindowMaximumSize(window: *SDL_Window, w: *c_int, h: *c_int))
    externfn!(fn SDL_SetWindowBordered(window: *SDL_Window, bordered: SDL_bool))
    externfn!(fn SDL_ShowWindow(window: *SDL_Window))
    externfn!(fn SDL_HideWindow(window: *SDL_Window))
    externfn!(fn SDL_RaiseWindow(window: *SDL_Window))
    externfn!(fn SDL_MaximizeWindow(window: *SDL_Window))
    externfn!(fn SDL_MinimizeWindow(window: *SDL_Window))
    externfn!(fn SDL_RestoreWindow(window: *SDL_Window))
    externfn!(fn SDL_SetWindowFullscreen(window: *SDL_Window, flags: uint32_t) -> c_int)
    externfn!(fn SDL_GetWindowSurface(window: *SDL_Window) -> *SDL_Surface)
    externfn!(fn SDL_UpdateWindowSurface(window: *SDL_Window) -> c_int)
    externfn!(fn SDL_UpdateWindowSurfaceRects(window: *SDL_Window, rects: *SDL_Rect, numrects: c_int) -> c_int)
    externfn!(fn SDL_SetWindowGrab(window: *SDL_Window, grabbed: SDL_bool))
    externfn!(fn SDL_GetWindowGrab(window: *SDL_Window) -> SDL_bool)
    externfn!(fn SDL_SetWindowBrightness(window: *SDL_Window, brightness: float) -> c_int)
    externfn!(fn SDL_GetWindowBrightness(window: *SDL_Window) -> c_float)
    externfn!(fn SDL_SetWindowGammaRamp(window: *SDL_Window, red: *uint16_t, green: *uint16_t, blue: *uint16_t) -> c_int)
    externfn!(fn SDL_GetWindowGammaRamp(window: *SDL_Window, red: *uint16_t, green: *uint16_t, blue: *uint16_t) -> c_int)
    externfn!(fn SDL_DestroyWindow(window: *SDL_Window))
    externfn!(fn SDL_IsScreenSaverEnabled() -> SDL_bool)
    externfn!(fn SDL_EnableScreenSaver())
    externfn!(fn SDL_DisableScreenSaver())
    externfn!(fn SDL_GL_LoadLibrary(path: *c_char) -> c_int)
    externfn!(fn SDL_GL_GetProcAddress(proc: *c_char))
    externfn!(fn SDL_GL_UnloadLibrary())
    externfn!(fn SDL_GL_ExtensionSupported(extension: *c_char) -> SDL_bool)
    externfn!(fn SDL_GL_SetAttribute(attr: SDL_GLattr, value: c_int) -> c_int)
    externfn!(fn SDL_GL_GetAttribute(attr: SDL_GLattr, value: *c_int) -> c_int)
    externfn!(fn SDL_GL_CreateContext(window: *SDL_Window) -> SDL_GLContext)
    externfn!(fn SDL_GL_MakeCurrent(window: *SDL_Window, context: SDL_GLContext) -> c_int)
    externfn!(fn SDL_GL_GetCurrentWindow() -> *SDL_Window)
    externfn!(fn SDL_GL_GetCurrentContext() -> SDL_GLContext)
    externfn!(fn SDL_GL_SetSwapInterval(interval: c_int) -> c_int)
    externfn!(fn SDL_GL_GetSwapInterval() -> c_int)
    externfn!(fn SDL_GL_SwapWindow(window: *SDL_Window))
    externfn!(fn SDL_GL_DeleteContext(context: SDL_GLContext))
}

fn empty_sdl_display_mode() -> ll::SDL_DisplayMode {
    ll::SDL_DisplayMode {
        format: 0,
        w: 0,
        h: 0,
        refresh_rate: 0,
        driverdata: ptr::null()
    }
}

#[deriving(Eq)]
pub struct DisplayMode {
    format: u32,
    w: int,
    h: int,
    refresh_rate: int
}

impl DisplayMode {

    pub fn new(format: u32, w: int, h: int, refresh_rate: int) -> DisplayMode {
        DisplayMode {
            format: format,
            w: w,
            h: h,
            refresh_rate: refresh_rate
        }
    }

    pub fn from_ll(raw: &ll::SDL_DisplayMode) -> DisplayMode {
        DisplayMode::new(
            raw.format as u32,
            raw.w as int,
            raw.h as int,
            raw.refresh_rate as int
        )
    }

    pub fn to_ll(&self) -> ll::SDL_DisplayMode {
        ll::SDL_DisplayMode {
            format: self.format as uint32_t,
            w: self.w as c_int,
            h: self.h as c_int,
            refresh_rate: self.refresh_rate as c_int,
            driverdata: ptr::null()
        }
    }
}

#[deriving(Eq)]
pub enum WindowFlags {
    Fullscreen = ll::SDL_WINDOW_FULLSCREEN as int,
    OpenGL = ll::SDL_WINDOW_OPENGL as int,
    Shown = ll::SDL_WINDOW_SHOWN as int,
    Hidden = ll::SDL_WINDOW_HIDDEN as int,
    Borderless = ll::SDL_WINDOW_BORDERLESS as int,
    Resizable = ll::SDL_WINDOW_RESIZABLE as int,
    Minimized = ll::SDL_WINDOW_MINIMIZED as int,
    Maximized = ll::SDL_WINDOW_MAXIMIZED as int,
    InputGrabbed = ll::SDL_WINDOW_INPUT_GRABBED as int,
    InputFocus = ll::SDL_WINDOW_INPUT_FOCUS as int,
    MouseFocus = ll::SDL_WINDOW_MOUSE_FOCUS as int,
    FullscreenDesktop = ll::SDL_WINDOW_FULLSCREEN_DESKTOP as int,
    Foreign = ll::SDL_WINDOW_FOREIGN as int
}

#[deriving(Eq)]
pub enum WindowPos {
    PosUndefined,
    PosCentered,
    Positioned(int)
}

fn unwrap_windowpos (pos: WindowPos) -> ll::SDL_WindowPos {
    match pos {
        PosUndefined => ll::SDL_WINDOWPOS_UNDEFINED,
        PosCentered => ll::SDL_WINDOWPOS_CENTERED, 
        Positioned(x) => x as ll::SDL_WindowPos
    }
}


#[deriving(Eq)]
pub struct Window {
    raw: *ll::SDL_Window,
    owned: bool
}

impl Drop for Window {
    fn drop(&self) {
        if self.owned {
            unsafe {
                ll::SDL_DestroyWindow(self.raw);
            }
        }
    }
}

impl Window {
    pub fn new(title: &str, x: WindowPos, y: WindowPos, width: int, height: int, window_flags: &[WindowFlags]) -> Result<~Window, ~str> {
        let flags = window_flags.iter().fold(0u32, |flags, flag| { flags | *flag as u32 });

        unsafe {
            let raw = do title.with_c_str |buff| {
                ll::SDL_CreateWindow(
                    buff,
                    unwrap_windowpos(x),
                    unwrap_windowpos(y),
                    width as c_int,
                    height as c_int,
                    flags
                )
            };

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Window{ raw: raw, owned: true })
            }
        }
    }

    pub fn get_display_index(&self) -> Result<int, ~str> {
        let result = unsafe { ll::SDL_GetWindowDisplayIndex(self.raw) };
        if result < 0 {
            return Err(get_error())
        } else {
            Ok(result as int)
        }
    }

    pub fn set_display_mode(&self, display_mode: Option<DisplayMode>) -> bool {
        return unsafe { 
            ll::SDL_SetWindowDisplayMode(
                self.raw,
                match display_mode {
                    Some(ref mode) => cast::transmute(&mode.to_ll()),
                    None => ptr::null() 
                }
            ) == 0 
        }
    }

    pub fn get_display_mode(&self, display_mode: &DisplayMode) -> Result<~DisplayMode, ~str> {
        let dm = empty_sdl_display_mode();

        let result = unsafe { 
            ll::SDL_GetWindowDisplayMode(
                self.raw,
                &display_mode.to_ll()
            ) == 0 
        };

        if result {
            Ok(~DisplayMode::from_ll(&dm))
        } else {
            Err(get_error())
        }
    }

    /*pub fn SDL_GetWindowPixelFormat(window: *SDL_Window) -> uint32_t;
    pub fn SDL_CreateWindow(title: *c_char, x: c_int, y: c_int, w: c_int, h: c_int, flags: uint32_t) -> *SDL_Window;
    pub fn SDL_CreateWindowFrom(data: *c_void) -> *SDL_Window;
    pub fn SDL_GetWindowID(window: *SDL_Window) -> uint32_t;
    pub fn SDL_GetWindowFromID(id: uint32_t) -> *SDL_Window;
    pub fn SDL_GetWindowFlags(window: *SDL_Window) -> uint32_t;
    pub fn SDL_SetWindowTitle(window: *SDL_Window, title: *c_char);
    pub fn SDL_GetWindowTitle(window: *SDL_Window) -> *c_char;
    pub fn SDL_SetWindowIcon(window: *SDL_Window, icon: *SDL_Surface);
    pub fn SDL_SetWindowData(window: *SDL_Window, name: *c_char, userdata: *c_void) -> *c_void;
    pub fn SDL_GetWindowData(window: *SDL_Window, name: *c_char) -> *c_void;
    pub fn SDL_SetWindowPosition(window: *SDL_Window, x: c_int, y: c_int);
    pub fn SDL_GetWindowPosition(window: *SDL_Window, x: *c_int, y: *c_int);
    pub fn SDL_SetWindowSize(window: *SDL_Window, w: c_int, h: c_int);
    pub fn SDL_GetWindowSize(window: *SDL_Window, w: *c_int, h: *c_int);
    pub fn SDL_SetWindowMinimumSize(window: *SDL_Window, min_w: c_int, min_h: c_int);
    pub fn SDL_GetWindowMinimumSize(window: *SDL_Window, w: *c_int, h: *c_int);
    pub fn SDL_SetWindowMaximumSize(window: *SDL_Window, max_w: c_int, max_h: c_int);
    pub fn SDL_GetWindowMaximumSize(window: *SDL_Window, w: *c_int, h: *c_int);
    pub fn SDL_SetWindowBordered(window: *SDL_Window, bordered: SDL_bool);
    pub fn SDL_ShowWindow(window: *SDL_Window);
    pub fn SDL_HideWindow(window: *SDL_Window);
    pub fn SDL_RaiseWindow(window: *SDL_Window);
    pub fn SDL_MaximizeWindow(window: *SDL_Window);
    pub fn SDL_MinimizeWindow(window: *SDL_Window);
    pub fn SDL_RestoreWindow(window: *SDL_Window);
    pub fn SDL_SetWindowFullscreen(window: *SDL_Window, flags: uint32_t) -> c_int;
    pub fn SDL_GetWindowSurface(window: *SDL_Window) -> *SDL_Surface;
    pub fn SDL_UpdateWindowSurface(window: *SDL_Window) -> c_int;
    pub fn SDL_UpdateWindowSurfaceRects(window: *SDL_Window, rects: *SDL_Rect, numrects: c_int) -> c_int;
    pub fn SDL_SetWindowGrab(window: *SDL_Window, grabbed: SDL_bool);
    pub fn SDL_GetWindowGrab(window: *SDL_Window) -> SDL_bool;
    pub fn SDL_SetWindowBrightness(window: *SDL_Window, brightness: float) -> c_int;
    pub fn SDL_GetWindowBrightness(window: *SDL_Window) -> c_float;
    pub fn SDL_SetWindowGammaRamp(window: *SDL_Window, red: *uint16_t, green: *uint16_t, blue: *uint16_t) -> c_int;
    pub fn SDL_GetWindowGammaRamp(window: *SDL_Window, red: *uint16_t, green: *uint16_t, blue: *uint16_t) -> c_int;
    pub fn SDL_DestroyWindow(window: *SDL_Window);*/
}

pub fn get_num_video_drivers() -> Result<int, ~str> {
    let result = unsafe { ll::SDL_GetNumVideoDrivers() };
    if result < 0 {
        Err(get_error())
    } else {
        Ok(result as int)
    }
}

pub fn get_video_driver(id: int) -> ~str {
    unsafe {
        let cstr = ll::SDL_GetVideoDriver(id as c_int);
        str::raw::from_c_str(cast::transmute_copy(&cstr))
    }
}

pub fn video_init(name: &str) -> bool {
    do name.with_c_str |buf| {
        unsafe { ll::SDL_VideoInit(buf) == 0 }
    }
}

pub fn video_quit() {
    unsafe { ll::SDL_VideoQuit() }
}

pub fn get_current_video_driver() -> ~str {
    unsafe {
        let cstr = ll::SDL_GetCurrentVideoDriver();
        str::raw::from_c_str(cast::transmute_copy(&cstr))
    }
}

pub fn get_num_video_displays() -> Result<int, ~str> {
    let result = unsafe { ll::SDL_GetNumVideoDisplays() };
    if result < 0 {
        Err(get_error())
    } else {
        Ok(result as int)
    }
}

pub fn get_display_name(display_index: int) -> ~str {
    unsafe {
        let cstr = ll::SDL_GetDisplayName(display_index as c_int);
        str::raw::from_c_str(cast::transmute_copy(&cstr))
    }
}

pub fn get_display_bounds(display_index: int) -> Result<Rect, ~str> {
    let out: Rect = Rect::new(0, 0, 0, 0);
    let result = unsafe { ll::SDL_GetDisplayBounds(display_index as c_int, &out) == 0 };

    if result {
        Ok(out)
    } else {
        Err(get_error())
    }
}

pub fn get_num_display_modes(display_index: int) -> Result<int, ~str> {
    let result = unsafe { ll::SDL_GetNumDisplayModes(display_index as c_int) };
    if result < 0 {
        Err(get_error())
    } else {
        Ok(result as int)
    }
}

pub fn get_display_mode(display_index: int, mode_index: int) -> Result<~DisplayMode, ~str> {
    let dm = empty_sdl_display_mode();
    let result = unsafe { ll::SDL_GetDisplayMode(display_index as c_int, mode_index as c_int, &dm) == 0};

    if result {
        Ok(~DisplayMode::from_ll(&dm))
    } else {
        Err(get_error())
    }
}

pub fn get_desktop_display_mode(display_index: int) -> Result<~DisplayMode, ~str> {
    let dm = empty_sdl_display_mode();
    let result = unsafe { ll::SDL_GetDesktopDisplayMode(display_index as c_int, &dm) == 0};

    if result {
        Ok(~DisplayMode::from_ll(&dm))
    } else {
        Err(get_error())
    }
}

pub fn get_current_display_mode(display_index: int) -> Result<~DisplayMode, ~str> {
    let dm = empty_sdl_display_mode();
    let result = unsafe { ll::SDL_GetCurrentDisplayMode(display_index as c_int, &dm) == 0};

    if result {
        Ok(~DisplayMode::from_ll(&dm))
    } else {
        Err(get_error())
    }
}

pub fn get_closest_display_mode(display_index: int, mode: &DisplayMode) -> Result<~DisplayMode, ~str> {
    let input = mode.to_ll();
    let out = empty_sdl_display_mode();

    let result = unsafe { ll::SDL_GetClosestDisplayMode(display_index as c_int, &input, &out) };

    if result == ptr::null() {
        Err(get_error())
    } else {
        Ok(~DisplayMode::from_ll(&out))
    }
}
