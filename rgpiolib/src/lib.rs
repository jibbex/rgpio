/// GPIO library for Rust
///
/// This library provides a simple interface to interact with GPIO pins on a Raspberry Pi.
///
/// # Examples
///
/// ```rust
/// use rgpiolib::gpio;
///
/// fn main() {
///     let gpio_num = 4;
///     gpio::export(gpio_num);
///     gpio::set_direction(gpio_num, gpio::Directions::Output);
///     gpio::write(gpio_num, true);
///     let val = gpio::read(gpio_num);
///     gpio::unexport(gpio_num);
/// }
/// ```
///
/// # Note
///
/// This library is intended to be used on a Raspberry Pi running a Linux-based OS.
///
/// # Features
///
/// - Export GPIO pins
/// - Unexport GPIO pins
/// - Set GPIO pin direction
/// - Write to GPIO pins
/// - Read from GPIO pins
///
/// # References
///
/// - [Raspberry Pi GPIO](https://www.raspberrypi.org/documentation/usage/gpio/)
/// - [Linux GPIO](https://www.kernel.org/doc/Documentation/gpio/)
pub mod gpio {
    use std::{fmt, fs};
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    /// GPIO paths
    ///
    /// This enum represents the paths to the GPIO files.
    enum GpioPaths {
        /// Export GPIO pin
        EXPORT,
        /// Unexport GPIO pin
        UNEXPORT,
        /// Value of GPIO pin
        VALUE(i32),
        /// Direction of GPIO pin
        DIRECTION(i32),
    }

    /// Implement the `as_str` method for the `GpioPaths` enum.
    impl GpioPaths {
        /// Returns the path as a string slice.
        pub fn as_str(&self) -> &'static str {
            match *self {
                /// Path to export GPIO pin
                GpioPaths::EXPORT => "/sys/class/gpio/export",
                /// Path to unexport GPIO pin
                GpioPaths::UNEXPORT => "/sys/class/gpio/unexport",
                /// Path to value of GPIO pin
                GpioPaths::VALUE(num) => {
                    let path = format!("/sys/class/gpio/gpio{}/value", num);
                    Box::leak(path.into_boxed_str())
                },
                /// Path to direction of GPIO pin
                GpioPaths::DIRECTION(num) => {
                    let path = format!("/sys/class/gpio/gpio{}/direction", num);
                    Box::leak(path.into_boxed_str())
                },
            }
        }
    }

    /// GPIO directions
    ///
    /// This enum represents the directions of a GPIO pin.
    ///
    /// - Input
    /// - Output
    pub type Directions = directions::Directions;

    /// Implement the `as_str` method for the `Directions` enum.
    pub mod directions {
        /// GPIO directions
        ///
        /// This enum represents the directions of a GPIO pin.
        pub enum Directions {
            Input,
            Output,
        }

        /// Implement the `as_str` method for the `Directions` enum.
        impl Directions {
            /// Returns the direction as a string slice.
            pub fn as_str(&self) -> &'static str {
                match *self {
                    Directions::Input => "in",
                    Directions::Output => "out",
                }
            }

            /// Returns the direction as a byte slice.
            pub fn as_bytes(&self) -> &[u8] {
                self.as_str().as_bytes()
            }
        }
    }

    /// GPIO errors
    ///
    /// This enum represents the errors that can occur when interacting with GPIO pins.
    ///
    /// - IO error
    /// - ParseInt error
    #[derive(Debug)]
    pub enum GpioError {
        Io(std::io::Error),
        ParseInt(std::num::ParseIntError),
    }

    /// Implement the `Display` trait for the `GpioError` enum.
    ///
    /// This trait allows the `GpioError` enum to be formatted using the `format!` macro.
    impl fmt::Display for GpioError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                GpioError::Io(ref err) => write!(f, "IO error: {}", err),
                GpioError::ParseInt(ref err) => write!(f, "ParseInt error: {}", err),
            }
        }
    }

    /// Implement the `From<std::io::Error>` trait for the `GpioError` enum.
    impl From<std::io::Error> for GpioError {
        fn from(err: std::io::Error) -> GpioError {
            GpioError::Io(err)
        }
    }

    /// Implement the `From<std::num::ParseIntError>` trait for the `GpioError` enum.
    impl From<std::num::ParseIntError> for GpioError {
        fn from(err: std::num::ParseIntError) -> GpioError {
            GpioError::ParseInt(err)
        }
    }

    /// GPIO result
    ///
    /// This type alias represents the result of a GPIO operation.
    type GpioResult<T> = Result<T, GpioError>;

    /// Open file
    ///
    /// This function opens a file and returns a file handle.
    ///
    /// # Arguments
    ///
    /// - `filepath` - A string slice that represents the path to the file.
    ///
    /// # Returns
    ///
    /// A `GpioResult` that contains a file handle.
    fn open_file(filepath: &'static str) -> GpioResult<File> {
        let path = Path::new(&filepath);
        Ok(fs::OpenOptions::new().write(true).open(path)?)
    }

    /// Export GPIO pin
    ///
    /// This function exports (enables) a GPIO pin. The GPIO pin is
    /// passed as an argument. The function writes the GPIO pin to
    /// the `/sys/class/gpio/export` file.
    ///
    ///
    /// # Arguments
    ///
    /// - `gpio_num` - An integer that represents the GPIO pin number.
    ///
    /// # Returns
    ///
    /// A `GpioResult` that contains the result of the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rgpiolib::gpio;
    ///
    /// fn main() {
    ///     let gpio_num = 4;
    ///     gpio::export(gpio_num);
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function should be called before setting the direction of the GPIO pin.
    pub fn export(gpio_num: i32) -> GpioResult<()> {
       open_file(GpioPaths::EXPORT.as_str()).and_then(|mut file| {
            file.write_all(gpio_num.to_string().as_bytes()).map_err(|why| {
                GpioError::Io(why)
            })
        })
    }

    /// Unexport GPIO pin
    ///
    /// This function unexports (disables) a GPIO pin. The GPIO pin is
    /// passed as an argument. The function writes the GPIO pin to the
    /// `/sys/class/gpio/unexport` file.
    ///
    /// # Arguments
    ///
    /// - `gpio_num` - An integer that represents the GPIO pin number.
    ///
    /// # Returns
    ///
    /// A `GpioResult` that contains the result of the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rgpiolib::gpio;
    ///
    /// fn main() {
    ///     let gpio_num = 4;
    ///     gpio::export(gpio_num);
    ///     gpio::unexport(gpio_num);
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function should be called after exporting the GPIO pin.
    pub fn unexport(gpio_num: i32) -> GpioResult<()> {
        open_file(GpioPaths::UNEXPORT.as_str()).and_then(|mut file| {
            file.write_all(gpio_num.to_string().as_bytes()).map_err(|why| {
                GpioError::Io(why)
            })
        })
    }

    /// Write to GPIO pin
    ///
    /// This function writes a signal to a GPIO pin. The GPIO pin and signal are
    /// passed as arguments. The function writes the signal to the
    /// `/sys/class/gpio/gpio{num}/value` file.
    ///
    /// # Arguments
    ///
    /// - `gpio_num` - An integer that represents the GPIO pin number.
    /// - `signal` - A boolean that represents the signal to write to the GPIO pin.
    ///
    /// # Returns
    ///
    /// A `GpioResult` that contains the result of the operation.
    pub fn write(gpio_num: i32, signal: bool) -> GpioResult<()> {
        Ok(
            open_file(GpioPaths::VALUE(gpio_num).as_str()).and_then(|mut file| {
                file.write_all(signal.to_string().as_bytes()).map_err(GpioError::from)
            })?
        )
    }

    /// Read from GPIO pin
    ///
    /// This function reads a signal from a GPIO pin. The GPIO pin is passed as an argument.
    /// The function reads the signal from the `/sys/class/gpio/gpio{num}/value` file.
    ///
    /// # Arguments
    ///
    /// - `gpio_num` - An integer that represents the GPIO pin number.
    ///
    /// # Returns
    ///
    /// A `GpioResult` that contains the signal read from the GPIO pin.
    pub fn read(gpio_num: i32) -> GpioResult<bool> {
        let value = fs::read_to_string(GpioPaths::VALUE(gpio_num).as_str()).and_then(|contents| {
           match contents.parse::<i32>() {
                Ok(val) => Ok(val),
                Err(why) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, why)),
           }
        }).map_err(GpioError::from);

        value.map(|val| val > 0)
    }

    /// Set GPIO pin direction
    ///
    /// This function sets the direction of a GPIO pin. The GPIO pin and direction are passed as arguments.
    /// The function writes the direction to the `/sys/class/gpio/gpio{num}/direction` file.
    ///
    /// # Arguments
    ///
    /// - `gpio_num` - An integer that represents the GPIO pin number.
    /// - `direction` - A `Directions` enum that represents the direction of the GPIO pin.
    ///
    /// # Returns
    ///
    /// A `GpioResult` that contains the result of the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rgpiolib::gpio;
    ///
    /// fn main() {
    ///     let gpio_num = 4;
    ///     gpio::export(gpio_num);
    ///     gpio::set_direction(gpio_num, gpio::Directions::Output);
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function should be called after exporting the GPIO pin.
    pub fn set_direction(gpio_num: i32, direction: Directions) -> GpioResult<()> {
        open_file(GpioPaths::DIRECTION(gpio_num).as_str()).and_then(|mut file| {
            file.write_all(direction.as_bytes()).map_err(GpioError::from)
        })
    }
}