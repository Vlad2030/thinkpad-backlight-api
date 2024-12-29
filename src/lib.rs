use std::io::{Read, Write};

const BRIGHTNESS_PATH: &str = "/sys/class/leds/tpacpi::kbd_backlight/brightness";

#[derive(Clone, Debug)]
pub enum Error {
    PermissionDenied,
    FileNotFound(String),
    IoError(String),
    InvalidLevel(String),
}

#[derive(Clone, Debug)]
pub enum LightLevel {
    Off = 0,
    Medium = 1,
    Full = 2,
}

impl LightLevel {
    pub fn from_u8(level: u8) -> Result<Self, Error> {
        match level {
            0 => Ok(Self::Off),
            1 => Ok(Self::Medium),
            2 => Ok(Self::Full),
            _ => Err(Error::InvalidLevel(format!(
                "Unexpected level: {}",
                level
            ))),
        }
    }

    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

pub struct KeyboardBacklight;

impl KeyboardBacklight {
    fn ensure_root() -> Result<(), Error> {
        if unsafe { libc::getuid() } != 0 {
            return Err(Error::PermissionDenied);
        }
        Ok(())
    }

    pub fn set(level: LightLevel) -> Result<(), Error> {
        Self::ensure_root()?;

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .open(BRIGHTNESS_PATH)
            .map_err(|e| Error::FileNotFound(format!("Cannot open file: {}", e)))?;

        file.write_all(level.as_u8().to_string().as_bytes())
            .map_err(|e| Error::IoError(format!("Failed to set brightness level: {}", e)))?;

        Ok(())
    }

    pub fn get() -> Result<LightLevel, Error> {
        Self::ensure_root()?;

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open(BRIGHTNESS_PATH)
            .map_err(|e| Error::FileNotFound(format!("Cannot open file: {}", e)))?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| Error::IoError(format!("Failed to get brightness level: {}", e)))?;

        let level: u8 = content
            .trim()
            .parse()
            .map_err(|_| Error::InvalidLevel(format!("Invalid brightness value: {}", content)))?;

        LightLevel::from_u8(level)
    }
}
