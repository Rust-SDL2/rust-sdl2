/// This is represented in SDL2 as a bitfield but obviously not all
/// combinations make sense: 5 for instance would mean up and down at
/// the same time... To simplify things I turn it into an enum which
/// is how the SDL2 docs present it anyway (using macros).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum HatState {
    Centered = 0,
    Up = 0x01,
    Right = 0x02,
    Down = 0x04,
    Left = 0x08,
    RightUp = 0x02 | 0x01,
    RightDown = 0x02 | 0x04,
    LeftUp = 0x08 | 0x01,
    LeftDown = 0x08 | 0x04,
}

impl HatState {
    pub fn from_raw(raw: u8) -> HatState {
        match raw {
            0 => HatState::Centered,
            1 => HatState::Up,
            2 => HatState::Right,
            4 => HatState::Down,
            8 => HatState::Left,
            3 => HatState::RightUp,
            6 => HatState::RightDown,
            9 => HatState::LeftUp,
            12 => HatState::LeftDown,

            // The Xinput driver on Windows can report hat states on certain hardware that don't
            // make any sense from a gameplay perspective, and so aren't worth putting in the
            // HatState enumeration.
            _ => HatState::Centered,
        }
    }

    pub fn to_raw(self) -> u8 {
        match self {
            HatState::Centered => 0,
            HatState::Up => 1,
            HatState::Right => 2,
            HatState::Down => 4,
            HatState::Left => 8,
            HatState::RightUp => 3,
            HatState::RightDown => 6,
            HatState::LeftUp => 9,
            HatState::LeftDown => 12,
        }
    }
}
