use crate::pixels;
use crate::render::TextureAccess;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextureQuery {
    pub format: pixels::PixelFormatEnum,
    pub access: TextureAccess,
    pub width: u32,
    pub height: u32,
}
