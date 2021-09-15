fn main() {
    {
        let guess_one: u32 = "42".parse().expect("Not a number");
        let guess_two = "42".parse::<u32>().expect("Not a number");
        println!("{}, {}", guess_one, guess_two);
    }

    {
        let x = 2.0;
        let y: f32 = 3.0;
        println!("x: {}, y: {}", x, y);
    }

    {
        let sum = 5 + 10;
        let difference = 95.5 - 4.3;
        let product = 4 * 30;
        let quotient = 56.7 / 32.2;
        let floored = 2 / 3;
        let remainder = 43 % 5;

        println!(
            "sum: {}, difference: {}, product: {}, quotient: {}, floored: {}, remainder: {}",
            sum, difference, product, quotient, floored, remainder
        );
    }

    {
        let t = true;
        let f: bool = false;
        println!("true: {}, false: {}", t, f);
    }

    {
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';
        println!("{:?}, {:?}, {:?}", c, z, heart_eyed_cat);
    }

    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        println!("{:?}", tup);

        let str = "abcdefg";
        let arr = [1, 2];
        let tup = (500, 6.4, "tup", &str, arr);
        let (zero, one, two, three, four) = tup;
        println!("{:?},{:?},{:?},{:?},{:?}", zero, one, two, three, four);
        println!("{}, {}", tup.0, tup.1);
    }

    {
        let a = [1, 2, 3, 4, 5];
        println!("{:?}", a);

        let months = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
        println!("{:?}", months);

        let a: [i32; 5] = [1, 2, 3, 4, 5];
        println!("{:?}", a);

        let a = [3; 5];
        println!("{:?}", a);

        let a = [1, 2, 3, 4, 5];
        let first = a[0];
        let second = a[1];
        println!("first: {}, second: {}", first, second);
    }

    {
        let a = [1, 2, 3, 4, 5];
        println!("Please enter an array index.");
        let mut index = String::new();

        std::io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line!");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!(
            "The value of the element at index {} is :{}",
            index, element
        );
    }
}
