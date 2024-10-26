use crate::sys;
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::ptr::null_mut;

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
        if value == sys::SDL_LogCategory::SDL_LOG_CATEGORY_APPLICATION as u32 {
            Category::Application
        } else if value == sys::SDL_LogCategory::SDL_LOG_CATEGORY_ERROR as u32 {
            Category::Error
        } else if value == sys::SDL_LogCategory::SDL_LOG_CATEGORY_ASSERT as u32 {
            Category::Assert
        } else if value == sys::SDL_LogCategory::SDL_LOG_CATEGORY_SYSTEM as u32 {
            Category::System
        } else if value == sys::SDL_LogCategory::SDL_LOG_CATEGORY_AUDIO as u32 {
            Category::Audio
        } else if value == sys::SDL_LogCategory::SDL_LOG_CATEGORY_VIDEO as u32 {
            Category::Video
        } else if value == sys::SDL_LogCategory::SDL_LOG_CATEGORY_RENDER as u32 {
            Category::Render
        } else if value == sys::SDL_LogCategory::SDL_LOG_CATEGORY_INPUT as u32 {
            Category::Input
        } else if value == sys::SDL_LogCategory::SDL_LOG_CATEGORY_TEST as u32 {
            Category::Test
        } else {
            Category::Custom
        }
    }

    fn into_ll(value: Category) -> u32 {
        return match value {
            Category::Application => sys::SDL_LogCategory::SDL_LOG_CATEGORY_APPLICATION as u32,
            Category::Error => sys::SDL_LogCategory::SDL_LOG_CATEGORY_ERROR as u32,
            Category::Assert => sys::SDL_LogCategory::SDL_LOG_CATEGORY_ASSERT as u32,
            Category::System => sys::SDL_LogCategory::SDL_LOG_CATEGORY_SYSTEM as u32,
            Category::Audio => sys::SDL_LogCategory::SDL_LOG_CATEGORY_AUDIO as u32,
            Category::Video => sys::SDL_LogCategory::SDL_LOG_CATEGORY_VIDEO as u32,
            Category::Render => sys::SDL_LogCategory::SDL_LOG_CATEGORY_RENDER as u32,
            Category::Input => sys::SDL_LogCategory::SDL_LOG_CATEGORY_INPUT as u32,
            Category::Test => sys::SDL_LogCategory::SDL_LOG_CATEGORY_TEST as u32,
            Category::Custom => sys::SDL_LogCategory::SDL_LOG_CATEGORY_CUSTOM as u32,
            Category::Unknown => sys::SDL_LogCategory::SDL_LOG_CATEGORY_APPLICATION as u32,
        };
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
    fn from_ll(value: sys::SDL_LogPriority) -> Priority {
        use crate::sys::SDL_LogPriority::*;
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

fn dummy(_priority: Priority, _category: Category, _message: &str) {}

#[allow(non_upper_case_globals)]
// NEVER make this public
static mut custom_log_fn: fn(Priority, Category, &str) = dummy;

unsafe extern "C" fn rust_sdl2_log_fn(
    _userdata: *mut libc::c_void,
    category: libc::c_int,
    priority: sys::SDL_LogPriority,
    message: *const libc::c_char,
) {
    let category = Category::from_ll(category as u32);
    let priority = Priority::from_ll(priority);
    let message = CStr::from_ptr(message).to_string_lossy();
    custom_log_fn(priority, category, &message);
}

#[doc(alias = "SDL_LogSetOutputFunction")]
pub fn set_output_function(callback: fn(Priority, Category, &str)) {
    unsafe {
        custom_log_fn = callback;
        sys::SDL_LogSetOutputFunction(Some(rust_sdl2_log_fn), null_mut());
    };
}

/// Standard log function which takes as priority INFO and
/// as category APPLICATION
#[doc(alias = "SDL_Log")]
pub fn log(message: &str) {
    let message = message.replace('%', "%%");
    let message = CString::new(message).unwrap();
    unsafe {
        crate::sys::SDL_Log(message.as_ptr());
    }
}

/// Critical log function which takes as priority CRITICAL
#[doc(alias = "SDL_LogCritical")]
pub fn log_critical(message: &str, category: Category) {
    unsafe {
        log_with_category(message, category, |category, fmt| {
            crate::sys::SDL_LogCritical(category, fmt)
        });
    }
}

/// Debug log function which takes as priority DEBUG
#[doc(alias = "SDL_LogDebug")]
pub fn log_debug(message: &str, category: Category) {
    unsafe {
        log_with_category(message, category, |category, fmt| {
            crate::sys::SDL_LogDebug(category, fmt)
        });
    }
}

/// Error log function which takes as priority ERROR
#[doc(alias = "SDL_LogError")]
pub fn log_error(message: &str, category: Category) {
    unsafe {
        log_with_category(message, category, |category, fmt| {
            crate::sys::SDL_LogError(category, fmt)
        });
    }
}

/// Info log function which takes as priority INFO
#[doc(alias = "SDL_LogInfo")]
pub fn log_info(message: &str, category: Category) {
    unsafe {
        log_with_category(message, category, |category, fmt| {
            crate::sys::SDL_LogInfo(category, fmt)
        });
    }
}

/// Verbose log function which takes as priority VERBOSE
#[doc(alias = "SDL_LogVerbose")]
pub fn log_verbose(message: &str, category: Category) {
    unsafe {
        log_with_category(message, category, |category, fmt| {
            crate::sys::SDL_LogVerbose(category, fmt)
        });
    }
}

/// Warn log function which takes as priority WARN
#[doc(alias = "SDL_LogWarn")]
pub fn log_warn(message: &str, category: Category) {
    unsafe {
        log_with_category(message, category, |category, fmt| {
            crate::sys::SDL_LogWarn(category, fmt)
        });
    }
}

/// uses the sdl_log_func to log the message, when category cannot be converted, then Category: Application will be used
fn log_with_category(
    message: &str,
    category: Category,
    sdl_log_func: unsafe fn(category: libc::c_int, fmt: *const libc::c_char),
) {
    let message = message.replace('%', "%%");
    let message = CString::new(message).unwrap();
    let uccategory = Category::into_ll(category).try_into();
    let ccategory = match uccategory {
        Err(_) => Category::into_ll(Category::Application).try_into().unwrap(),
        Ok(success) => success,
    };

    unsafe {
        sdl_log_func(ccategory, message.as_ptr());
    }
}
