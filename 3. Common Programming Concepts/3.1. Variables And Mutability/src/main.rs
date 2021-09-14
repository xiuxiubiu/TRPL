fn main() {
    {
        let x = 5;
        println!("The value of x is: {}", x);

        // x = 6;
        // println!("The value of x is: {}", x);

        let mut y = 5;
        println!("The value of y is: {}", y);

        y = 6;
        println!("The value of y is: {}", y);
    }

    println!("--------------------------------");

    {
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
        println!("{}", THREE_HOURS_IN_SECONDS);
    }

    // println!("{}", THREE_HOURS_IN_SECONDS);

    println!("--------------------------------");

    {
        let x = 5;

        let x = x + 1;

        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {}", x);
        }

        println!("The value of x is: {}", x);

        let spaces = "      ";
        let spaces = spaces.len();
        println!("spaces lenth: {}", spaces);

        // let mut spaces = "      ";
        // spaces = spaces.len();
    }
}
