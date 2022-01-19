fn main() {
    {
        use std::net::IpAddr;
        let home: IpAddr = "127.0.0.1".parse().unwrap();
        println!("home: {:?}", home);
    }

    {
        fn empty_param(str: &str) {
            println!("str: {:?}", str);
        }
        empty_param("1");
        empty_param(&"2".to_string())
    }

    {
        use rand::Rng;
        use std::cmp::Ordering;
        use std::error;
        use std::fmt;
        use std::io;
        use std::result::Result;

        #[derive(Debug)]
        pub struct GuessErr;

        impl fmt::Display for GuessErr {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                write!(fmt, "invalid guess value")
            }
        }

        impl error::Error for GuessErr {
            fn source(&self) -> Option<&(dyn error::Error + 'static)> {
                None
            }
        }

        pub struct Guess {
            value: i32,
        }

        impl Guess {
            pub fn new(value: i32) -> Result<Guess, GuessErr> {
                if value < 1 || value > 100 {
                    return Err(GuessErr);
                }

                Ok(Guess { value })
            }

            pub fn value(&self) -> i32 {
                self.value
            }
        }

        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: i32 = match guess.trim().parse() {
                Ok(n) => n,
                Err(_) => continue,
            };

            println!("You guessed: {}", guess);

            Guess::new(guess).unwrap();

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
}
