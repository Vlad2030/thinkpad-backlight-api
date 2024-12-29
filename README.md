# thinkpad-backlight-api

Keyboard backlight API for thinkpad. Implemented on Rust

# How to use

## Add a crate

Add this to your `Cargo.toml`:

```toml
thinkpad-backlight-api = "0.1.0"
```

## Example usage

```rust
use thinkpad_backlight_api::{KeyboardBacklight, LightLevel};

fn main() {
    if let Err(e) = KeyboardBacklight::set(LightLevel::Full) {
        println!("Error: {:?}", e)
    }

    match KeyboardBacklight::get() {
        Ok(level) => println!("Level: {:?}", level),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

## Tested on

- Lenovo thinkpad 480s (Ubuntu 24.04)
