#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    {
        #[derive(Debug)]
        enum IpAddrKind {
            V4,
            V6,
        }

        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        fn route(ip_type: IpAddrKind) {}

        route(four);
        route(six);

        #[derive(Debug)]
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };

        println!("{:?}, {:?}", home, loopback);
    }

    println!("---------------------------");

    {
        #[derive(Debug)]
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let home = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::1"));
        println!("{:?}, {:?}", home, loopback);
    }

    println!("---------------------------");

    {
        #[derive(Debug)]
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
        println!("{:?}, {:?}", home, loopback);
    }

    println!("---------------------------");

    {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        impl Message {
            fn call(&self) {
                println!("{:?}", self);
            }
        }

        let m = Message::Write(String::from("hello"));
        m.call()
    }

    println!("---------------------------");

    {
        let some_number = Some(5);
        let some_string = Some("a string");
        let absent_number: Option<i32> = None;

        // let x: i8 = 5;
        // let y: Option<i8> = Some(5);
        // let sum = x + y;

        let x = Some(3333);
        println!("{}", x.expect("fruits are healthy"));
    }
}
