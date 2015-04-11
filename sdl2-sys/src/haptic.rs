use joystick::SDL_Joystick;
#[cfg(feature = "no_std")]
use core::prelude::*;
use libc::{c_int, c_uint, c_char, c_float, c_void, int16_t, int32_t, uint8_t, uint16_t, uint32_t};

pub const SDL_HAPTIC_CONSTANT: uint16_t = 1 << 0;
pub const SDL_HAPTIC_SINE: uint16_t = 1 << 1;
pub const SDL_HAPTIC_LEFTRIGHT: uint16_t = 1 << 2;
pub const SDL_HAPTIC_TRIANGLE: uint16_t = 1 << 3;
pub const SDL_HAPTIC_SAWTOOTHUP: uint16_t = 1 << 4;
pub const SDL_HAPTIC_SAWTOOTHDOWN: uint16_t = 1 << 5;
pub const SDL_HAPTIC_RAMP: uint16_t = 1 << 6;
pub const SDL_HAPTIC_SPRING: uint16_t = 1 << 7;
pub const SDL_HAPTIC_DAMPER: uint16_t = 1 << 8;
pub const SDL_HAPTIC_INERTIA: uint16_t = 1 << 9;
pub const SDL_HAPTIC_FRICTION: uint16_t = 1 << 10;
pub const SDL_HAPTIC_CUSTOM: uint16_t = 1 << 11;
pub const SDL_HAPTIC_GAIN: uint16_t = 1 << 12;
pub const SDL_HAPTIC_AUTOCENTER: uint16_t = 1 << 13;
pub const SDL_HAPTIC_STATUS: uint16_t = 1 << 14;
pub const SDL_HAPTIC_PAUSE: uint16_t = 1 << 15;

pub type SDL_Haptic = c_void;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_HapticDirection {
    pub type_: uint8_t,
    pub dir: [int32_t; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_HapticConstant {
    pub type_: uint16_t,
    pub direction: SDL_HapticDirection,
    pub length: uint32_t,
    pub delay: uint16_t,
    pub button: uint16_t,
    pub interval: uint16_t,
    pub level: int16_t,
    pub attack_length: uint16_t,
    pub attack_level: uint16_t,
    pub fade_length: uint16_t,
    pub fade_level: uint16_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_HapticPeriodic {
    pub type_: uint16_t,
    pub direction: SDL_HapticDirection,
    pub length: uint32_t,
    pub delay: uint16_t,
    pub button: uint16_t,
    pub interval: uint16_t,
    pub period: uint16_t,
    pub magnitude: int16_t,
    pub offset: int16_t,
    pub phase: uint16_t,
    pub attack_length: uint16_t,
    pub attack_level: uint16_t,
    pub fade_length: uint16_t,
    pub fade_level: uint16_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_HapticCondition {
    pub type_: uint16_t,
    pub direction: SDL_HapticDirection,
    pub length: uint32_t,
    pub delay: uint16_t,
    pub button: uint16_t,
    pub interval: uint16_t,
    pub right_sat: [uint16_t; 3],
    pub left_sat: [uint16_t; 3],
    pub right_coeff: [int16_t; 3],
    pub left_coeff: [int16_t; 3],
    pub deadband: [uint16_t; 3],
    pub center: [int16_t; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_HapticRamp {
    pub type_: uint16_t,
    pub length: uint32_t,
    pub delay: uint16_t,
    pub button: uint16_t,
    pub interval: uint16_t,
    pub start: int16_t,
    pub end: int16_t,
    pub attack_length: uint16_t,
    pub attack_level: uint16_t,
    pub fade_length: uint16_t,
    pub fade_level: uint16_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_HapticLeftRight {
    pub type_: uint16_t,
    pub length: uint32_t,
    pub large_magnitude: uint16_t,
    pub small_magnitude: uint16_t,
}

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_HapticCustom {
    pub type_: uint16_t,
    pub direction: SDL_HapticDirection,
    pub length: uint32_t,
    pub delay: uint16_t,
    pub button: uint16_t,
    pub interval: uint16_t,
    pub channels: uint8_t,
    pub period: uint16_t,
    pub samples: uint16_t,
    pub data: *const uint16_t,
    pub attack_length: uint16_t,
    pub attack_level: uint16_t,
    pub fade_length: uint16_t,
    pub fade_level: uint16_t,
}

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_HapticEffect {
    pub data: [uint8_t; 72],
}

impl SDL_HapticEffect {
    pub fn type_(&mut self) -> *mut uint16_t {
        self.data.as_mut_ptr() as *mut _
    }

    pub fn constant(&mut self) -> *mut SDL_HapticConstant {
        self.data.as_mut_ptr() as *mut _
    }

    pub fn periodic(&mut self) -> *mut SDL_HapticPeriodic {
        self.data.as_mut_ptr() as *mut _
    }

    pub fn condition(&mut self) -> *mut SDL_HapticCondition {
        self.data.as_mut_ptr() as *mut _
    }

    pub fn ramp(&mut self) -> *mut SDL_HapticRamp {
        self.data.as_mut_ptr() as *mut _
    }

    pub fn left_right(&mut self) -> *mut SDL_HapticLeftRight {
        self.data.as_mut_ptr() as *mut _
    }

    pub fn custom(&mut self) -> *mut SDL_HapticCustom {
        self.data.as_mut_ptr() as *mut _
    }
}

extern "C" {
    pub fn SDL_NumHaptics() -> c_int;
    pub fn SDL_HapticName(device_index: c_int) -> *const c_char;
    pub fn SDL_HapticOpen(device_index: c_int) -> *mut SDL_Haptic;
    pub fn SDL_HapticOpened(device_index: c_int) -> c_int;
    pub fn SDL_HapticIndex(haptic: *mut SDL_Haptic) -> c_int;
    pub fn SDL_MouseIsHaptic() -> c_int;
    pub fn SDL_HapticOpenFromMouse() -> *mut SDL_Haptic;
    pub fn SDL_JoystickIsHaptic(joystick: *mut SDL_Joystick) -> c_int;
    pub fn SDL_HapticOpenFromJoystick(joystick: *mut SDL_Joystick) -> *mut SDL_Haptic;
    pub fn SDL_HapticClose(haptic: *mut SDL_Haptic);
    pub fn SDL_HapticNumEffects(haptic: *mut SDL_Haptic) -> c_int;
    pub fn SDL_HapticNumEffectsPlaying(haptic: *mut SDL_Haptic) -> c_int;
    pub fn SDL_HapticQuery(haptic: *mut SDL_Haptic) -> c_uint;
    pub fn SDL_HapticNumAxes(haptic: *mut SDL_Haptic) -> c_int;
    pub fn SDL_HapticEffectSupported(haptic: *mut SDL_Haptic, effect: *mut SDL_HapticEffect) -> c_int;
    pub fn SDL_HapticNewEffect(haptic: *mut SDL_Haptic, effect: *mut SDL_HapticEffect) -> c_int;
    pub fn SDL_HapticUpdateEffect(haptic: *mut SDL_Haptic, effect: *mut SDL_HapticEffect) -> c_int;
    pub fn SDL_HapticRunEffect(haptic: *mut SDL_Haptic, effect: c_int, iterations: uint32_t) -> c_int;
    pub fn SDL_HapticStopEffect(haptic: *mut SDL_Haptic, effect: c_int) -> c_int;
    pub fn SDL_HapticDestroyEffect(haptic: *mut SDL_Haptic, effect: c_int);
    pub fn SDL_HapticGetEffectStatus(haptic: *mut SDL_Haptic, effect: c_int) -> c_int;
    pub fn SDL_HapticSetGain(haptic: *mut SDL_Haptic, gain: c_int) -> c_int;
    pub fn SDL_HapticSetAutocenter(haptic: *mut SDL_Haptic, autocenter: c_int) -> c_int;
    pub fn SDL_HapticPause(haptic: *mut SDL_Haptic) -> c_int;
    pub fn SDL_HapticUnpause(haptic: *mut SDL_Haptic) -> c_int;
    pub fn SDL_HapticStopAll(haptic: *mut SDL_Haptic) -> c_int;
    pub fn SDL_HapticRumbleSupported(haptic: *mut SDL_Haptic) -> c_int;
    pub fn SDL_HapticRumbleInit(haptic: *mut SDL_Haptic) -> c_int;
    pub fn SDL_HapticRumblePlay(haptic: *mut SDL_Haptic, strength: c_float, length: uint32_t) -> c_int;
    pub fn SDL_HapticRumbleStop(haptic: *mut SDL_Haptic) -> c_int;
}
