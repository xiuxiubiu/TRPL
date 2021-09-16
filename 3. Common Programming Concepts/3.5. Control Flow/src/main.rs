fn main() {
    {
        let number = 7;
        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }

    println!("--------------------------------");

    {
        // let number = 3;
        // if number {
        //     println!("number was three");
        // }
    }

    println!("--------------------------------");

    {
        let number = 7;
        if number % 4 == 0 {
            println!("numnber is divisible by 4");
        } else if number % 3 == 0 {
            println!("numnber is divisible by 3");
        } else if number % 2 == 0 {
            println!("numnber is divisible by 2");
        } else {
            println!("numnber is not divisible by 4, 3, or 2")
        }
    }

    println!("--------------------------------");

    {
        let condition = true;
        let number = if condition {
            5
        } else {
            6
            // "six"
        };
        println!("The value of number is: {}", number);
    }

    println!("--------------------------------");

    {
        // loop {
        //     println!("again!");
        // }
    }

    println!("--------------------------------");

    {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {}", count);
            let mut remaining = 10;

            loop {
                println!("remaining = {}", remaining);
                if remaining == 5 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }

        println!("End count = {}", count);
    }

    println!("--------------------------------");

    {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {}", result);
    }

    println!("--------------------------------");

    {
        let mut number = 3;
        while number != 0 {
            println!("{}!", number);
            number = number - 1;
        }
        println!("LIFTOFF!!!");
    }

    println!("--------------------------------");

    {
        let a = [10, 20, 30, 4, 50];
        let mut index = 0;
        while index < 5 {
            println!("the value is: {}", a[index]);
            index += 1;
        }

        for element in a.iter() {
            println!("the value is: {}", element);
        }
    }

    println!("--------------------------------");

    {
        for number in (1..4).rev() {
            println!("{}!", number);
        }
    }
}
