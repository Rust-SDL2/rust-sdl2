#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum FullscreenType {
    Off = 0,
    True = 0x00_00_00_01,
    Desktop = 0x00_00_10_01,
}

impl FullscreenType {
    pub fn from_window_flags(window_flags: u32) -> FullscreenType {
        if window_flags & FullscreenType::Desktop as u32 == FullscreenType::Desktop as u32 {
            FullscreenType::Desktop
        } else if window_flags & FullscreenType::True as u32 == FullscreenType::True as u32 {
            FullscreenType::True
        } else {
            FullscreenType::Off
        }
    }
}
