#[allow(unused_variables)]
fn main() {
    {
        let s = "hello";
    }
    // println!("{}", s);

    {
        let mut s = String::from("hello");
        println!("{}", s);

        s.push_str(", world!");
        println!("{}", s);
    }

    println!("--------------------------------");

    {
        let x = 5;
        let y = x;
        println!("x: {}, y: {}", x, y);

        let s1 = String::from("hello");
        let s2 = s1;
        // println!("s1: {}, s2: {}", s1, s2);
    }

    println!("--------------------------------");

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1: {}, s2: {}", s1, s2);

        let t1 = (10, "abc");
        // let t1 = (10, String::from("abc"));
        let t2 = t1;
        println!("t1: {:?}, t2: {:?}", t1, t2);
    }

    {
        fn takes_ownership(some_string: String) {
            println!("{}", some_string);
        }

        fn make_copy(some_integer: i32) {
            println!("{}", some_integer);
        }

        let s = String::from("hello");
        takes_ownership(s);
        // println!("s: {}", s);

        let x = 5;
        make_copy(x);
        println!("x: {}", x);
    }

    println!("--------------------------------");

    {
        fn gives_ownership() -> String {
            let some_string = String::from("hello");
            some_string
        }

        fn takes_and_gives_back(a_string: String) -> String {
            a_string
        }

        let s1 = gives_ownership();
        println!("s1: {}", s1);

        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
        // println!("s2: {}, s3: {}", s2, s3);
    }

    println!("--------------------------------");

    {
        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len();
            (s, length)
        }

        let s1 = String::from("hello world");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
    }
}
