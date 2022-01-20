fn main() {
    {
        fn largest_i32(list: &[i32]) -> i32 {
            let mut largest = list[0];

            for &item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        fn largest_char(list: &[char]) -> char {
            let mut largest = list[0];

            for &item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    }

    println!("---------------------------------");

    {

        // fn largest<T>(list: &[T]) -> T {
        //     let mut largest = list[0];

        //     for &item in list {
        //         if item > largest {
        //             largest = item;
        //         }
        //     }

        //     largest
        // }

        // let number_list = vec![34, 50, 25, 100, 65];

        // let result = largest(&number_list);
        // println!("The largest number is {}", result);

        // let char_list = vec!['y', 'm', 'a', 'q'];

        // let result = largest(&char_list);
        // println!("The largest char is {}", result);
    }

    println!("------------------------");

    {
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer = Point { x: 1, y: 2 };
        let float = Point { x: 1.0, y: 1.0 };

        println!("integet: {:?}, float: {:?}", integer, float);

        // let err = Point {x: 1.0, y: 2};
    }

    println!("------------------------");

    {
        #[derive(Debug)]
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let both_integer = Point { x: 5, y: 10 };
        let both_float = Point { x: 1.0, y: 4.0 };
        let integer_and_float = Point { x: 5, y: 4.0 };

        println!(
            "both_integer: {:?}, both_float: {:?}, integer_and_float: {:?}",
            both_integer, both_float, integer_and_float
        );
    }

    println!("------------------------");

    {
        #[allow(dead_code)]
        enum Option<T> {
            Some(T),
            None,
        }

        #[allow(dead_code)]
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }

    println!("------------------------");

    {
        struct Point<T> {
            x: T,

            #[allow(dead_code)]
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.x());

        let fp = Point { x: 1.0, y: 2.0 };
        println!("distance_from_origin: {}", fp.distance_from_origin());
    }

    println!("------------------------");

    {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> Point<T, U> {
            fn mixup(self, other: Point<T, U>) -> Point<T, U> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}
