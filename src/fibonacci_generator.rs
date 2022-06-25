pub mod fibonacci_generator {
    use std::io;

    pub fn number_generator() {
        // Генерирование n-го числа Фибоначчи
        println!("Generating the N-th Fibonacci number");
        loop {
            println!("Please input number:");

            let mut number = String::new();

            io::stdin().read_line(&mut number).expect("Failed to read line");

            let number: u128 = match number.trim().parse() {
                Ok(num) => {
                    if num == 0 {
                        println!("It is not a valid integer!");
                        continue
                    }
                    num
                },
                Err(_) => {
                    println!("It is not a valid integer!");
                    continue
                }
            };
            let mut previous: u128 = 1;
            let mut current = 1;

            for _ in 3..=number {
                let next = previous + current;
                previous = current;
                current = next;
            }
            println!("{number}-th Fibonacci number is {current}")
        }
    }
}