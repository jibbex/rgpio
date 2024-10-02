use std::env;
use std::{thread, time};
use std::error::Error;
use rgpiolib::gpio;

fn main() -> Result<(), dyn Error> {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check if no arguments are provided
    if args.len() == 1 {
        print!("usage: rgpio ...[num]");
    } else {
        // Iterate over the provided arguments
        for i in 1..args.len() {
            // Parse the argument to an integer
            let gpio_num = match args[i].parse() {
                Err(why) => panic!("an error has occurred: {}", why),
                Ok(num) => num,
            };

            // Export the GPIO pin and set its direction to output
            if (gpio::export(gpio_num).is_ok()) {
                gpio::set_direction(gpio_num, gpio::Directions::Output)?;
                gpio::write(gpio_num, true)?;

                // Read the GPIO pin value and print it
                let mut val = gpio::read(gpio_num)?;
                println!("{}", val);

                // Sleep for 500 milliseconds
                thread::sleep(time::Duration::from_millis(500));

                // Write false to the GPIO pin and sleep again
                gpio::write(gpio_num, false)?;
                thread::sleep(time::Duration::from_millis(500));

                // Read the GPIO pin value again and print it
                val = gpio::read(gpio_num)?;
                println!("{}", val);

                // Unexport the GPIO pin
                gpio::unexport(gpio_num)?;
            } else {
                println!("failed to export gpio {}", gpio_num);
            }
        }
    }

    Ok(())
}