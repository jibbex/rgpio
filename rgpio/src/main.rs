use std::env;
use std::{thread, time};
use rgpiolib::gpio;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print!("usage: rgpio ...[num]");
    } else {
        for i in 1..args.len() {
            let gpio: i32;

            match args[i].parse() {
                Err(why) => panic!("an error has occurred: {}", why),
                Ok(num) => gpio = num,
            }

            gpio::export(gpio);
            gpio::set_direction(gpio, gpio::Directions::Output);
            gpio::write(gpio, true);
            let mut val = gpio::read(gpio);
            println!("{}", val);

            thread::sleep(time::Duration::from_millis(500));

            gpio::write(gpio, false);
            thread::sleep(time::Duration::from_millis(500));
            val = gpio::read(gpio);
            println!("{}", val);
            gpio::unexport(gpio);
        }   
    }     
}