#[allow(unused_variables)]
fn main() {
    {
        let s1 = String::from("hello");
        let len = calulate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);

        fn calulate_length(s: &String) -> usize {
            s.len()
        }
    }

    println!("---------------------------");

    {
        let mut s = String::from("hello");
        change(&mut s);
        println!("s is: {}", s);

        fn change(some_string: &mut String) {
            some_string.push_str(", world!");
        }
    }

    {
        let s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        // let r3 = &mut s;
        println!("{}, {}", r1, r2);

        // let mut s = String::from("hello");
        // let r1 = &mut s;
        // let r2 = &mut s;
        // println!("{}, {}", r1, r2);
    }

    {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
        }
        let r3 = &mut s;
        println!("{}", r3);
    }

    {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);

        let r3 = &mut s;
        println!("{}", r3);
    }

    println!("---------------------------");

    {
        // fn dangle() -> &String {
        //     let s = String::from("hello");
        //     &s
        // }
        // let reference_to_nothing = dangle();

        fn no_dangle() -> String {
            String::from("hello")
        }
        let h = no_dangle();
        println!("{}", h);
    }
}
