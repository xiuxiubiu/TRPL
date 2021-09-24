#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    {
        #[derive(Debug)]
        struct User {
            username: String,
            email: String,
            sign_in_count: u64,
            active: bool,
        }

        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        println!("{:?}", user1);

        user1.email = String::from("anotheremail@example.com");
        println!("{:?}", user1);

        fn build_user(email: String, username: String) -> User {
            User {
                email: email,
                username: username,
                active: true,
                sign_in_count: 1,
            }
        }
        let email = String::from("1111");
        let username = String::from("a@abc.com");
        println!("{:?}", build_user(email, username));

        fn simple_build_user(email: String, username: String) -> User {
            User {
                active: false,
                sign_in_count: 2,
                email,
                username,
            }
        }
        let email = String::from("1111");
        let username = String::from("a@abc.com");
        println!("{:?}", simple_build_user(email, username));

        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            active: user1.active,
            sign_in_count: user1.sign_in_count,
        };
        println!("{:?}", user2);

        let user3 = User {
            email: String::from("another@example.com"),
            ..user2
        };
        println!("{:?}", user3);
    }

    println!("---------------------------");

    {
        #[derive(Debug)]
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(100, 0, 0);
        let origin = Point(0, 0, 0);
        println!("{:?}, {}", black, black.0);
    }

    println!("---------------------------");

    {
        struct User<'a> {
            username: &'a str,
            email: &'a str,
            sign_in_count: u64,
            active: bool,
        }

        let user1 = User {
            username: "someusername123",
            email: "someone@example.com",
            active: false,
            sign_in_count: 1,
        };
    }
}
