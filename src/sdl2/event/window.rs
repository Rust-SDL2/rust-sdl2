#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
/// An enum of window events.
pub enum WindowEvent {
    None,
    Shown,
    Hidden,
    Exposed,
    Moved(i32, i32),
    Resized(i32, i32),
    SizeChanged(i32, i32),
    Minimized,
    Maximized,
    Restored,
    Enter,
    Leave,
    FocusGained,
    FocusLost,
    Close,
    TakeFocus,
    HitTest,
}

impl WindowEvent {
    #[allow(clippy::match_same_arms)]
    pub(super) fn from_ll(id: u8, data1: i32, data2: i32) -> WindowEvent {
        match id {
            0 => WindowEvent::None,
            1 => WindowEvent::Shown,
            2 => WindowEvent::Hidden,
            3 => WindowEvent::Exposed,
            4 => WindowEvent::Moved(data1, data2),
            5 => WindowEvent::Resized(data1, data2),
            6 => WindowEvent::SizeChanged(data1, data2),
            7 => WindowEvent::Minimized,
            8 => WindowEvent::Maximized,
            9 => WindowEvent::Restored,
            10 => WindowEvent::Enter,
            11 => WindowEvent::Leave,
            12 => WindowEvent::FocusGained,
            13 => WindowEvent::FocusLost,
            14 => WindowEvent::Close,
            15 => WindowEvent::TakeFocus,
            16 => WindowEvent::HitTest,
            _ => WindowEvent::None,
        }
    }

    pub(super) fn to_ll(&self) -> (u8, i32, i32) {
        match *self {
            WindowEvent::None => (0, 0, 0),
            WindowEvent::Shown => (1, 0, 0),
            WindowEvent::Hidden => (2, 0, 0),
            WindowEvent::Exposed => (3, 0, 0),
            WindowEvent::Moved(d1, d2) => (4, d1, d2),
            WindowEvent::Resized(d1, d2) => (5, d1, d2),
            WindowEvent::SizeChanged(d1, d2) => (6, d1, d2),
            WindowEvent::Minimized => (7, 0, 0),
            WindowEvent::Maximized => (8, 0, 0),
            WindowEvent::Restored => (9, 0, 0),
            WindowEvent::Enter => (10, 0, 0),
            WindowEvent::Leave => (11, 0, 0),
            WindowEvent::FocusGained => (12, 0, 0),
            WindowEvent::FocusLost => (13, 0, 0),
            WindowEvent::Close => (14, 0, 0),
            WindowEvent::TakeFocus => (15, 0, 0),
            WindowEvent::HitTest => (16, 0, 0),
        }
    }

    pub fn is_same_kind_as(&self, other: &WindowEvent) -> bool {
        match (self, other) {
            (Self::None, Self::None)
            | (Self::Shown, Self::Shown)
            | (Self::Hidden, Self::Hidden)
            | (Self::Exposed, Self::Exposed)
            | (Self::Moved(_, _), Self::Moved(_, _))
            | (Self::Resized(_, _), Self::Resized(_, _))
            | (Self::SizeChanged(_, _), Self::SizeChanged(_, _))
            | (Self::Minimized, Self::Minimized)
            | (Self::Maximized, Self::Maximized)
            | (Self::Restored, Self::Restored)
            | (Self::Enter, Self::Enter)
            | (Self::Leave, Self::Leave)
            | (Self::FocusGained, Self::FocusGained)
            | (Self::FocusLost, Self::FocusLost)
            | (Self::Close, Self::Close)
            | (Self::TakeFocus, Self::TakeFocus)
            | (Self::HitTest, Self::HitTest) => true,
            _ => false,
        }
    }
}
