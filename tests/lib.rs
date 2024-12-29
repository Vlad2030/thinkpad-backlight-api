use thinkpad_backlight_api;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_level() {
        let level = thinkpad_backlight_api::KeyboardBacklight::set(
            thinkpad_backlight_api::LightLevel::Off,
        );

        assert!(level.is_ok())
    }

    #[test]
    fn get_level() {
        let level = thinkpad_backlight_api::KeyboardBacklight::get();

        assert!(level.is_ok())
    }
}
