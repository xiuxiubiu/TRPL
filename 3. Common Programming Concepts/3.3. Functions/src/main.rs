fn main() {
    println!("Hello, world!");

    another_function();

    {
        another_function(5);
        fn another_function(x: i32) {
            println!("The value of x is: {}", x);
        }
    }

    {
        fn print_labled_measurement(value: i32, unit_label: char) {
            println!("The measurement is: {}{}", value, unit_label);
        }
        print_labled_measurement(5, 'h');
    }

    {
        // let x = (let y = 6);
        #[allow(unused_variables)]
        let x = 5;
        let y = {
            let x = 3;
            x + 1
        };
        println!("The value of y is: {}", y);
    }

    {
        fn five() -> i32 {
            5
        }
        let x = five();
        println!("The value of x is: {}", x);

        fn plus_one(x: i32) -> i32 {
            x + 1
            // x + 1;
        }
        let x = plus_one(5);
        println!("The value of x is: {}", x);
    }
}

fn another_function() {
    println!("Another function.");
}
