use sys::{SDL_LogSetOutputFunction, SDL_LogPriority};
use std::ptr::null_mut;
use std::ffi::{CStr, CString};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Category {
    Application,
    Error,
    Assert,
    System,
    Audio,
    Video,
    Render,
    Input,
    Test,
    Custom,
    Unknown,
}

impl Category {
    #[allow(dead_code)]
    fn from_ll(value: u32) -> Category {
        use ::sys::SDL_LOG_CATEGORY::*;
        if value == APPLICATION as u32 {
            Category::Application
        } else if value == ERROR as u32 {
            Category::Error
        } else if value == ASSERT as u32 {
            Category::Assert
        } else if value == SYSTEM as u32 {
            Category::System
        } else if value == AUDIO as u32 {
            Category::Audio
        } else if value == VIDEO as u32 {
            Category::Video
        } else if value == RENDER as u32 {
            Category::Render
        } else if value == INPUT as u32 {
            Category::Input
        } else if value == TEST as u32 {
            Category::Test
        } else {
            Category::Custom
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Priority {
    Verbose,
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

impl Priority {
    fn from_ll(value: SDL_LogPriority) -> Priority {
        use sys::SDL_LogPriority::*;
        match value {
            SDL_LOG_PRIORITY_VERBOSE => Priority::Verbose,
            SDL_LOG_PRIORITY_DEBUG => Priority::Debug,
            SDL_LOG_PRIORITY_INFO => Priority::Info,
            SDL_LOG_PRIORITY_WARN => Priority::Warn,
            SDL_LOG_PRIORITY_ERROR => Priority::Error,
            SDL_LOG_PRIORITY_CRITICAL | _ => Priority::Critical,
        }
    }
}

fn dummy(_priority: Priority, _category: Category, _message: &str) {
}

#[allow(non_upper_case_globals)]
// NEVER make this public
static mut custom_log_fn : fn(Priority, Category, &str) = dummy;

unsafe extern "C" fn rust_sdl2_log_fn(_userdata: *mut ::std::os::raw::c_void,
                                   category: ::std::os::raw::c_int,
                                   priority: SDL_LogPriority,
                                   message: *const ::std::os::raw::c_char) {
    let category = Category::from_ll(category as u32);
    let priority = Priority::from_ll(priority);
    let message = CStr::from_ptr(message).to_string_lossy();
    custom_log_fn(priority, category, &*message);
}

pub fn set_output_function(callback : fn(Priority, Category, &str)) {
    unsafe {
        custom_log_fn = callback;
        SDL_LogSetOutputFunction(Some(rust_sdl2_log_fn), null_mut());
    };
}

/// Standard log function which takes as priority INFO and
/// as category APPLICATION
pub fn log(message: &str) {
    let message = message.replace('%', "%%");
    let message = CString::new(message).unwrap();
    unsafe {
        ::sys::SDL_Log(message.into_raw());
    }
}

