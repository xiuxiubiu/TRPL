#[allow(dead_code)]
fn main() {
    {
        let some_u8_value = Some(3u8);

        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }

        if let Some(3) = some_u8_value {
            println!("Three if let");
        }
    }

    println!("--------------------------------");

    {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
        }

        enum Coin<'a> {
            Penny,
            Nickel,
            Dime,
            Quarter(&'a UsState),
        }

        let coin = Coin::Quarter(&UsState::Alaska);
        // let coin = Coin::Penny;
        let mut count = 0;

        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => count += 1,
        }
        println!("match count: {}", count);

        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }
        println!("if let count: {}", count);
    }
}
