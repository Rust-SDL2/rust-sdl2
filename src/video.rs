use std::cast;
use std::libc::{c_int, c_float};
use std::ptr;
use std::rand;
use std::rand::RngUtil;
use std::vec;

use Rect;
use get_error;

pub mod ll {
    use Rect;

    use std::libc::{c_void, c_uint, c_int, c_float, c_char, c_schar, c_uchar, uint8_t, uint16_t};
    use std::libc::{uint32_t, int32_t};

    pub type SDL_Rect = Rect;
    pub type SDL_bool = c_int;

    //SDL_video.h
    pub struct SDL_DisplayMode
    {
        format: uint32_t,
        w: c_int,
        h: c_int,
        refresh_rate: c_int,
        driverdata: *c_void
    }

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

    //SDL_surface.h
    pub struct SDL_Window;
    pub struct SDL_BlitMap;

    pub struct SDL_Surface {
        flags: uint32_t,
        format: *SDL_PixelFormat,
        w: c_int,
        h: c_int,
        pitch: c_int,
        pixels: *c_void,
        userdata: *c_void,
        locked: c_int,
        lock_data: *c_void,
        clip_rect: SDL_Rect,
        map: *SDL_BlitMap,
        refcount: c_int
    }

    //SDL_pixels.h
    pub struct SDL_Color {
        r: uint8_t,
        g: uint8_t,
        b: uint8_t,
        a: uint8_t,
    }

    pub struct SDL_Pallette {
        ncolors: c_int,
        colors: *SDL_Color,
        version: uint32_t,
        refcount: c_int
    }

    pub struct SDL_PixelFormat {
        format: uint32_t,
        palette: *SDL_Pallette,
        BitsPerPixel: uint8_t,
        BytesPerPixel: uint8_t,
        padding: [uint8_t, ..2],
        Rmask: uint8_t,
        Gmask: uint8_t,
        Bmask: uint8_t,
        Amask: uint8_t,
        Rloss: uint8_t,
        Gloss: uint8_t,
        Bloss: uint8_t,
        Aloss: uint8_t,
        Rshift: uint8_t,
        Gshift: uint8_t,
        Bshift: uint8_t,
        Ashift: uint8_t,
        refcount: c_int,
        next: *SDL_PixelFormat
    }

    /*struct SDL_RWops_Anon {
        data: [c_uchar, ..24],
    }

    pub struct SDL_RWops {
        pub seek: *uint8_t,
        pub read: *uint8_t,
        pub write: *uint8_t,
        pub close: *uint8_t,
        pub _type: uint32_t,
        pub hidden: SDL_RWops_Anon
    }


    pub struct SDL_Color {
        pub r: uint8_t,
        pub g: uint8_t,
        pub b: uint8_t,
        pub unused: uint8_t
    }

    pub struct SDL_Palette {
        pub ncolors: c_int,
        pub colors: *SDL_Color,
    }

    pub struct SDL_PixelFormat {
        pub palette: *SDL_Palette,
        pub BitsPerPixel: uint8_t,
        pub BytesPerPixel: uint8_t,
        pub Rloss: uint8_t,
        pub Gloss: uint8_t,
        pub Bloss: uint8_t,
        pub Aloss: uint8_t,
        pub Rshift: uint8_t,
        pub Gshift: uint8_t,
        pub Bshift: uint8_t,
        pub Ashift: uint8_t,
        pub Rmask: uint32_t,
        pub Gmask: uint32_t,
        pub Bmask: uint32_t,
        pub Amask: uint32_t,
        pub colorkey: uint32_t,
        pub alpha: uint8_t,
    }

    pub struct SDL_VideoInfo {
        pub flags: uint32_t,        // actually a set of packed fields
        pub video_mem: uint32_t,
        pub vfmt: *SDL_PixelFormat,
        pub current_w: c_int,
        pub current_h: c_int,
    }*/

    extern "C" {
        //SDL_video.h
        pub fn SDL_GetNumVideoDrivers() -> c_int;
        pub fn SDL_GetVideoDriver(index: c_int) -> *c_char;
        pub fn SDL_VideoInit(driver_name: *c_char) -> c_int;
        pub fn SDL_VideoQuit();
        pub fn SDL_GetCurrentVideoDriver() -> *c_char;
        pub fn SDL_GetNumVideoDisplays() -> c_int;
        pub fn SDL_GetDisplayName(displayIndex: c_int) -> *c_char;
        pub fn SDL_GetDisplayBounds(displayIndex: c_int, rect: *SDL_Rect) -> c_int;
        pub fn SDL_GetNumDisplayModes(displayIndex: c_int) -> c_int;
        pub fn SDL_GetDisplayMode(displayIndex: c_int, modeIndex: c_int, mode: *SDL_DisplayMode) -> c_int;
        pub fn SDL_GetDesktopDisplayMode(displayIndex: c_int, mode: *SDL_DisplayMode) -> c_int;
        pub fn SDL_GetCurrentDisplayMode(displayIndex: c_int, mode: *SDL_DisplayMode) -> c_int;
        pub fn SDL_GetClosestDisplayMode(displayIndex: c_int, mode: *SDL_DisplayMode, closest: *SDL_DisplayMode) -> *SDL_DisplayMode;
        pub fn SDL_GetWindowDisplayIndex(window: *SDL_Window) -> c_int;
        pub fn SDL_SetWindowDisplayMode(window: *SDL_Window, mode: *SDL_DisplayMode) -> c_int;
        pub fn SDL_GetWindowDisplayMode(window: *SDL_Window, mode: *SDL_DisplayMode) -> c_int;
        pub fn SDL_GetWindowPixelFormat(window: *SDL_Window) -> uint32_t;
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
        pub fn SDL_DestroyWindow(window: *SDL_Window);
        pub fn SDL_IsScreenSaverEnabled() -> SDL_bool;
        pub fn SDL_EnableScreenSaver();
        pub fn SDL_DisableScreenSaver();
        pub fn SDL_GL_LoadLibrary(path: *c_char) -> c_int;
        pub fn SDL_GL_GetProcAddress(proc: *c_char);
        pub fn SDL_GL_UnloadLibrary();
        pub fn SDL_GL_ExtensionSupported(extension: *c_char) -> SDL_bool;
        pub fn SDL_GL_SetAttribute(attr: SDL_GLattr, value: c_int) -> c_int;
        pub fn SDL_GL_GetAttribute(attr: SDL_GLattr, value: *c_int) -> c_int;
        pub fn SDL_GL_CreateContext(window: *SDL_Window) -> SDL_GLContext;
        pub fn SDL_GL_MakeCurrent(window: *SDL_Window, context: SDL_GLContext) -> c_int;
        pub fn SDL_GL_GetCurrentWindow() -> *SDL_Window;
        pub fn SDL_GL_GetCurrentContext() -> SDL_GLContext;
        pub fn SDL_GL_SetSwapInterval(interval: c_int) -> c_int;
        pub fn SDL_GL_GetSwapInterval() -> c_int;
        pub fn SDL_GL_SwapWindow(window: *SDL_Window);
        pub fn SDL_GL_DeleteContext(context: SDL_GLContext);
    }
}

#[deriving(Eq)]
pub struct Surface {
    raw: *ll::SDL_Surface,
    owned: bool //Might not be needed?
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
pub struct Window {
    raw: *ll::SDL_Window
}

impl Drop for Window {
    fn drop(&self) {
        unsafe {
            ll::SDL_DestroyWindow(self.raw);
        }
    }
}

impl Window {
    pub fn new(title: &str, x: int, y: int, width: int, height: int, window_flags: &[WindowFlags]) -> Result<~Window, ~str> {
        let flags = window_flags.iter().fold(0u32, |flags, flag| { flags | *flag as u32 });
        unsafe {
            let raw = do title.with_c_str |buff| {
                ll::SDL_CreateWindow(buff, x as c_int, y as c_int, width as c_int, height as c_int, flags) //FIXME: x and y are optional?
            };

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Window{ raw: raw })
            }
        }
    }
}

    /*pub fn new(surface_flags: &[SurfaceFlag], width: int, height: int, bpp: int,
               rmask: u32, gmask: u32, bmask: u32, amask: u32) -> Result<~Surface, ~str> {
        let flags = surface_flags.iter().fold(0u32, |flags, flag| { flags | *flag as u32 });

        unsafe {
            let raw = ll::SDL_CreateRGBSurface(flags, width as c_int, height as c_int, bpp as c_int,
                                               rmask, gmask, bmask, amask);

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }*/
/*fn wrap_surface(raw: *ll::SDL_Surface, owned: bool) -> ~Surface {
    ~Surface{ raw: raw, owned: owned }
}

impl Drop for Surface {
    pub fn drop(&self) {
        unsafe {
            if self.owned {
                ll::SDL_FreeSurface(self.raw);
            }
        }
    }
}*/

/*
#[deriving(Eq)]
pub struct Palette {
    colors: ~[Color]
}

fn wrap_palette(palette: *ll::SDL_Palette) -> Option<Palette> {
    match palette.is_null() {
        true => None,
        _ => Some(Palette {
            colors: unsafe {
                do vec::from_buf((*palette).colors, (*palette).ncolors as uint).map |color| {
                    Color::from_struct(color)
                }
            }
        })
    }
}

fn unwrap_palette(palette: &Palette) -> ll::SDL_Palette {
    ll::SDL_Palette {
        ncolors: palette.colors.len() as c_int,
        colors: vec::raw::to_ptr(do palette.colors.map |color| {
            color.to_struct()
        })
    }
}

#[deriving(Eq)]
pub struct PixelFormat {
    pub palette: Option<Palette>,
    pub bpp: u8,
    pub r_loss: u8,
    pub g_loss: u8,
    pub b_loss: u8,
    pub a_loss: u8,
    pub r_shift: u8,
    pub g_shift: u8,
    pub b_shift: u8,
    pub a_shift: u8,
    pub r_mask: u32,
    pub g_mask: u32,
    pub b_mask: u32,
    pub a_mask: u32,
    pub color_key: u32,
    pub alpha: u8
}

fn wrap_pixel_format(raw: *ll::SDL_PixelFormat) -> PixelFormat {
    let fmt = & unsafe { *raw };
    PixelFormat {
        palette: wrap_palette(fmt.palette),
        bpp: fmt.BitsPerPixel,
        r_loss: fmt.Rloss,
        g_loss: fmt.Gloss,
        b_loss: fmt.Bloss,
        a_loss: fmt.Aloss,
        r_shift: fmt.Rshift,
        g_shift: fmt.Gshift,
        b_shift: fmt.Bshift,
        a_shift: fmt.Ashift,
        r_mask: fmt.Rmask,
        g_mask: fmt.Gmask,
        b_mask: fmt.Bmask,
        a_mask: fmt.Amask,
        color_key: fmt.colorkey,
        alpha: fmt.alpha,
    }
}

fn unwrap_pixel_format(fmt: &PixelFormat) -> ll::SDL_PixelFormat {
    ll::SDL_PixelFormat {
        // FIXME: this will be freed at the end of this scope?
        palette: match fmt.palette {
            None => ptr::null(),
            Some(_) => {
                let workaround : *ll::SDL_Palette = &unwrap_palette(fmt.palette.get_ref());
                workaround
            }
        },
        BitsPerPixel: fmt.bpp,
        BytesPerPixel: fmt.bpp / 8,
        Rloss: fmt.r_loss,
        Gloss: fmt.g_loss,
        Bloss: fmt.b_loss,
        Aloss: fmt.a_loss,
        Rshift: fmt.r_shift,
        Gshift: fmt.g_shift,
        Bshift: fmt.b_shift,
        Ashift: fmt.a_shift,
        Rmask: fmt.r_mask,
        Gmask: fmt.g_mask,
        Bmask: fmt.b_mask,
        Amask: fmt.a_mask,
        colorkey: fmt.color_key,
        alpha: fmt.alpha
    }
}

#[deriving(Eq)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8)
}

impl rand::Rand for Color {
    fn rand<R: rand::Rng>(rng: &mut R) -> Color {
        if rng.gen() { RGBA(rng.gen(), rng.gen(), rng.gen(), rng.gen()) }
        else { RGB(rng.gen(), rng.gen(), rng.gen()) }
    }
}

impl Color {
    pub fn from_mapped(bit: u32, fmt: *ll::SDL_PixelFormat) -> Color {
        let r = 0;
        let g = 0;
        let b = 0;
        let a = 0;

        unsafe { ll::SDL_GetRGBA(bit, fmt,
                                 &r, &g,
                                 &b, &a) }

        RGBA(r, g, b, a)
    }

    pub fn to_mapped(&self, fmt: *ll::SDL_PixelFormat) -> u32 {
        match *self {
            RGB(r, g, b) => unsafe { ll::SDL_MapRGB(fmt, r, g, b) },
            RGBA(r, g, b, a) => unsafe { ll::SDL_MapRGBA(fmt, r, g, b, a) }
        }
    }

    pub fn from_struct(c: &ll::SDL_Color) -> Color {
        RGB(c.r, c.g, c.b)
    }

    pub fn to_struct(&self) -> ll::SDL_Color {
        match *self {
            RGB(r, g, b) => ll::SDL_Color {
                r: r,
                g: g,
                b: b,
                unused: 0,
            },
            RGBA(r, g, b, _) => ll::SDL_Color {
                r: r,
                g: g,
                b: b,
                unused: 0,
            }
        }
    }
}

#[deriving(Eq)]
pub enum SurfaceFlag {
    SWSurface = 0x00000000,
    HWSurface = 0x00000001,
    AsyncBlit = 0x00000004,
    SrcColorKey = 0x00001000,
    SrcAlpha = 0x00010000,
    RLEAccel = 0x00004000
}

#[deriving(Eq)]
pub enum VideoFlag {
    AnyFormat = 0x10000000,
    HWPalette = 0x20000000,
    DoubleBuf = 0x40000000,
    Fullscreen = 0x80000000,
    OpenGL = 0x00000002,
    OpenGLBlit = 0x0000000A,
    Resizable = 0x00000010,
    NoFrame = 0x00000020
}

pub fn set_video_mode(w: int, h: int, bpp: int,
                      surface_flags: &[SurfaceFlag],
                      video_flags: &[VideoFlag]) -> Result<~Surface, ~str> {
    let flags = do surface_flags.iter().fold(0u32) |flags, &flag| {
        flags | flag as u32
    };
    let flags = do video_flags.iter().fold(flags) |flags, &flag| {
        flags | flag as u32
    };

    unsafe {
        let raw = ll::SDL_SetVideoMode(w as c_int, h as c_int,
                                       bpp as c_int, flags);

        if raw == ptr::null() { Err(get_error()) }
        else { Ok(wrap_surface(raw, false)) }
    }
}

pub fn is_video_mode_ok(w: int, h: int, bpp: int,
                        surface_flags: &[SurfaceFlag],
                        video_flags: &[VideoFlag]) -> Option<int> {
    let flags = do surface_flags.iter().fold(0u32) |flags, &flag| {
        flags | flag as u32
    };
    let flags = do video_flags.iter().fold(flags) |flags, &flag| {
        flags | flag as u32
    };

    unsafe {
        let bpp = ll::SDL_VideoModeOK(w as c_int, h as c_int,
                                      bpp as c_int, flags);

        if bpp == 0 { None }
        else { Some(bpp as int) }
    }
}

#[deriving(Eq)]
pub enum VideoInfoFlag {
    HWAvailable    = 0x00000001,
    WMAvailable    = 0x00000002,
    BlitHW         = 0x00000200,
    BlitHWColorkey = 0x00000400,
    BlitHWAlpha    = 0x00000800,
    BlitSW         = 0x00001000,
    BlitSWColorkey = 0x00002000,
    BlitSWAlpha    = 0x00004000,
    BlitFill       = 0x00008000,
}

pub struct VideoInfo {
    pub flags: ~[VideoInfoFlag],
    pub width: int,
    pub height: int,
    pub format: PixelFormat,
}

fn wrap_video_info_flags(bitflags: u32) -> ~[VideoInfoFlag] {
    let flags = [HWAvailable,
        WMAvailable,
        BlitHW,
        BlitHWColorkey,
        BlitHWAlpha,
        BlitSW,
        BlitSWColorkey,
        BlitSWAlpha,
        BlitFill];

    do flags.iter().filter_map |&flag| {
        if bitflags & (flag as u32) != 0 { Some(flag) }
        else { None }
    }.collect()
}

pub fn get_video_info() -> ~VideoInfo {
    let raw = unsafe { ll::SDL_GetVideoInfo() };
    ~VideoInfo {
        flags:  wrap_video_info_flags(unsafe { (*raw).flags } as u32),
        width:  unsafe { (*raw).current_w } as int,
        height: unsafe { (*raw).current_h } as int,
        format: wrap_pixel_format(unsafe { (*raw).vfmt }),
    }
}

pub enum PaletteType {
    LogicalPaletteType = 1,
    PhysicalPaletteType
}

pub fn get_video_surface() -> Result<~Surface, ~str> {
    let raw = unsafe { ll::SDL_GetVideoSurface() };

    if raw.is_null() { Err(get_error()) }
    else { Ok(wrap_surface(raw, false)) }
}

// TODO: get_video_modes, get_video_driver_name

impl Surface {
    pub fn new(surface_flags: &[SurfaceFlag], width: int, height: int, bpp: int,
               rmask: u32, gmask: u32, bmask: u32, amask: u32) -> Result<~Surface, ~str> {
        let flags = surface_flags.iter().fold(0u32, |flags, flag| { flags | *flag as u32 });

        unsafe {
            let raw = ll::SDL_CreateRGBSurface(flags, width as c_int, height as c_int, bpp as c_int,
                                               rmask, gmask, bmask, amask);

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn from_bmp(path: &Path) -> Result<~Surface, ~str> {
        let raw = unsafe {
            do path.to_str().as_c_str |buf| {
                do "rb".as_c_str |mode_buf| {
                    ll::SDL_LoadBMP_RW(ll::SDL_RWFromFile(buf, mode_buf), 1)
                }
            }
        };

        if raw.is_null() { Err(get_error()) }
        else { Ok(wrap_surface(raw, true)) }
    }

    // TODO: from_data (hard because the pixel data has to stay alive)

    pub fn get_width(&self) -> u16 {
        unsafe { (*self.raw).w as u16 }
    }

    pub fn get_height(&self) -> u16 {
        unsafe { (*self.raw).h as u16 }
    }

    pub fn get_size(&self) -> (u16, u16) {
        (self.get_width(), self.get_height())
    }

    pub fn get_rect(&self) -> Rect {
        Rect {
            x: 0,
            y: 0,
            w: self.get_width(),
            h: self.get_height()
        }
    }

    pub fn update_rect(&self, rect: &Rect) {
        unsafe {
            ll::SDL_UpdateRect(self.raw, rect.x as i32, rect.y as i32,
                               rect.w as u32, rect.h as u32);
        }
    }

    pub fn update_rects(&self, rects: &[Rect]) {
        unsafe {
            ll::SDL_UpdateRects(self.raw, rects.len() as c_int,
                                cast::transmute(vec::raw::to_ptr(rects)));
        }
    }

    pub fn set_colors(&self, colors: ~[Color]) -> bool {
        let colors = do colors.map |color| {
            color.to_struct()
        };

        unsafe { ll::SDL_SetColors(self.raw, vec::raw::to_ptr(colors), 0,
                                   colors.len() as c_int) == 1 }
    }

    pub fn set_palette(&self, palettes: &[PaletteType],
                   colors: ~[Color]) -> bool {
        let colors = do colors.map |color| {
            color.to_struct()
        };
        let flags = do palettes.iter().fold(0 as c_int) |flags, &flag| {
            flags | flag as c_int
        };

        unsafe { ll::SDL_SetPalette(self.raw, flags,
                                    vec::raw::to_ptr(colors), 0,
                                    colors.len() as c_int) == 1 }
    }

    pub fn lock(&self) -> bool {
        unsafe { ll::SDL_LockSurface(self.raw) == 0 }
    }

    /// Locks a surface so that the pixels can be directly accessed safely.
    pub fn with_lock<R>(&self, f: &fn(pixels: &mut [u8]) -> R) -> R {
        unsafe {
            if ll::SDL_LockSurface(self.raw) != 0 { fail!(~"could not lock surface"); }
            let len = (*self.raw).pitch as uint * ((*self.raw).h as uint);
            let pixels: &mut [u8] = cast::transmute(((*self.raw).pixels, len));
            let rv = f(pixels);
            ll::SDL_UnlockSurface(self.raw);
            rv
        }
    }

    pub fn unlock(&self) {
        unsafe { ll::SDL_UnlockSurface(self.raw); }
    }

    pub fn flip(&self) -> bool {
        unsafe { ll::SDL_Flip(self.raw) == 0 }
    }

    pub fn convert(&self, fmt: &PixelFormat, flags: &[SurfaceFlag]) -> Result<~Surface, ~str> {
        let flags = do flags.iter().fold(0u32) |flags, &flag| {
            flags | flag as u32
        };

        let rawfmt = unwrap_pixel_format(fmt);

        let new = unsafe { ll::SDL_ConvertSurface(self.raw, &rawfmt, flags) };
        match new.is_null() {
            true  => Err(get_error()),
            false => Ok(wrap_surface(new, true)),
        }
    }

    pub fn display_format(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ll::SDL_DisplayFormat(self.raw) };

        if raw.is_null() { Err(get_error()) }
        else { Ok(wrap_surface(raw, true)) }
    }

    pub fn display_format_alpha(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ll::SDL_DisplayFormatAlpha(self.raw) };

        if raw.is_null() { Err(get_error()) }
        else { Ok(wrap_surface(raw, true)) }
    }

    pub fn save_bmp(&self, path: &Path) -> bool {
        unsafe {
            do path.to_str().as_c_str |buf| {
                do "wb".as_c_str |mode_buf| {
                    ll::SDL_SaveBMP_RW(self.raw, ll::SDL_RWFromFile(buf, mode_buf), 1) == 0
                }
            }
        }
    }

    pub fn set_alpha(&self, flags: &[SurfaceFlag], alpha: u8) -> bool {
        let flags = do flags.iter().fold(0u32) |flags, &flag| {
            flags | flag as u32
        };

        unsafe {
            ll::SDL_SetAlpha(self.raw, flags, alpha) == 0
        }
    }

    pub fn set_color_key(&self, flags: &[SurfaceFlag], color: Color) -> bool {
        let flags = do flags.iter().fold(0u32) |flags, &flag| {
            flags | flag as u32
        };

        unsafe {
            ll::SDL_SetColorKey(self.raw, flags,
                                color.to_mapped((*self.raw).format)) == 0
        }
    }

    pub fn set_clip_rect(&self, rect: &Rect) {
        unsafe {
            ll::SDL_SetClipRect(self.raw, rect);
        }
    }

    pub fn get_clip_rect(&self) -> Rect {
        let rect = Rect {
            x: 0,
            y: 0,
            w: 0,
            h: 0
        };

        unsafe {
            ll::SDL_SetClipRect(self.raw,
                                cast::transmute(&rect));
        }

        rect
    }

    pub fn blit_rect(&self, src: &Surface, src_rect: Option<Rect>,
                     dest_rect: Option<Rect>) -> bool {
        unsafe {
            ll::SDL_UpperBlit(src.raw, match src_rect {
                Some(ref rect) => cast::transmute(rect),
                None => ptr::null()
            }, self.raw, match dest_rect {
                Some(ref rect) => cast::transmute(rect),
                None => ptr::null()
            }) == 0
        }
    }

    pub fn blit(&self, src: &Surface) -> bool {
        self.blit_rect(src, None, None)
    }

    pub fn blit_at(&self, src: &Surface, x: i16, y: i16) -> bool {
        let (w, h) = src.get_size();

        self.blit_rect(src, None, Some(Rect {
            x: x,
            y: y,
            w: w,
            h: h
        }))
    }

    pub fn fill_rect(&self, rect: Option<Rect>,
                     color: Color) -> bool {
        unsafe { ll::SDL_FillRect(self.raw, match rect {
            Some(ref rect) => cast::transmute(rect),
            None => ptr::null()
        }, color.to_mapped((*self.raw).format)) == 0 }
    }

    pub fn fill(&self, color: Color) -> bool {
        self.fill_rect(None, color)
    }

    pub fn clear(&self) -> bool {
        self.fill(RGB(0, 0, 0))
    }
}

pub fn set_gamma(r: float, g: float, b: float) -> bool {
    unsafe { ll::SDL_SetGamma(r as c_float, g as c_float,
                              b as c_float) != -1 }
}

pub fn set_gamma_ramp(r: Option<[u16, ..256]>, g: Option<[u16, ..256]>,
                      b: Option<[u16, ..256]>) -> bool {
    unsafe { ll::SDL_SetGammaRamp(match r {
        Some(r) => vec::raw::to_ptr(r),
        None => ptr::null()
    }, match g {
        Some(g) => vec::raw::to_ptr(g),
        None => ptr::null()
    }, match b {
        Some(b) => vec::raw::to_ptr(b),
        None => ptr::null()
    }) != -1 }
}

pub fn get_gamma_ramp() -> ([u16, ..256], [u16, ..256], [u16, ..256]) {
    let r = [0u16, .. 256];
    let g = [0u16, .. 256];
    let b = [0u16, .. 256];

    unsafe { ll::SDL_GetGammaRamp(vec::raw::to_ptr(r),
                                  vec::raw::to_ptr(g),
                                  vec::raw::to_ptr(b)); }

    (r, g, b)
}

pub fn swap_buffers() {
    unsafe {
        ll::SDL_GL_SwapBuffers();
    }
}*/


// TODO: YUV
