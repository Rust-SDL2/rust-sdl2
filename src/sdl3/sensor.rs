/// Access to gyroscope and accelerometer on the controller.
///
/// Compatible controllers including Playstation, Switch and Steam controllers include a gyroscope
/// and accelerometer to get the movement in space of the device.
///
/// Units used by SDL:
/// - Accelerometer is in m/s²
/// - Gyroscope is in radian per second
///
/// Axis when holding the controller:
/// - -x ... +x is left ... right
/// - -y ... +y is down ... up
/// - -z ... +z is forward ... backward
///
/// Rotations uses the standard anti-clockwise direction around the corresponding axis from above:
/// - -x ... +x is pitch towards up
/// - -y ... +y is yaw from right to left
/// - -z ... +z is roll from right to left
use crate::sys;

use crate::common::{validate_int, IntegerOrSdlError};
use crate::get_error;
use crate::SensorSubsystem;
use libc::c_char;
use std::ffi::CStr;
use sys::SDL_SensorGetData;
use sys::SDL_SensorType;

impl SensorSubsystem {
    /// Retrieve the total number of attached sensor *and* controllers identified by SDL.
    #[doc(alias = "SDL_NumSensors")]
    pub fn num_sensors(&self) -> Result<u32, String> {
        let result = unsafe { sys::SDL_NumSensors() };

        if result >= 0 {
            Ok(result as u32)
        } else {
            Err(get_error())
        }
    }

    /// Attempt to open the sensor at index `sensor_index` and return it.
    #[doc(alias = "SDL_SensorOpen")]
    pub fn open(&self, sensor_index: u32) -> Result<Sensor, IntegerOrSdlError> {
        use crate::common::IntegerOrSdlError::*;
        let sensor_index = validate_int(sensor_index, "sensor_index")?;

        let sensor = unsafe { sys::SDL_SensorOpen(sensor_index) };

        if sensor.is_null() {
            Err(SdlError(get_error()))
        } else {
            Ok(Sensor {
                subsystem: self.clone(),
                raw: sensor,
            })
        }
    }

    /// Force sensor update when not using the event loop
    #[inline]
    #[doc(alias = "SDL_SensorUpdate")]
    pub fn update(&self) {
        unsafe { sys::SDL_SensorUpdate() };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorType {
    Unknown,
    Gyroscope,
    Accelerometer,
}

impl SensorType {
    pub fn from_ll(raw: i32) -> Self {
        match raw {
            x if x == SDL_SensorType::SDL_SENSOR_GYRO as i32 => SensorType::Gyroscope,
            x if x == SDL_SensorType::SDL_SENSOR_ACCEL as i32 => SensorType::Accelerometer,
            _ => SensorType::Unknown,
        }
    }
}

impl Into<SDL_SensorType> for SensorType {
    fn into(self) -> SDL_SensorType {
        match self {
            SensorType::Unknown => SDL_SensorType::SDL_SENSOR_UNKNOWN,
            SensorType::Gyroscope => SDL_SensorType::SDL_SENSOR_GYRO,
            SensorType::Accelerometer => SDL_SensorType::SDL_SENSOR_ACCEL,
        }
    }
}

/// Wrapper around the `SDL_Sensor` object
pub struct Sensor {
    subsystem: SensorSubsystem,
    raw: *mut sys::SDL_Sensor,
}

impl Sensor {
    #[inline]
    pub const fn subsystem(&self) -> &SensorSubsystem {
        &self.subsystem
    }

    /// Return the name of the sensor or an empty string if no name
    /// is found.
    #[doc(alias = "SDL_SensorGetName")]
    pub fn name(&self) -> String {
        let name = unsafe { sys::SDL_SensorGetName(self.raw) };

        c_str_to_string(name)
    }

    #[doc(alias = "SDL_SensorGetInstanceID")]
    pub fn instance_id(&self) -> u32 {
        let result = unsafe { sys::SDL_SensorGetInstanceID(self.raw) };

        if result < 0 {
            // Should only fail if the joystick is NULL.
            panic!("{}", get_error())
        } else {
            result as u32
        }
    }

    /// Return the type of the sensor or `Unknown` if unsupported.
    #[doc(alias = "SDL_SensorGetType")]
    pub fn sensor_type(&self) -> SensorType {
        let result = unsafe { sys::SDL_SensorGetType(self.raw) };

        match result {
            sys::SDL_SensorType::SDL_SENSOR_INVALID => {
                panic!("{}", get_error())
            }
            sys::SDL_SensorType::SDL_SENSOR_UNKNOWN => SensorType::Unknown,
            sys::SDL_SensorType::SDL_SENSOR_ACCEL => SensorType::Accelerometer,
            sys::SDL_SensorType::SDL_SENSOR_GYRO => SensorType::Gyroscope,
        }
    }

    /// Get the current data from the sensor.
    ///
    /// Output depends on the type of the sensor. See module documentation for units and axis.
    #[doc(alias = "SDL_SensorGetType")]
    pub fn get_data(&self) -> Result<SensorData, IntegerOrSdlError> {
        let mut data = [0f32; 16];
        let result = unsafe { SDL_SensorGetData(self.raw, data.as_mut_ptr(), data.len() as i32) };

        if result != 0 {
            Err(IntegerOrSdlError::SdlError(get_error()))
        } else {
            Ok(match self.sensor_type() {
                SensorType::Gyroscope => SensorData::Accel([data[0], data[1], data[2]]),
                SensorType::Accelerometer => SensorData::Accel([data[0], data[1], data[2]]),
                SensorType::Unknown => SensorData::Unknown(data),
            })
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SensorData {
    Gyro([f32; 3]),
    Accel([f32; 3]),
    Unknown([f32; 16]),
}

impl Drop for Sensor {
    #[doc(alias = "SDL_SensorClose")]
    fn drop(&mut self) {
        unsafe { sys::SDL_SensorClose(self.raw) }
    }
}

/// Convert C string `c_str` to a String. Return an empty string if
/// `c_str` is NULL.
fn c_str_to_string(c_str: *const c_char) -> String {
    if c_str.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(c_str as *const _)
                .to_str()
                .unwrap()
                .to_owned()
        }
    }
}
