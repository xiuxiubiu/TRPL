#![allow(unused_imports)]

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    {
        let f = File::open("hello.text");

        #[allow(unused_variables)]
        let f = match f {
            Ok(file) => file,

            // panic!("Problem opening the file: {:?}", error)
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.text") {
                    Ok(file) => file,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem Opening the file: {:?}", other_error),
            },
        };
    }

    {
        let x: Option<u32> = Some(3);
        match x {
            Some(34) => println!("34"),
            n => println!("{:?}", n),
        }
    }

    {
        #[allow(unused_variables)]
        let f = File::open("dev.text").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("dev.text").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    {
        // #[allow(unused_variables)]
        // let f = File::open("test.text").unwrap();
    }

    {
        // #[allow(unused_variables)]
        // let f = File::open("test.txt").expect("Failed to open test.txt");
    }

    {
        // #[allow(dead_code)]
        // fn read_username_from_file() -> Result<String, io::Error> {
        //     let f = File::open("users.txt");

        //     let mut f = match f {
        //         Ok(file) => file,
        //         Err(e) => return Err(e),
        //     };

        //     let mut s = String::new();

        //     match f.read_to_string(&mut s) {
        //         Ok(_) => Ok(s),
        //         Err(e) => Err(e),
        //     }
        // }
        // read_username_from_file().unwrap_or_else(|error| {
        //     panic!("read username from file error: {:?}", error);
        // });
    }

    {
        // fn read_username_from_file() -> Result<String, io::Error> {
        //     let mut f = File::open("hello.txt")?;
        //     let mut s = String::new();
        //     f.read_to_string(&mut s)?;
        //     Ok(s)
        // }
        // read_username_from_file().unwrap_or_else(|error| {
        //     panic!("read username from file error: {:?}", error);
        // });
    }

    {
        // fn read_username_from_file() -> Result<String, io::Error> {
        //     let mut s = String::new();
        //     File::open("hello.txt")?.read_to_string(&mut s)?;
        //     Ok(s)
        // }
        // read_username_from_file().unwrap_or_else(|error| {
        //     panic!("read username from file error: {:?}", error);
        // });
    }

    {
        use std::fs;
        fn read_username_from_file() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")
        }
        read_username_from_file().unwrap_or_else(|error| {
            panic!("read username from file error: {:?}", error);
        });
    }

    {
        use std::fs::File;

        let _f = File::open("hello.txt")?;
        Ok(())
    }
}
