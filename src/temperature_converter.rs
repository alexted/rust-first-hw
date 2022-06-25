pub mod temperature_converter {
    use std::io;

    pub fn converter() {
        // Конвертация температур между значениями по Фаренгейту к Цельсия
        println!("Converting temperature from Fahrenheit to Celsius!");
        loop {
            println!("Please input value of temperature in Fahrenheit:");

            let mut fahrenheit_temperature = String::new();

            io::stdin()
                .read_line(&mut fahrenheit_temperature)
                .expect("Failed to read line");
            let fahrenheit_temperature: i64 = match fahrenheit_temperature.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("It is not an integer!");
                    continue;
                }
            };

            let res = (fahrenheit_temperature - 32) * 5 / 9;
            println!("{res} of Celsius");
        }
    }
}
