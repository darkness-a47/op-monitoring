pub mod gpio {
    pub struct GpioStatus {
        pub values: (String, String)
    }

    const GPIO_PINS: [(&str, u8, &str); 9] = [
        ("پمپ ۱", 12, "pump 1"),
        ("پمپ ۲", 13, "pump 2"),
        ("پمپ ۳", 14, "pump 3"),
        ("پمپ ۴", 15, "pump 4"),
        ("پمپ ۵", 16, "pump 5"),
        ("پمپ ۶", 17, "pump 6"),
        ("پمپ ۷", 18, "pump 7"),
        ("پمپ ۸", 19, "pump 8"),
        ("پمپ ۹", 20, "pump 9"),
    ];
    pub fn init_pins() {
        for key in GPIO_PINS {
            println!("configuration for pin number: {} ({})", key.1, key.2)
        }
    }

    pub fn read_status() -> Vec<GpioStatus> {
        let mut res: Vec<GpioStatus> = vec![];
        for key in GPIO_PINS {
            res.push(GpioStatus { values: ( key.0.to_string(), "0".to_string()) });
        }

        res
    }
}