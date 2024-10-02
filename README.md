# rgpio - Rust GPIO Library for Raspberry Pi

A modern, safe Rust implementation for GPIO control on Raspberry Pi using the Linux sysfs interface. This project
provides a simple and efficient way to interact with GPIO pins without relying on deprecated libraries like wiringPi.

## Overview

The project consists of two main components:
- `rgpiolib`: A library crate providing the core GPIO functionality
- `rgpio`: A command-line tool demonstrating the library's usage

## Features

- Export and unexport GPIO pins
- Set GPIO pin directions (input/output)
- Write digital signals (high/low)
- Read pin states
- Error handling with custom GPIO error types
- Safe file operations using Rust's type system

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rgpiolib = { git = "https://github.com/jibbex/rgpio" }
```

## Usage

### Library Usage

```rust
use rgpiolib::gpio;

fn main() -> Result<(), Box<dyn Error>> {
    let gpio_num = 4;
    
    // Export the GPIO pin
    gpio::export(gpio_num)?;
    
    // Set it as output
    gpio::set_direction(gpio_num, gpio::Directions::Output)?;
    
    // Write a high signal
    gpio::write(gpio_num, true)?;
    
    // Read the current state
    let value = gpio::read(gpio_num)?;
    println!("Pin state: {}", value);
    
    // Clean up
    gpio::unexport(gpio_num)?;
    
    Ok(())
}
```

### Command-line Tool

The `rgpio` binary allows you to test GPIO pins from the command line:

```bash
rgpio 18 23 24  # Test multiple GPIO pins
```

## API Reference

| Function      | Arguments                              | Return         | Description                         |
|---------------|----------------------------------------|----------------|-------------------------------------|
| export        | `gpio_num: i32`                        | `Result<()>`   | Enables GPIO pin                    |
| unexport      | `gpio_num: i32`                        | `Result<()>`   | Disables GPIO pin                   |
| write         | `gpio_num: i32, signal: bool`          | `Result<()>`   | Sets GPIO pin high/low              |
| read          | `gpio_num: i32`                        | `Result<bool>` | Reads current GPIO pin state        |
| set_direction | `gpio_num: i32, direction: Directions` | `Result<()>`   | Configures GPIO pin as input/output |

## Requirements

- Raspberry Pi running Linux
- Rust 2021 edition or newer
- Appropriate permissions to access `/sys/class/gpio`

## Safety Note

GPIO operations typically require root privileges. Make sure to run your application with appropriate permissions or 
using `sudo` when necessary.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments

This project was created as a modern Rust alternative to the now-deprecated wiringPi library, originally developed as 
a teaching tool for apprentices learning system programming.