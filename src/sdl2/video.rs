use libc::{c_int, c_float, uint32_t};
use std::ffi::{c_str_to_bytes, CString};
use std::ptr;
use std::vec::Vec;

use rect::Rect;
use surface::Surface;
use pixels;
use SdlResult;
use std::num::FromPrimitive;

use get_error;

pub use sys::video as ll;

#[derive(Copy, Clone, PartialEq)]
pub enum GLAttr {
    GLRedSize = 0,
    GLGreenSize = 1,
    GLBlueSize = 2,
    GLAlphaSize = 3,
    GLBufferSize = 4,
    GLDoubleBuffer = 5,
    GLDepthSize = 6,
    GLStencilSize = 7,
    GLAccumRedSize = 8,
    GLAccumGreenSize = 9,
    GLAccumBlueSize = 10,
    GLAccumAlphaSize = 11,
    GLStereo = 12,
    GLMultiSampleBuffers = 13,
    GLMultiSampleSamples = 14,
    GLAcceleratedVisual = 15,
    GLRetailedBacking = 16,
    GLContextMajorVersion = 17,
    GLContextMinorVersion = 18,
    GLContextEGL = 19,
    GLContextFlags = 20,
    GLContextProfileMask = 21,
    GLShareWithCurrentContext = 22,
    GLFramebufferSRGBCapable = 23,
}

#[derive(Copy, Clone, PartialEq)]
pub enum GLProfile {
  GLCoreProfile = 0x0001,
  GLCompatibilityProfile = 0x0002,
  GLESProfile = 0x0004,
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
#[derive(Clone, PartialEq)]
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

#[derive(Copy, Clone, PartialEq)]
pub enum FullscreenType {
    FTOff = 0,
    FTTrue = 0x00000001,
    FTDesktop = 0x00001001,
}

#[derive(PartialEq, Copy)]
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

#[derive(PartialEq)]
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

#[derive(PartialEq)]
#[allow(raw_pointer_derive)]
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
			let buff = CString::from_slice(title.as_bytes()).as_ptr();
            let raw = ll::SDL_CreateWindow(
                    buff,
                    unwrap_windowpos(x),
                    unwrap_windowpos(y),
                    width as c_int,
                    height as c_int,
                    window_flags.bits()
			);

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
		let buff = CString::from_slice(title.as_bytes()).as_ptr();
		unsafe { ll::SDL_SetWindowTitle(self.raw, buff) }
    }

    pub fn get_title(&self) -> String {
        unsafe {
            let buf = ll::SDL_GetWindowTitle(self.raw);
            String::from_utf8_lossy(c_str_to_bytes(&buf)).to_string()
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

    pub fn set_gamma_ramp(&self, red: Option<&[u16; 256]>, green: Option<&[u16; 256]>, blue: Option<&[u16; 256]>) -> bool {
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
        let buf = ll::SDL_GetVideoDriver(id as c_int);
        String::from_utf8_lossy(c_str_to_bytes(&buf)).to_string()
    }
}

pub fn video_init(name: &str) -> bool {
	let buf = CString::from_slice(name.as_bytes()).as_ptr();
	unsafe { ll::SDL_VideoInit(buf) == 0 }
}

pub fn video_quit() {
    unsafe { ll::SDL_VideoQuit() }
}

pub fn get_current_video_driver() -> String {
    unsafe {
        let video = ll::SDL_GetCurrentVideoDriver();
        String::from_utf8_lossy(c_str_to_bytes(&video)).to_string()
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
        let display = ll::SDL_GetDisplayName(display_index as c_int);
        String::from_utf8_lossy(c_str_to_bytes(&display)).to_string()
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
		let path = CString::from_slice(path.as_bytes()).as_ptr();

		if ll::SDL_GL_LoadLibrary(path) == 0 {
			Ok(())
		} else {
			Err(get_error())
		}
    }
}

pub fn gl_unload_library() {
    unsafe { ll::SDL_GL_UnloadLibrary(); }
}

pub fn gl_get_proc_address(procname: &str) -> Option<extern "system" fn()> {
    unsafe {
		let procname = CString::from_slice(procname.as_bytes()).as_ptr();
		ll::SDL_GL_GetProcAddress(procname)
    }
}

pub fn gl_extension_supported(extension: &str) -> bool {
	let buff = CString::from_slice(extension.as_bytes()).as_ptr();
	unsafe { ll::SDL_GL_ExtensionSupported(buff) == 1 }
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
