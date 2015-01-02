pub use sys::joystick as ll;

bitflags! {
    flags HatState: u8 {
        const CENTEREDHATSTATE = 0,
        const UPHATSTATE = 0x01,
        const RIGHTHATSTATE = 0x02,
        const DOWNHATSTATE = 0x04,
        const LEFTHATSTATE = 0x08,
        const RIGHTUPHATSTATE = 0x02 | 0x01,   // RightHatState | UpHatState
        const RIGHTDOWNHATSTATE = 0x02 | 0x04, // RightHatState | DownHatState,
        const LEFTUPHATSTATE = 0x08 | 0x01,    // LeftHatState | UpHatState,
        const LEFTDOWNHATSTATE = 0x08 | 0x04   // LeftHatState | DownHatState
    }
}
