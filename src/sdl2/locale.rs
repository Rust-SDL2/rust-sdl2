//! System locale information.

/// A locale defines a user's language and (optionally) region.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Locale {
    pub lang: String,
    pub country: Option<String>,
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lang)?;

        if let Some(region) = &self.country {
            write!(f, "_{}", region)?;
        }

        Ok(())
    }
}

/// Get the user's preferred locales.
///
/// [Official SDL documentation](https://wiki.libsdl.org/SDL_GetPreferredLocales)
///
/// # Example
/// ```
/// let locales = [sdl2::locale::Locale {
///   lang: "en".to_string(),
///   country: Some("US".to_string()),
/// }];
///
/// sdl2::hint::set_preferred_locales(&locales);
///
/// let preferred_locales = sdl2::locale::get_preferred_locales().collect::<Vec<_>>();
/// assert_eq!(preferred_locales, locales);
/// ```
pub fn get_preferred_locales() -> LocaleIterator {
    unsafe {
        LocaleIterator {
            raw: sys::SDL_GetPreferredLocales(),
            index: 0,
        }
    }
}

pub struct LocaleIterator {
    raw: *mut sys::SDL_Locale,
    index: isize,
}

impl Drop for LocaleIterator {
    fn drop(&mut self) {
        unsafe { sys::SDL_free(self.raw as *mut _) }
    }
}

impl Iterator for LocaleIterator {
    type Item = Locale;

    fn next(&mut self) -> Option<Self::Item> {
        let locale = unsafe { get_locale(self.raw.offset(self.index))? };
        self.index += 1;
        Some(locale)
    }
}

unsafe fn get_locale(ptr: *const sys::SDL_Locale) -> Option<Locale> {
    let sdl_locale = ptr.as_ref()?;

    if sdl_locale.language.is_null() {
        return None;
    }
    let lang = std::ffi::CStr::from_ptr(sdl_locale.language)
        .to_string_lossy()
        .into_owned();

    let region = try_get_string(sdl_locale.country);

    Some(Locale {
        lang,
        country: region,
    })
}

unsafe fn try_get_string(ptr: *const libc::c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned())
    }
}
