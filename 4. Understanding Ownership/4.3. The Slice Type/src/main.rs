#[allow(unused_variables)]
fn main() {
    {
        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i;
                }
            }
            s.len()
        }

        let mut s = String::from("hello world!");
        let word = first_word(&s);
        s.clear();
        println!("s: {}, word: {}", s, word);
    }

    println!("---------------------------");

    {
        let s = String::from("hello world");
        let hello = &s[..5];
        let world = &s[6..11];

        println!("{}, {}", hello, world);

        println!("{}, {}", &s[..], &s[0..s.len()]);
    }

    println!("--------------------------------");

    {
        fn first_word(s: &String) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[..i];
                }
            }
            &s[..]
        }
        let s = String::from("hello world");
        let word = first_word(&s);

        // s.clear();

        println!("the first word is: {}", word);
    }

    println!("--------------------------------");

    {
        fn first_word(s: &str) -> &str {
            s
        }

        // let s = String::from("123456");
        // let a: &str;
        // a = &s;

        let my_string = String::from("my_string");
        let word = first_word(&my_string);
        println!("{}", word);

        let my_strign_literal = "my_strign_literal";
        let word = first_word(&my_strign_literal);
        let word = first_word(my_strign_literal);
        println!("{}", word);
    }

    println!("---------------------------");

    {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        println!("{:?}", slice);
    }
}
