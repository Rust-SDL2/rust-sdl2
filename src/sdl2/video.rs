use libc::{c_int, c_float, uint32_t};
use std::ptr;
use std::vec::Vec;

use rect::Rect;
use surface::Surface;
use pixels;
use SdlResult;
use std::num::FromPrimitive;

use get_error;

#[allow(non_camel_case_types)]
pub mod ll {
    use rect::Rect;
    use surface::ll::SDL_Surface;

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

    #[deriving(Copy, Clone)]
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

    #[deriving(Copy, Clone)]
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

    #[deriving(Copy, Clone, FromPrimitive)]
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

    #[deriving(Copy, Clone)]
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
}

#[deriving(Copy, Clone, PartialEq)]
pub enum GLAttr {
    GLRedSize = ll::SDL_GLattr::SDL_GL_RED_SIZE as int,
    GLGreenSize = ll::SDL_GLattr::SDL_GL_GREEN_SIZE as int,
    GLBlueSize = ll::SDL_GLattr::SDL_GL_BLUE_SIZE as int,
    GLAlphaSize = ll::SDL_GLattr::SDL_GL_ALPHA_SIZE as int,
    GLBufferSize = ll::SDL_GLattr::SDL_GL_BUFFER_SIZE as int,
    GLDoubleBuffer = ll::SDL_GLattr::SDL_GL_DOUBLEBUFFER as int,
    GLDepthSize = ll::SDL_GLattr::SDL_GL_DEPTH_SIZE as int,
    GLStencilSize = ll::SDL_GLattr::SDL_GL_STENCIL_SIZE as int,
    GLAccumRedSize = ll::SDL_GLattr::SDL_GL_ACCUM_RED_SIZE as int,
    GLAccumGreenSize = ll::SDL_GLattr::SDL_GL_ACCUM_GREEN_SIZE as int,
    GLAccumBlueSize = ll::SDL_GLattr::SDL_GL_ACCUM_BLUE_SIZE as int,
    GLAccumAlphaSize = ll::SDL_GLattr::SDL_GL_ACCUM_ALPHA_SIZE as int,
    GLStereo = ll::SDL_GLattr::SDL_GL_STEREO as int,
    GLMultiSampleBuffers = ll::SDL_GLattr::SDL_GL_MULTISAMPLEBUFFERS as int,
    GLMultiSampleSamples = ll::SDL_GLattr::SDL_GL_MULTISAMPLESAMPLES as int,
    GLAcceleratedVisual = ll::SDL_GLattr::SDL_GL_ACCELERATED_VISUAL as int,
    GLRetailedBacking = ll::SDL_GLattr::SDL_GL_RETAINED_BACKING as int,
    GLContextMajorVersion = ll::SDL_GLattr::SDL_GL_CONTEXT_MAJOR_VERSION as int,
    GLContextMinorVersion = ll::SDL_GLattr::SDL_GL_CONTEXT_MINOR_VERSION as int,
    GLContextEGL = ll::SDL_GLattr::SDL_GL_CONTEXT_EGL as int,
    GLContextFlags = ll::SDL_GLattr::SDL_GL_CONTEXT_FLAGS as int,
    GLContextProfileMask = ll::SDL_GLattr::SDL_GL_CONTEXT_PROFILE_MASK as int,
    GLShareWithCurrentContext = ll::SDL_GLattr::SDL_GL_SHARE_WITH_CURRENT_CONTEXT as int,
    GLFramebufferSRGBCapable = ll::SDL_GLattr::SDL_GL_FRAMEBUFFER_SRGB_CAPABLE as int
}

#[deriving(Copy, Clone, PartialEq)]
pub enum GLProfile {
  GLCoreProfile = ll::SDL_GLprofile::SDL_GL_CONTEXT_PROFILE_CORE as int,
  GLCompatibilityProfile = ll::SDL_GLprofile::SDL_GL_CONTEXT_PROFILE_COMPATIBILITY as int,
  GLESProfile = ll::SDL_GLprofile::SDL_GL_CONTEXT_PROFILE_ES as int
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

#[allow(missing_copy_implementations)]
#[deriving(Clone, PartialEq)]
pub struct DisplayMode {
    pub format: u32,
    pub w: int,
    pub h: int,
    pub refresh_rate: int
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

bitflags! {
    flags WindowFlags: u32 {
        const FULLSCREEN = ll::SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as u32,
        const OPENGL = ll::SDL_WindowFlags::SDL_WINDOW_OPENGL as u32,
        const SHOWN = ll::SDL_WindowFlags::SDL_WINDOW_SHOWN as u32,
        const HIDDEN = ll::SDL_WindowFlags::SDL_WINDOW_HIDDEN as u32,
        const BORDERLESS = ll::SDL_WindowFlags::SDL_WINDOW_BORDERLESS as u32,
        const RESIZABLE = ll::SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32,
        const MINIMIZED = ll::SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32,
        const MAXIMIZED = ll::SDL_WindowFlags::SDL_WINDOW_MAXIMIZED as u32,
        const INPUT_GRABBED = ll::SDL_WindowFlags::SDL_WINDOW_INPUT_GRABBED as u32,
        const INPUT_FOCUS = ll::SDL_WindowFlags::SDL_WINDOW_INPUT_FOCUS as u32,
        const MOUSE_FOCUS = ll::SDL_WindowFlags::SDL_WINDOW_MOUSE_FOCUS as u32,
        const FULLSCREEN_DESKTOP = ll::SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as u32,
        const FOREIGN = ll::SDL_WindowFlags::SDL_WINDOW_FOREIGN as u32,
        const ALLOW_HIGHDPI = ll::SDL_WindowFlags::SDL_WINDOW_ALLOW_HIGHDPI as u32
    }
}

#[deriving(Copy, Clone, PartialEq)]
pub enum FullscreenType {
    FTOff = 0,
    FTTrue = ll::SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as int,
    FTDesktop = ll::SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as int
}

#[deriving(PartialEq, Copy)]
pub enum WindowPos {
    PosUndefined,
    PosCentered,
    Positioned(int)
}

fn unwrap_windowpos (pos: WindowPos) -> ll::SDL_WindowPos {
    match pos {
        WindowPos::PosUndefined => ll::SDL_WINDOWPOS_UNDEFINED,
        WindowPos::PosCentered => ll::SDL_WINDOWPOS_CENTERED,
        WindowPos::Positioned(x) => x as ll::SDL_WindowPos
    }
}

#[deriving(PartialEq)]
pub struct GLContext {
    raw: ll::SDL_GLContext,
    owned: bool
}

impl Drop for GLContext {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ll::SDL_GL_DeleteContext(self.raw)
            }
        }
    }
}

#[deriving(PartialEq)]
#[allow(raw_pointer_deriving)]
pub struct Window {
    raw: *const ll::SDL_Window,
    owned: bool
}

impl_raw_accessors!(
    GLContext, ll::SDL_GLContext;
    Window, *const ll::SDL_Window
);

impl_owned_accessors!(
    GLContext, owned;
    Window, owned
);

impl_raw_constructor!(
    Window -> Window (raw: *const ll::SDL_Window, owned: bool)
);

impl Drop for Window {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ll::SDL_DestroyWindow(self.raw);
            }
        }
    }
}

impl Window {
    pub fn new(title: &str, x: WindowPos, y: WindowPos, width: int, height: int, window_flags: WindowFlags) -> SdlResult<Window> {
        unsafe {
            let raw = title.with_c_str(|buff| {
                ll::SDL_CreateWindow(
                    buff,
                    unwrap_windowpos(x),
                    unwrap_windowpos(y),
                    width as c_int,
                    height as c_int,
                    window_flags.bits()
                )
            });

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(Window{ raw: raw, owned: true })
            }
        }
    }

    pub fn from_id(id: u32) -> SdlResult<Window> {
        let raw = unsafe { ll::SDL_GetWindowFromID(id) };
        if raw == ptr::null() {
            Err(get_error())
        } else {
            Ok(Window{ raw: raw, owned: false})
        }
    }

    pub fn get_display_index(&self) -> SdlResult<int> {
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
                    Some(ref mode) => &mode.to_ll() as *const _,
                    None => ptr::null()
                }
            ) == 0
        }
    }

    pub fn get_display_mode(&self, display_mode: &DisplayMode) -> SdlResult<DisplayMode> {
        let dm = empty_sdl_display_mode();

        let result = unsafe {
            ll::SDL_GetWindowDisplayMode(
                self.raw,
                &display_mode.to_ll()
            ) == 0
        };

        if result {
            Ok(DisplayMode::from_ll(&dm))
        } else {
            Err(get_error())
        }
    }

    pub fn get_window_pixel_format(&self) -> pixels::PixelFormatFlag {
        unsafe{ FromPrimitive::from_u64(ll::SDL_GetWindowPixelFormat(self.raw) as u64).unwrap() }
    }

    pub fn get_id(&self) -> u32 {
        unsafe { ll::SDL_GetWindowID(self.raw) }
    }

    pub fn get_flags(&self) -> WindowFlags {
        unsafe {
            let raw = ll::SDL_GetWindowFlags(self.raw);
            WindowFlags::from_bits(raw).unwrap()
        }
    }

    pub fn set_title(&self, title: &str) {
        title.with_c_str(|buff| {
            unsafe { ll::SDL_SetWindowTitle(self.raw, buff) }
        })
    }

    pub fn get_title(&self) -> String {
        unsafe {
            let cstr = ll::SDL_GetWindowTitle(self.raw);
            String::from_raw_buf(cstr as *const _)
        }
    }

    pub fn set_icon(&self, icon: &Surface) {
        unsafe { ll::SDL_SetWindowIcon(self.raw, icon.raw()) }
    }

    //pub fn SDL_SetWindowData(window: *SDL_Window, name: *c_char, userdata: *c_void) -> *c_void; //TODO: Figure out what this does
    //pub fn SDL_GetWindowData(window: *SDL_Window, name: *c_char) -> *c_void;

    pub fn set_position(&self, x: WindowPos, y: WindowPos) {
        unsafe { ll::SDL_SetWindowPosition(self.raw, unwrap_windowpos(x), unwrap_windowpos(y)) }
    }

    pub fn get_position(&self) -> (int, int) {
        let x: c_int = 0;
        let y: c_int = 0;
        unsafe { ll::SDL_GetWindowPosition(self.raw, &x, &y) };
        (x as int, y as int)
    }

    pub fn set_size(&self, w: int, h: int) {
        unsafe { ll::SDL_SetWindowSize(self.raw, w as c_int, h as c_int) }
    }

    pub fn get_size(&self) -> (int, int) {
        let w: c_int = 0;
        let h: c_int = 0;
        unsafe { ll::SDL_GetWindowSize(self.raw, &w, &h) };
        (w as int, h as int)
    }

    pub fn get_drawable_size(&self) -> (int, int) {
        let w: c_int = 0;
        let h: c_int = 0;
        unsafe { ll::SDL_GL_GetDrawableSize(self.raw, &w, &h) };
        (w as int, h as int)
    }

    pub fn set_minimum_size(&self, w: int, h: int) {
        unsafe { ll::SDL_SetWindowMinimumSize(self.raw, w as c_int, h as c_int) }
    }

    pub fn get_minimum_size(&self) -> (int, int) {
        let w: c_int = 0;
        let h: c_int = 0;
        unsafe { ll::SDL_GetWindowMinimumSize(self.raw, &w, &h) };
        (w as int, h as int)
    }

    pub fn set_maximum_size(&self, w: int, h: int) {
        unsafe { ll::SDL_SetWindowMaximumSize(self.raw, w as c_int, h as c_int) }
    }

    pub fn get_maximum_size(&self) -> (int, int) {
        let w: c_int = 0;
        let h: c_int = 0;
        unsafe { ll::SDL_GetWindowMaximumSize(self.raw, &w, &h) };
        (w as int, h as int)
    }

    pub fn set_bordered(&self, bordered: bool) {
        unsafe { ll::SDL_SetWindowBordered(self.raw, if bordered { 1 } else { 0 }) }
    }

    pub fn show(&self) {
        unsafe { ll::SDL_ShowWindow(self.raw) }
    }

    pub fn hide(&self) {
        unsafe { ll::SDL_HideWindow(self.raw) }
    }

    pub fn raise(&self) {
        unsafe { ll::SDL_RaiseWindow(self.raw) }
    }

    pub fn maximize(&self) {
        unsafe { ll::SDL_MaximizeWindow(self.raw) }
    }

    pub fn minimize(&self) {
        unsafe { ll::SDL_MinimizeWindow(self.raw) }
    }

    pub fn restore(&self) {
        unsafe { ll::SDL_RestoreWindow(self.raw) }
    }

    pub fn set_fullscreen(&self, fullscreen_type: FullscreenType) -> bool {
        unsafe { ll::SDL_SetWindowFullscreen(self.raw, fullscreen_type as uint32_t) == 0 }
    }

    pub fn get_surface(&self) -> SdlResult<Surface> {
        let raw = unsafe { ll::SDL_GetWindowSurface(self.raw) };

        if raw == ptr::null() {
            Err(get_error())
        } else {
            unsafe { Ok(Surface::from_ll(raw, false)) } //Docs say that it releases with the window
        }
    }

    pub fn update_surface(&self) -> bool {
        unsafe { ll::SDL_UpdateWindowSurface(self.raw) == 0 }
    }

    pub fn update_surface_rects(&self, rects: &[Rect]) -> bool {
        unsafe { ll::SDL_UpdateWindowSurfaceRects(self.raw, rects.as_ptr(), rects.len() as c_int) == 0}
    }

    pub fn set_grab(&self, grabbed: bool) {
        unsafe { ll::SDL_SetWindowGrab(self.raw, if grabbed { 1 } else { 0 }) }
    }

    pub fn get_grab(&self) -> bool {
        unsafe { ll::SDL_GetWindowGrab(self.raw) == 1 }
    }

    pub fn set_brightness(&self, brightness: f64) -> bool {
        unsafe { ll::SDL_SetWindowBrightness(self.raw, brightness as c_float) == 0 }
    }

    pub fn get_brightness(&self) -> f64 {
        unsafe { ll::SDL_GetWindowBrightness(self.raw) as f64 }
    }

    pub fn set_gamma_ramp(&self, red: Option<&[u16, ..256]>, green: Option<&[u16, ..256]>, blue: Option<&[u16, ..256]>) -> bool {
        unsafe {
            let unwrapped_red = match red {
                Some(values) => values.as_ptr(),
                None => ptr::null()
            };
            let unwrapped_green = match green {
                Some(values) => values.as_ptr(),
                None => ptr::null()
            };
            let unwrapped_blue = match blue {
                Some(values) => values.as_ptr(),
                None => ptr::null()
            };
            ll::SDL_SetWindowGammaRamp(self.raw, unwrapped_red, unwrapped_green, unwrapped_blue) == 0
        }
    }

    pub fn get_gamma_ramp(&self) -> SdlResult<(Vec<u16>, Vec<u16>, Vec<u16>)> {
        let red: Vec<u16> = Vec::with_capacity(256);
        let green: Vec<u16> = Vec::with_capacity(256);
        let blue: Vec<u16> = Vec::with_capacity(256);
        let result = unsafe {ll::SDL_GetWindowGammaRamp(self.raw, red.as_ptr(), green.as_ptr(), blue.as_ptr()) == 0};
        if result {
            Ok((red, green, blue))
        } else {
            Err(get_error())
        }
    }

    pub fn gl_create_context(&self) -> SdlResult<GLContext> {
        let result = unsafe { ll::SDL_GL_CreateContext(self.raw) };
        if result == ptr::null() {
            Err(get_error())
        } else {
            Ok(GLContext{raw: result, owned: true})
        }
    }

    pub fn gl_make_current(&self, context: &GLContext) -> bool {
        unsafe { ll::SDL_GL_MakeCurrent(self.raw, context.raw) == 0 }
    }

    pub fn gl_swap_window(&self) {
        unsafe { ll::SDL_GL_SwapWindow(self.raw) }
    }
}

pub fn get_num_video_drivers() -> SdlResult<int> {
    let result = unsafe { ll::SDL_GetNumVideoDrivers() };
    if result < 0 {
        Err(get_error())
    } else {
        Ok(result as int)
    }
}

pub fn get_video_driver(id: int) -> String {
    unsafe {
        let cstr = ll::SDL_GetVideoDriver(id as c_int);
        String::from_raw_buf(cstr as *const _)
    }
}

pub fn video_init(name: &str) -> bool {
    name.with_c_str(|buf| {
        unsafe { ll::SDL_VideoInit(buf) == 0 }
    })
}

pub fn video_quit() {
    unsafe { ll::SDL_VideoQuit() }
}

pub fn get_current_video_driver() -> String {
    unsafe {
        let cstr = ll::SDL_GetCurrentVideoDriver();
        String::from_raw_buf(cstr as *const _)
    }
}

pub fn get_num_video_displays() -> SdlResult<int> {
    let result = unsafe { ll::SDL_GetNumVideoDisplays() };
    if result < 0 {
        Err(get_error())
    } else {
        Ok(result as int)
    }
}

pub fn get_display_name(display_index: int) -> String {
    unsafe {
        let cstr = ll::SDL_GetDisplayName(display_index as c_int);
        String::from_raw_buf(cstr as *const _)
    }
}

pub fn get_display_bounds(display_index: int) -> SdlResult<Rect> {
    let out: Rect = Rect::new(0, 0, 0, 0);
    let result = unsafe { ll::SDL_GetDisplayBounds(display_index as c_int, &out) == 0 };

    if result {
        Ok(out)
    } else {
        Err(get_error())
    }
}

pub fn get_num_display_modes(display_index: int) -> SdlResult<int> {
    let result = unsafe { ll::SDL_GetNumDisplayModes(display_index as c_int) };
    if result < 0 {
        Err(get_error())
    } else {
        Ok(result as int)
    }
}

pub fn get_display_mode(display_index: int, mode_index: int) -> SdlResult<DisplayMode> {
    let dm = empty_sdl_display_mode();
    let result = unsafe { ll::SDL_GetDisplayMode(display_index as c_int, mode_index as c_int, &dm) == 0};

    if result {
        Ok(DisplayMode::from_ll(&dm))
    } else {
        Err(get_error())
    }
}

pub fn get_desktop_display_mode(display_index: int) -> SdlResult<DisplayMode> {
    let dm = empty_sdl_display_mode();
    let result = unsafe { ll::SDL_GetDesktopDisplayMode(display_index as c_int, &dm) == 0};

    if result {
        Ok(DisplayMode::from_ll(&dm))
    } else {
        Err(get_error())
    }
}

pub fn get_current_display_mode(display_index: int) -> SdlResult<DisplayMode> {
    let dm = empty_sdl_display_mode();
    let result = unsafe { ll::SDL_GetCurrentDisplayMode(display_index as c_int, &dm) == 0};

    if result {
        Ok(DisplayMode::from_ll(&dm))
    } else {
        Err(get_error())
    }
}

pub fn get_closest_display_mode(display_index: int, mode: &DisplayMode) -> SdlResult<DisplayMode> {
    let input = mode.to_ll();
    let out = empty_sdl_display_mode();

    let result = unsafe { ll::SDL_GetClosestDisplayMode(display_index as c_int, &input, &out) };

    if result == ptr::null() {
        Err(get_error())
    } else {
        Ok(DisplayMode::from_ll(&out))
    }
}

pub fn is_screen_saver_enabled() -> bool {
    unsafe { ll::SDL_IsScreenSaverEnabled() == 1 }
}

pub fn enable_screen_saver() {
    unsafe { ll::SDL_EnableScreenSaver() }
}

pub fn disable_screen_saver() {
    unsafe { ll::SDL_DisableScreenSaver() }
}

pub fn gl_load_library(path: &str) -> SdlResult<()> {
    unsafe {
        path.with_c_str(|path| {
            if ll::SDL_GL_LoadLibrary(path) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        })
    }
}

pub fn gl_unload_library() {
    unsafe { ll::SDL_GL_UnloadLibrary(); }
}

pub fn gl_get_proc_address(procname: &str) -> Option<extern "system" fn()> {
    unsafe {
        procname.with_c_str(|procname| {
            ll::SDL_GL_GetProcAddress(procname)
        })
    }
}

pub fn gl_extension_supported(extension: &str) -> bool {
    extension.with_c_str(|buff| {
        unsafe { ll::SDL_GL_ExtensionSupported(buff) == 1 }
    })
}

pub fn gl_set_attribute(attr: GLAttr, value: int) -> bool {
    unsafe { ll::SDL_GL_SetAttribute(FromPrimitive::from_u64(attr as u64).unwrap(), value as c_int) == 0 }
}

pub fn gl_get_attribute(attr: GLAttr) -> SdlResult<int> {
    let out: c_int = 0;

    let result = unsafe { ll::SDL_GL_GetAttribute(FromPrimitive::from_u64(attr as u64).unwrap(), &out) } == 0;
    if result {
        Ok(out as int)
    } else {
        Err(get_error())
    }
}

pub fn gl_get_current_window() -> SdlResult<Window> {
    let raw = unsafe { ll::SDL_GL_GetCurrentWindow() };
    if raw == ptr::null() {
        Err(get_error())
    } else {
        Ok(Window{ raw: raw, owned: false })
    }
}

pub fn gl_get_current_context() -> SdlResult<GLContext> {
    let raw = unsafe { ll::SDL_GL_GetCurrentContext() };
    if raw == ptr::null() {
        Err(get_error())
    } else {
        Ok(GLContext{ raw: raw, owned: false })
    }
}

pub fn gl_set_swap_interval(interval: int) -> bool {
    unsafe { ll::SDL_GL_SetSwapInterval(interval as c_int) == 0 }
}

pub fn gl_get_swap_interval() -> int {
    unsafe { ll::SDL_GL_GetSwapInterval() as int }
}
