#[allow(dead_code)]
fn main() {
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("Lucky penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }

        println!("coin: {}", value_in_cents(Coin::Nickel));
        println!("coin: {}", value_in_cents(Coin::Penny));
    }

    println!("---------------------------");

    {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 2,
                Coin::Dime => 3,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }

        println!(
            "coin: {:?}",
            value_in_cents(Coin::Quarter(UsState::Alabama))
        );
    }

    println!("---------------------------");

    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        println!("{:?}, {:?}", six.unwrap(), none);
    }

    println!("---------------------------");

    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                Some(i) => Some(i + 1),
                None => None,
            }
        }

        let some_u8_value = 1u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => (),
        }
    }
}
