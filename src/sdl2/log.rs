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

/// Log function which takes as priority CRITICAL and category APPLICATION
#[doc(alias = "SDL_LogCritial")]
pub fn log_critical(message: &str) {
    log_with_category(message, Category::Application, Priority::Critical);
}

/// Log function which takes as priority DEBUG and category APPLICATION
#[doc(alias = "SDL_LogDebug")]
pub fn log_debug(message: &str) {
    log_with_category(message, Category::Application, Priority::Debug);
}

/// Log function which takes as priority ERROR and category APPLICATION
#[doc(alias = "SDL_LogError")]
pub fn log_error(message: &str) {
    log_with_category(message, Category::Application, Priority::Error);
}

/// Log function which takes as priority INFO and category APPLICATION
#[doc(alias = "SDL_LogInfo")]
pub fn log_info(message: &str) {
    log_with_category(message, Category::Application, Priority::Info);
}

/// Log function which takes as priority VERBOSE and category APPLICATION
#[doc(alias = "SDL_LogVerbose")]
pub fn log_verbose(message: &str) {
    log_with_category(message, Category::Application, Priority::Verbose);
}

/// Log function which takes as priority WARN and category APPLICATION
#[doc(alias = "SDL_LogWarn")]
pub fn log_warn(message: &str) {
    log_with_category(message, Category::Application, Priority::Warn);
}

/// Log function where Category and Priority can be specified
pub fn log_with_category(message: &str, category: Category, priority: Priority) {
    let message = message.replace('%', "%%");
    let message = CString::new(message).unwrap();
    let uccategory = Category::into_ll(category).try_into();
    let ccategory = match uccategory {
        Err(_) => Category::into_ll(Category::Application).try_into().unwrap(),
        Ok(success) => success,
    };

    unsafe {
        match priority {
            Priority::Critical => crate::sys::SDL_LogCritical(ccategory, message.as_ptr()),
            Priority::Debug => crate::sys::SDL_LogDebug(ccategory, message.as_ptr()),
            Priority::Error => crate::sys::SDL_LogError(ccategory, message.as_ptr()),
            Priority::Info => crate::sys::SDL_LogInfo(ccategory, message.as_ptr()),
            Priority::Verbose => crate::sys::SDL_LogVerbose(ccategory, message.as_ptr()),
            Priority::Warn => crate::sys::SDL_LogWarn(ccategory, message.as_ptr()),
        }
    }
}
