#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(dead_code)]
fn main() {
    {
        String::from("hello world!");

        let _s = "aaa";
        let _s2: &str = "cccc";
        let _s3: String = String::from(_s2);
    }

    {
        let mut s = String::new();
        let data = "initial contents";
        let s = data.to_string();
        let s = "initial contents";

        struct S {
            name: String,
            age: u8,
        }

        use std::fmt;
        impl fmt::Display for S {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "name: {}, age: {}", self.name, self.age)
            }
        }

        let obj = S {
            name: String::from(s),
            age: 10,
        };
        let ss = obj.to_string();

        println!("ss: {}", obj);
    }

    println!("--------------------------------");

    {
        let hello = String::from("السلام عليكم");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("שָׁלוֹם");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");
    }

    println!("---------------------------");

    {
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("{}", s);
    }

    println!("---------------------------");

    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(&s2);
        println!("s1 is {}", s1);
        println!("s2 is {}", s2);

        let mut s = String::from("lo");
        s.push_str("l");
        println!("s: {}", s);
    }

    println!("---------------------------");

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");

        let s4: &str = &(&s2)[0..];
        // s1: String s2: &str
        let s3 = s1 + &s2 + s4;
        // println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
        println!("s4: {}", s4);
    }

    println!("---------------------------");

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("s: {}", s);

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("s: {}", s);
    }

    println!("------------------------");

    {
        let s1 = String::from("hello");
        // let h = s1[0];
    }

    println!("------------------------");

    {
        let len = String::from("Hola").len();
        println!("len: {}", len);

        let len = String::from("Здравствуйте").len();
        println!("len: {}", len);

        let hello = "Здравствуйте";
        // let answer = &hello[0];
        // let s = &hello[0..3];
        // println!("s: {}", s);
    }

    println!("------------------------");

    {
        for c in "नमस्ते".chars() {
            println!("{}", c);
        }

        for c in "你好".chars() {
            println!("{}", c);
        }

        for b in "你啊后".bytes() {
            println!("{}", b);
        }
    }
}
