use std::ffi::{CStr, CString};
use std::marker::{NoCopy, PhantomData};

use sys::sdl as ll;
use event::EventPump;

bitflags! {
    flags InitFlag: u32 {
        const INIT_TIMER = ll::SDL_INIT_TIMER,
        const INIT_AUDIO = ll::SDL_INIT_AUDIO,
        const INIT_VIDEO = ll::SDL_INIT_VIDEO,
        const INIT_JOYSTICK = ll::SDL_INIT_JOYSTICK,
        const INIT_HAPTIC = ll::SDL_INIT_HAPTIC,
        const INIT_GAME_CONTROLLER = ll::SDL_INIT_GAMECONTROLLER,
        const INIT_EVENTS = ll::SDL_INIT_EVENTS,
        const INIT_NOPARACHUTE = ll::SDL_INIT_NOPARACHUTE,
        const INIT_EVERYTHING = ll::SDL_INIT_EVERYTHING
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Error {
    NoMemError = ll::SDL_ENOMEM as isize,
    ReadError = ll::SDL_EFREAD as isize,
    WriteError = ll::SDL_EFWRITE as isize,
    SeekError = ll::SDL_EFSEEK as isize,
    UnsupportedError = ll::SDL_UNSUPPORTED as isize
}

pub type SdlResult<T> = Result<T, String>;

use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};
/// Only one Sdl context can be alive at a time.
/// Set to false by default (not alive).
static IS_SDL_CONTEXT_ALIVE: AtomicBool = ATOMIC_BOOL_INIT;

/// The SDL context type. Initialize with `sdl2::init()`.
///
/// From a thread-safety perspective, `Sdl` represents the main thread.
/// Only one instance of `Sdl` is allowed per process, and cannot be moved or
/// used across non-main threads.
///
/// As such, `Sdl` is a useful type for ensuring that SDL types that can only
/// be used on the main thread are initialized that way.
///
/// For instance, `SDL_PumpEvents()` is not thread safe, and may only be
/// called on the main thread.
/// All functionality that calls `SDL_PumpEvents()` is thus put into an
/// `EventPump` type, which can only be obtained through `Sdl`.
/// This guarantees that the only way to call event-pumping functions is on
/// the main thread.
pub struct Sdl {
    _marker: NoCopy
}

impl !Send for Sdl {}
impl !Sync for Sdl {}

impl Sdl {
    /// Initializes specific SDL subsystems.
    pub fn init_subsystem(&self, flags: InitFlag) -> SdlResult<Subsystem> {
        unsafe {
            if ll::SDL_InitSubSystem(flags.bits()) == 0 {
                Ok(Subsystem {
                    flags: flags,
                    _marker: PhantomData
                })
            } else {
                Err(get_error())
            }
        }
    }

    /// Returns the mask of the specified subsystems which have previously been initialized.
    pub fn was_init(&self, flags: InitFlag) -> InitFlag {
        unsafe {
            let raw = ll::SDL_WasInit(flags.bits());
            flags & InitFlag::from_bits(raw).unwrap()
        }
    }

    /// Obtains the SDL event pump.
    pub fn event_pump(&self) -> EventPump {
        unsafe { EventPump::_unchecked_new() }
    }
}

impl Drop for Sdl {
    fn drop(&mut self) {
        use std::sync::atomic::Ordering;

        let was_alive = IS_SDL_CONTEXT_ALIVE.swap(false, Ordering::Relaxed);
        assert!(was_alive);

        unsafe { ll::SDL_Quit(); }
    }
}

/// A RAII value representing initalized SDL subsystems. See `sdl2::Sdl::init_subsystem()`.
///
/// Subsystem initialization is ref-counted. Once `Subsystem::drop()` is called,
/// the specified subsystems' ref-counts are decremented via `SDL_QuitSubSystem`.
pub struct Subsystem<'sdl> {
    flags: InitFlag,
    _marker: PhantomData<&'sdl Sdl>
}

#[unsafe_destructor]
impl<'sdl> Drop for Subsystem<'sdl> {
    fn drop(&mut self) {
        unsafe { ll::SDL_QuitSubSystem(self.flags.bits()); }
    }
}

/// Initializes the SDL library.
/// This must be called before using any other SDL function.
///
/// # Example
/// ```no_run
/// let sdl_context = sdl2::init(sdl2::INIT_EVERYTHING).unwrap();
///
/// let mut event_pump = sdl_context.event_pump();
/// for event in event_pump.poll_iter() {
///     // ...
/// }
///
/// // SDL_Quit() is called here as `sdl_context` is dropped.
/// ```
pub fn init(flags: InitFlag) -> SdlResult<Sdl> {
    unsafe {
        use std::sync::atomic::Ordering;

        // Atomically switch the `IS_SDL_CONTEXT_ALIVE` global to true
        let was_alive = IS_SDL_CONTEXT_ALIVE.swap(true, Ordering::Relaxed);

        if was_alive {
            IS_SDL_CONTEXT_ALIVE.swap(false, Ordering::Relaxed);
            Err(format!("Cannot have more than one `Sdl` in use at the same time"))
        } else {
            if ll::SDL_Init(flags.bits()) == 0 {
                Ok(Sdl {
                    _marker: NoCopy
                })
            } else {
                IS_SDL_CONTEXT_ALIVE.swap(false, Ordering::Relaxed);
                Err(get_error())
            }
        }
    }
}

pub fn get_error() -> String {
    unsafe {
        let err = ll::SDL_GetError();
        String::from_utf8_lossy(CStr::from_ptr(err).to_bytes()).to_string()
    }
}

pub fn set_error(err: &str) {
    let buf = CString::new(err).unwrap().as_ptr();
    unsafe { ll::SDL_SetError(buf); }
}

pub fn set_error_from_code(err: Error) {
    unsafe { ll::SDL_Error(err as ll::SDL_errorcode); }
}

pub fn clear_error() {
    unsafe { ll::SDL_ClearError(); }
}

pub fn get_ticks() -> u32 {
    unsafe { ll::SDL_GetTicks() as u32 }
}
