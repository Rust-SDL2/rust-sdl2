use libc::c_char;
use std::cell::Cell;
use std::error;
use std::ffi::{CStr, CString, NulError};
use std::fmt;
use std::mem::transmute;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::sys;

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Error {
    NoMemError = sys::SDL_errorcode::SDL_ENOMEM as i32,
    ReadError = sys::SDL_errorcode::SDL_EFREAD as i32,
    WriteError = sys::SDL_errorcode::SDL_EFWRITE as i32,
    SeekError = sys::SDL_errorcode::SDL_EFSEEK as i32,
    UnsupportedError = sys::SDL_errorcode::SDL_UNSUPPORTED as i32,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match *self {
            NoMemError => write!(f, "Out of memory"),
            ReadError => write!(f, "Error reading from datastream"),
            WriteError => write!(f, "Error writing to datastream"),
            SeekError => write!(f, "Error seeking in datastream"),
            UnsupportedError => write!(f, "Unknown SDL error"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;

        match *self {
            NoMemError => "out of memory",
            ReadError => "error reading from datastream",
            WriteError => "error writing to datastream",
            SeekError => "error seeking in datastream",
            UnsupportedError => "unknown SDL error",
        }
    }
}

/// True if the main thread has been declared. The main thread is declared when
/// SDL is first initialized.
static IS_MAIN_THREAD_DECLARED: AtomicBool = AtomicBool::new(false);

/// Number of active `SdlDrop` objects keeping SDL alive.
static SDL_COUNT: AtomicU32 = AtomicU32::new(0);

thread_local! {
    /// True if the current thread is the main thread.
    static IS_MAIN_THREAD: Cell<bool> = Cell::new(false);
}

/// The SDL context type. Initialize with `sdl2::init()`.
///
/// From a thread-safety perspective, `Sdl` represents the main thread.
/// As such, `Sdl` is a useful type for ensuring that SDL types that can only
/// be used on the main thread are initialized that way.
///
/// For instance, `SDL_PumpEvents()` is not thread safe, and may only be
/// called on the main thread.
/// All functionality that calls `SDL_PumpEvents()` is thus put into an
/// `EventPump` type, which can only be obtained through `Sdl`.
/// This guarantees that the only way to call event-pumping functions is on
/// the main thread.
#[derive(Clone)]
pub struct Sdl {
    sdldrop: SdlDrop,
}

impl Sdl {
    #[inline]
    #[doc(alias = "SDL_Init")]
    fn new() -> Result<Sdl, String> {
        // Check if we can safely initialize SDL on this thread.
        let was_main_thread_declared = IS_MAIN_THREAD_DECLARED.swap(true, Ordering::SeqCst);

        IS_MAIN_THREAD.with(|is_main_thread| {
            if was_main_thread_declared {
                if !is_main_thread.get() {
                    return Err("Cannot initialize `Sdl` from more than one thread.".to_owned());
                }
            } else {
                is_main_thread.set(true);
            }
            Ok(())
        })?;

        // Initialize SDL.
        if SDL_COUNT.fetch_add(1, Ordering::Relaxed) == 0 {
            let result;

            unsafe {
                result = sys::SDL_Init(0);
            }

            if result != 0 {
                SDL_COUNT.store(0, Ordering::Relaxed);
                return Err(get_error());
            }
        }

        Ok(Sdl {
            sdldrop: SdlDrop {
                _anticonstructor: std::ptr::null_mut(),
            },
        })
    }

    /// Initializes the audio subsystem.
    #[inline]
    pub fn audio(&self) -> Result<AudioSubsystem, String> {
        AudioSubsystem::new(self)
    }

    /// Initializes the event subsystem.
    #[inline]
    pub fn event(&self) -> Result<EventSubsystem, String> {
        EventSubsystem::new(self)
    }

    /// Initializes the joystick subsystem.
    #[inline]
    pub fn joystick(&self) -> Result<JoystickSubsystem, String> {
        JoystickSubsystem::new(self)
    }

    /// Initializes the haptic subsystem.
    #[inline]
    pub fn haptic(&self) -> Result<HapticSubsystem, String> {
        HapticSubsystem::new(self)
    }

    /// Initializes the game controller subsystem.
    #[inline]
    pub fn game_controller(&self) -> Result<GameControllerSubsystem, String> {
        GameControllerSubsystem::new(self)
    }

    /// Initializes the game controller subsystem.
    #[inline]
    pub fn sensor(&self) -> Result<SensorSubsystem, String> {
        SensorSubsystem::new(self)
    }

    /// Initializes the timer subsystem.
    #[inline]
    pub fn timer(&self) -> Result<TimerSubsystem, String> {
        TimerSubsystem::new(self)
    }

    /// Initializes the video subsystem.
    #[inline]
    pub fn video(&self) -> Result<VideoSubsystem, String> {
        VideoSubsystem::new(self)
    }

    /// Obtains the SDL event pump.
    ///
    /// At most one `EventPump` is allowed to be alive during the program's execution.
    /// If this function is called while an `EventPump` instance is alive, the function will return
    /// an error.
    #[inline]
    pub fn event_pump(&self) -> Result<EventPump, String> {
        EventPump::new(self)
    }

    #[inline]
    #[doc(hidden)]
    pub fn sdldrop(&self) -> SdlDrop {
        self.sdldrop.clone()
    }
}

/// When SDL is no longer in use, the library is quit.
#[doc(hidden)]
#[derive(Debug)]
pub struct SdlDrop {
    // Make it impossible to construct `SdlDrop` without access to this member,
    // and opt out of Send and Sync.
    _anticonstructor: *mut c_void,
}

impl Clone for SdlDrop {
    fn clone(&self) -> SdlDrop {
        let prev_count = SDL_COUNT.fetch_add(1, Ordering::Relaxed);
        assert!(prev_count > 0);
        SdlDrop {
            _anticonstructor: std::ptr::null_mut(),
        }
    }
}

impl Drop for SdlDrop {
    #[inline]
    #[doc(alias = "SDL_Quit")]
    fn drop(&mut self) {
        let prev_count = SDL_COUNT.fetch_sub(1, Ordering::Relaxed);
        assert!(prev_count > 0);
        if prev_count == 1 {
            unsafe {
                sys::SDL_Quit();
            }
            IS_MAIN_THREAD_DECLARED.swap(false, Ordering::SeqCst);
        }
    }
}

// No subsystem can implement `Send` because the destructor, `SDL_QuitSubSystem`,
// utilizes non-atomic reference counting and should thus be called on a single thread.
// Some subsystems have functions designed to be thread-safe, such as adding a timer or accessing
// the event queue. These subsystems implement `Sync`.

macro_rules! subsystem {
    ($name:ident, $flag:expr, $counter:ident, nosync) => {
        static $counter: AtomicU32 = AtomicU32::new(0);

        #[derive(Debug, Clone)]
        pub struct $name {
            /// Subsystems cannot be moved or (usually) used on non-main threads.
            /// Luckily, Rc restricts use to the main thread.
            _subsystem_drop: SubsystemDrop,
        }

        impl $name {
            #[inline]
            #[doc(alias = "SDL_InitSubSystem")]
            fn new(sdl: &Sdl) -> Result<$name, String> {
                if $counter.fetch_add(1, Ordering::Relaxed) == 0 {
                    let result;

                    unsafe {
                        result = sys::SDL_InitSubSystem($flag);
                    }

                    if result != 0 {
                        $counter.store(0, Ordering::Relaxed);
                        return Err(get_error());
                    }
                }

                Ok($name {
                    _subsystem_drop: SubsystemDrop {
                        _sdldrop: sdl.sdldrop.clone(),
                        counter: &$counter,
                        flag: $flag,
                    },
                })
            }

            /// Obtain an SDL context.
            #[inline]
            pub fn sdl(&self) -> Sdl {
                Sdl {
                    sdldrop: self._subsystem_drop._sdldrop.clone(),
                }
            }
        }
    };
    ($name:ident, $flag:expr, $counter:ident, sync) => {
        subsystem!($name, $flag, $counter, nosync);
        unsafe impl Sync for $name {}
    };
}

/// When a subsystem is no longer in use (the refcount in an `Rc<SubsystemDrop>` reaches 0),
/// the subsystem is quit.
#[derive(Debug)]
struct SubsystemDrop {
    _sdldrop: SdlDrop,
    counter: &'static AtomicU32,
    flag: u32,
}

impl Clone for SubsystemDrop {
    fn clone(&self) -> SubsystemDrop {
        let prev_count = self.counter.fetch_add(1, Ordering::Relaxed);
        assert!(prev_count > 0);
        SubsystemDrop {
            _sdldrop: self._sdldrop.clone(),
            counter: self.counter,
            flag: self.flag,
        }
    }
}

impl Drop for SubsystemDrop {
    #[inline]
    #[doc(alias = "SDL_QuitSubSystem")]
    fn drop(&mut self) {
        let prev_count = self.counter.fetch_sub(1, Ordering::Relaxed);
        assert!(prev_count > 0);
        if prev_count == 1 {
            unsafe {
                sys::SDL_QuitSubSystem(self.flag);
            }
        }
    }
}

subsystem!(AudioSubsystem, sys::SDL_INIT_AUDIO, AUDIO_COUNT, nosync);
subsystem!(
    GameControllerSubsystem,
    sys::SDL_INIT_GAMECONTROLLER,
    GAMECONTROLLER_COUNT,
    nosync
);
subsystem!(HapticSubsystem, sys::SDL_INIT_HAPTIC, HAPTIC_COUNT, nosync);
subsystem!(
    JoystickSubsystem,
    sys::SDL_INIT_JOYSTICK,
    JOYSTICK_COUNT,
    nosync
);
subsystem!(VideoSubsystem, sys::SDL_INIT_VIDEO, VIDEO_COUNT, nosync);
// Timers can be added on other threads.
subsystem!(TimerSubsystem, sys::SDL_INIT_TIMER, TIMER_COUNT, sync);
// The event queue can be read from other threads.
subsystem!(EventSubsystem, sys::SDL_INIT_EVENTS, EVENTS_COUNT, sync);
subsystem!(SensorSubsystem, sys::SDL_INIT_SENSOR, SENSOR_COUNT, sync);

static IS_EVENT_PUMP_ALIVE: AtomicBool = AtomicBool::new(false);

/// A thread-safe type that encapsulates SDL event-pumping functions.
pub struct EventPump {
    _event_subsystem: EventSubsystem,
}

impl EventPump {
    /// Obtains the SDL event pump.
    #[inline]
    #[doc(alias = "SDL_InitSubSystem")]
    fn new(sdl: &Sdl) -> Result<EventPump, String> {
        // Called on the main SDL thread.
        if IS_EVENT_PUMP_ALIVE.load(Ordering::Relaxed) {
            Err("an `EventPump` instance is already alive - there can only be one `EventPump` in use at a time.".to_owned())
        } else {
            let _event_subsystem = sdl.event()?;
            IS_EVENT_PUMP_ALIVE.store(true, Ordering::Relaxed);
            Ok(EventPump { _event_subsystem })
        }
    }
}

impl Drop for EventPump {
    #[inline]
    #[doc(alias = "SDL_QuitSubSystem")]
    fn drop(&mut self) {
        // Called on the main SDL thread.
        assert!(IS_EVENT_PUMP_ALIVE.load(Ordering::Relaxed));
        IS_EVENT_PUMP_ALIVE.store(false, Ordering::Relaxed);
    }
}

/// Get platform name
#[inline]
#[doc(alias = "SDL_GetPlatform")]
pub fn get_platform() -> &'static str {
    unsafe { CStr::from_ptr(sys::SDL_GetPlatform()).to_str().unwrap() }
}

/// Initializes the SDL library.
/// This must be called before using any other SDL function.
///
/// # Example
/// ```no_run
/// let sdl_context = sdl2::init().unwrap();
/// let mut event_pump = sdl_context.event_pump().unwrap();
///
/// for event in event_pump.poll_iter() {
///     // ...
/// }
///
/// // SDL_Quit() is called here as `sdl_context` is dropped.
/// ```
#[inline]
#[doc(alias = "SDL_GetError")]
pub fn init() -> Result<Sdl, String> {
    Sdl::new()
}

pub fn get_error() -> String {
    unsafe {
        let err = sys::SDL_GetError();
        CStr::from_ptr(err as *const _).to_str().unwrap().to_owned()
    }
}

#[doc(alias = "SDL_SetError")]
pub fn set_error(err: &str) -> Result<(), NulError> {
    let c_string = CString::new(err)?;
    unsafe {
        sys::SDL_SetError(
            b"%s\0".as_ptr() as *const c_char,
            c_string.as_ptr() as *const c_char,
        );
    }
    Ok(())
}

#[doc(alias = "SDL_Error")]
pub fn set_error_from_code(err: Error) {
    unsafe {
        sys::SDL_Error(transmute(err));
    }
}

#[doc(alias = "SDL_ClearError")]
pub fn clear_error() {
    unsafe {
        sys::SDL_ClearError();
    }
}
