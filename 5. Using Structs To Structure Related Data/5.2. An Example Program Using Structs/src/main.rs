fn main() {
    {
        fn area(width: u32, height: u32) -> u32 {
            width * height
        }

        let width = 30;
        let height = 50;
        println!(
            "The area of the rectangle is {} square pixels.",
            area(width, height)
        );
    }

    println!("---------------------------");

    {
        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }

        let rect = (30, 50);
        println!(
            "The area of the rectangle is {} squaare pixels.",
            area(rect)
        );
    }

    println!("---------------------------");

    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl std::fmt::Display for Rectangle {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "width: {}, heigth: {}", self.width, self.height)
            }
        }

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }

        let rect = Rectangle {
            width: 30,
            height: 50,
        };
        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect)
        );
        println!("{:?}, {0}", rect);
    }
}
