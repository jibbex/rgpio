#[cfg(test)]
mod tests {
    use rgpiolib::gpio;

    #[test]
    fn export_valid_gpio() {
        let result = gpio::export(4);
        assert!(result.is_ok());
    }

    #[test]
    fn unexport_valid_gpio() {
        let result = gpio::unexport(4);
        assert!(result.is_ok());
    }

    #[test]
    fn set_direction_output() {
        let result = gpio::set_direction(4, gpio::Directions::Output);
        assert!(result.is_ok());
    }

    #[test]
    fn set_direction_input() {
        let result = gpio::set_direction(4, gpio::Directions::Input);
        assert!(result.is_ok());
    }

    #[test]
    fn write_high_signal() {
        let result = gpio::write(4, true);
        assert!(result.is_ok());
    }

    #[test]
    fn write_low_signal() {
        let result = gpio::write(4, false);
        assert!(result.is_ok());
    }

    #[test]
    fn read_high_signal() {
        let result = gpio::read(4);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn read_low_signal() {
        let result = gpio::read(4);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }
}