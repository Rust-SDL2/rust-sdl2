use std::ffi::{CStr, CString};
use std::marker::PhantomData;

use sys::sdl as ll;
use event::EventPump;
use video::WindowBuilder;
use util::CStringExt;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
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
    _nosyncsend: PhantomData<*mut ()>
}

impl Sdl {
    /// Returns the mask of the specified subsystems which have previously been initialized.
    pub fn was_init(&self, flags: u32) -> u32 {
        unsafe {
            ll::SDL_WasInit(flags)
        }
    }

    /// Obtains the SDL event pump.
    pub fn event_pump(&mut self) -> EventPump {
        EventPump::new(self)
    }

    /// Initializes a new `WindowBuilder`; a convenience method that calls `WindowBuilder::new()`.
    pub fn window(&self, title: &str, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder::new(self, title, width, height)
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
    flags: u32,
    _marker: PhantomData<&'sdl Sdl>
}

impl<'sdl> Drop for Subsystem<'sdl> {
    fn drop(&mut self) {
        unsafe { ll::SDL_QuitSubSystem(self.flags); }
    }
}

/// The type that allows you to build the SDL2 context.
pub struct InitBuilder {
    flags: u32
}

impl InitBuilder {
    /// Initializes a new `InitBuilder`.
    pub fn new() -> InitBuilder {
        InitBuilder { flags: 0 }
    }

    /// Builds the SDL2 context.
    pub fn build(&self) -> SdlResult<Sdl> {
        unsafe {
            use std::sync::atomic::Ordering;

            // Atomically switch the `IS_SDL_CONTEXT_ALIVE` global to true
            let was_alive = IS_SDL_CONTEXT_ALIVE.swap(true, Ordering::Relaxed);

            if was_alive {
                Err(format!("Cannot have more than one `Sdl` in use at the same time"))
            } else {
                if ll::SDL_Init(self.flags) == 0 {
                    Ok(Sdl {
                        _nosyncsend: PhantomData
                    })
                } else {
                    IS_SDL_CONTEXT_ALIVE.swap(false, Ordering::Relaxed);
                    Err(get_error())
                }
            }
        }
    }

    /// Builds the SDL2 context. Convenience method for `.build().unwrap()`.
    ///
    /// Panics if there was an error initializing SDL2.
    pub fn unwrap(&self) -> Sdl { self.build().unwrap() }

    /// Builds an SDL2 subsystem. Requires SDL2 to have already been initialized.
    pub fn build_subsystem(&self, _sdl: &Sdl) -> SdlResult<Subsystem> {
        unsafe {
            if ll::SDL_InitSubSystem(self.flags) == 0 {
                Ok(Subsystem {
                    flags: self.flags,
                    _marker: PhantomData
                })
            } else {
                Err(get_error())
            }
        }
    }

    /// Initializes every subsystem.
    pub fn everything(&mut self) -> &mut InitBuilder {
        self.flags |= ll::SDL_INIT_EVERYTHING as u32;
        self
    }

    /// Initializes the timer subsystem.
    pub fn timer(&mut self) -> &mut InitBuilder {
        self.flags |= ll::SDL_INIT_TIMER as u32;
        self
    }

    /// Initializes the audio subsystem.
    pub fn audio(&mut self) -> &mut InitBuilder {
        self.flags |= ll::SDL_INIT_AUDIO as u32;
        self
    }

    /// Initializes the video subsystem.
    pub fn video(&mut self) -> &mut InitBuilder {
        self.flags |= ll::SDL_INIT_VIDEO as u32;
        self
    }

    /// Initializes the joystick subsystem.
    pub fn joystick(&mut self) -> &mut InitBuilder {
        self.flags |= ll::SDL_INIT_JOYSTICK as u32;
        self
    }

    /// Initializes the haptic (force feedback) subsystem.
    pub fn haptic(&mut self) -> &mut InitBuilder {
        self.flags |= ll::SDL_INIT_HAPTIC as u32;
        self
    }

    /// Initializes the controller subsystem.
    pub fn game_controller(&mut self) -> &mut InitBuilder {
        self.flags |= ll::SDL_INIT_GAMECONTROLLER as u32;
        self
    }

    /// Initializes the events subsystem.
    pub fn events(&mut self) -> &mut InitBuilder {
        self.flags |= ll::SDL_INIT_EVENTS as u32;
        self
    }
}

/// Initializes the SDL library.
/// This must be called before using any other SDL function.
///
/// # Example
/// ```no_run
/// let mut sdl_context = sdl2::init().everything().unwrap();
///
/// for event in sdl_context.event_pump().poll_iter() {
///     // ...
/// }
///
/// // SDL_Quit() is called here as `sdl_context` is dropped.
/// ```
pub fn init() -> InitBuilder { InitBuilder::new() }

pub fn get_error() -> String {
    unsafe {
        let err = ll::SDL_GetError();
        String::from_utf8_lossy(CStr::from_ptr(err).to_bytes()).to_string()
    }
}

pub fn set_error(err: &str) {
    let err = CString::new(err).remove_nul();
    unsafe { ll::SDL_SetError(err.as_ptr()); }
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
