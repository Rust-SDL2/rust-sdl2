use std::ffi::CStr;

use crate::pixels::PixelFormatEnum;

/// A structure that contains information on the capabilities of a render driver
/// or the current render context.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct RendererInfo {
    pub name: &'static str,
    pub flags: u32,
    pub texture_formats: Vec<PixelFormatEnum>,
    pub max_texture_width: u32,
    pub max_texture_height: u32,
}

impl RendererInfo {
    pub unsafe fn from_ll(info: &sys::SDL_RendererInfo) -> RendererInfo {
        use std::convert::TryFrom;
        let texture_formats: Vec<PixelFormatEnum> = info.texture_formats
            [0..(info.num_texture_formats as usize)]
            .iter()
            .map(|&format| {
                PixelFormatEnum::try_from(format as u32).unwrap_or(PixelFormatEnum::Unknown)
            })
            .collect();

        // The driver name is always a static string, compiled into SDL2.
        let name = CStr::from_ptr(info.name as *const _).to_str().unwrap();

        RendererInfo {
            name,
            flags: info.flags,
            texture_formats,
            max_texture_width: info.max_texture_width as u32,
            max_texture_height: info.max_texture_height as u32,
        }
    }
}
