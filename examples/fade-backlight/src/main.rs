use thinkpad_backlight_api;

fn main() -> Result<(), thinkpad_backlight_api::Error> {
    let levels = [
        thinkpad_backlight_api::LightLevel::Full,
        thinkpad_backlight_api::LightLevel::Off,
    ];

    loop {
        for level in levels.clone().into_iter() {
            thinkpad_backlight_api::KeyboardBacklight::set(level)?;

            let level_status = thinkpad_backlight_api::KeyboardBacklight::get()?;
            println!("Status level: {:?}", level_status);

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}
