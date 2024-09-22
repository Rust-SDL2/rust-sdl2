use crate::{locale::Locale, sys};
use libc::c_char;
use std::ffi::{CStr, CString};

const VIDEO_MINIMIZE_ON_FOCUS_LOSS: &str = "SDL_VIDEO_MINIMIZE_ON_FOCUS_LOSS";
const PREFERRED_LOCALES: &str = "SDL_PREFERRED_LOCALES";

pub enum Hint {
    Default,
    Normal,
    Override,
}

/// A hint that specifies whether a fullscreen [Window](../video/Window.t.html) will be
/// minimized if key focus is lost.
///
/// [Official SDL documentation](https://wiki.libsdl.org/SDL_HINT_VIDEO_MINIMIZE_ON_FOCUS_LOSS)
///
/// # Default
/// This is enabled by default.
///
/// # Example
/// ```rust,no_run
/// sdl2::hint::set_video_minimize_on_focus_loss(false);
/// ```
///
/// * `value`: `true` to enable minimizing of the Window if it loses key focus when in fullscreen mode,
///            `false` to disable this feature.
pub fn set_video_minimize_on_focus_loss(value: bool) -> bool {
    set(VIDEO_MINIMIZE_ON_FOCUS_LOSS, if value { "1" } else { "0" })
}

/// A hint that specifies whether a fullscreen [Window](../video/Window.t.html) will be
/// minimized if key focus is lost.
///
/// [Official SDL documentation](https://wiki.libsdl.org/SDL_HINT_VIDEO_MINIMIZE_ON_FOCUS_LOSS)
///
/// # Example
/// ```rust,no_run
/// sdl2::hint::set_video_minimize_on_focus_loss_with_priority(false, &sdl2::hint::Hint::Override);
/// ```
///
/// * `value`: `true` to enable minimizing of the Window if it loses key focus when in fullscreen mode,
///            `false` to disable this feature.
/// * `priority`: The priority controls the behavior when setting a hint that already has a value.
///               Hints will replace existing hints of their priority and lower.
///               Environment variables are considered to have override priority.
pub fn set_video_minimize_on_focus_loss_with_priority(value: bool, priority: &Hint) -> bool {
    set_with_priority(
        VIDEO_MINIMIZE_ON_FOCUS_LOSS,
        if value { "1" } else { "0" },
        priority,
    )
}

/// A hint that specifies whether a fullscreen [Window](../video/Window.t.html) will be
/// minimized if key focus is lost.
///
/// [Official SDL documentation](https://wiki.libsdl.org/SDL_HINT_VIDEO_MINIMIZE_ON_FOCUS_LOSS)
///
/// # Default
/// By default this will return `true`.
///
/// # Example
/// ```rust,no_run
/// assert_eq!(sdl2::hint::get_video_minimize_on_focus_loss(), true);
///
/// sdl2::hint::set_video_minimize_on_focus_loss(false);
/// assert_eq!(sdl2::hint::get_video_minimize_on_focus_loss(), false);
/// ```
pub fn get_video_minimize_on_focus_loss() -> bool {
    matches!(
        get(VIDEO_MINIMIZE_ON_FOCUS_LOSS).as_deref(),
        Some("1") | None
    )
}

/// A hint that overrides the user's locale settings.
///
/// [Official SDL documentation](https://wiki.libsdl.org/SDL2/SDL_HINT_PREFERRED_LOCALES)
///
/// # Default
/// This is disabled by default.
///
/// # Example
///
/// See [`crate::locale::get_preferred_locales`].
pub fn set_preferred_locales<T: std::borrow::Borrow<Locale>>(
    locales: impl IntoIterator<Item = T>,
) -> bool {
    set(PREFERRED_LOCALES, &format_locale_hint(locales))
}

fn format_locale_hint<T: std::borrow::Borrow<Locale>>(
    locales: impl IntoIterator<Item = T>,
) -> String {
    use std::fmt::Write;

    let mut iter = locales.into_iter();
    let (reserve, _) = iter.size_hint();
    // Assuming that most locales will be of the form "xx_yy",
    // plus 1 char for the comma.
    let mut formatted = String::with_capacity(reserve * 6);

    if let Some(first) = iter.next() {
        write!(formatted, "{}", first.borrow()).ok();
    }

    for locale in iter {
        write!(formatted, ",{}", locale.borrow()).ok();
    }

    formatted
}

#[doc(alias = "SDL_SetHint")]
pub fn set(name: &str, value: &str) -> bool {
    let name = CString::new(name).unwrap();
    let value = CString::new(value).unwrap();
    unsafe {
        sys::SDL_SetHint(
            name.as_ptr() as *const c_char,
            value.as_ptr() as *const c_char,
        ) == sys::SDL_bool::SDL_TRUE
    }
}

#[doc(alias = "SDL_GetHint")]
pub fn get(name: &str) -> Option<String> {
    use std::str;

    let name = CString::new(name).unwrap();

    unsafe {
        let res = sys::SDL_GetHint(name.as_ptr() as *const c_char);

        if res.is_null() {
            None
        } else {
            Some(
                str::from_utf8(CStr::from_ptr(res as *const _).to_bytes())
                    .unwrap()
                    .to_owned(),
            )
        }
    }
}

#[doc(alias = "SDL_SetHintWithPriority")]
pub fn set_with_priority(name: &str, value: &str, priority: &Hint) -> bool {
    let name = CString::new(name).unwrap();
    let value = CString::new(value).unwrap();

    let priority_val = match *priority {
        Hint::Normal => sys::SDL_HintPriority::SDL_HINT_NORMAL,
        Hint::Override => sys::SDL_HintPriority::SDL_HINT_OVERRIDE,
        Hint::Default => sys::SDL_HintPriority::SDL_HINT_DEFAULT,
    };

    unsafe {
        sys::SDL_SetHintWithPriority(
            name.as_ptr() as *const c_char,
            value.as_ptr() as *const c_char,
            priority_val,
        ) == sys::SDL_bool::SDL_TRUE
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn locale() {
        // Test set_preferred_locales
        let locales = [Locale {
            lang: "en".to_string(),
            country: Some("US".to_string()),
        }];
        set_preferred_locales(&locales);
        set_preferred_locales(locales);

        // Test hint formatting
        assert_eq!(format_locale_hint(&[]), "");

        assert_eq!(
            format_locale_hint([Locale {
                lang: "en".to_string(),
                country: None,
            }]),
            "en"
        );

        assert_eq!(
            format_locale_hint([Locale {
                lang: "en".to_string(),
                country: Some("US".to_string()),
            }]),
            "en_US"
        );

        assert_eq!(
            format_locale_hint([
                Locale {
                    lang: "en".to_string(),
                    country: Some("US".to_string()),
                },
                Locale {
                    lang: "fr".to_string(),
                    country: Some("FR".to_string()),
                },
            ]),
            "en_US,fr_FR"
        );
    }
}
