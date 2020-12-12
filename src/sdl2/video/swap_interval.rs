/// Represents a setting for vsync/swap interval.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(i32)]
pub enum SwapInterval {
    Immediate = 0,
    VSync = 1,
    LateSwapTearing = -1,
}

impl From<i32> for SwapInterval {
    fn from(i: i32) -> Self {
        match i {
            -1 => SwapInterval::LateSwapTearing,
            0 => SwapInterval::Immediate,
            1 => SwapInterval::VSync,
            other => panic!(
                "Invalid value for SwapInterval: {}; valid values are -1, 0, 1",
                other
            ),
        }
    }
}
