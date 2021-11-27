fn main() {
    {
        let _v: Vec<i32> = Vec::new();
    }

    {
        let v = vec![1, 2, 3, 4];
        println!("{:?}", v);
    }

    {
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);

        println!("{:?}", v);
    }

    println!("---------------------------");

    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The thrid element is {}", third);

        match v.get(4) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    }

    println!("---------------------------");

    {
        let v = vec![1, 2, 3, 4, 5];

        // let _does_not_exist = &v[100];
        let _does_not_exist = v.get(100);
    }

    println!("---------------------------");

    {
        // let mut v = vec![1, 2, 3, 4, 5];
        // let first = &v[0];
        // v.push(6);
        // println!("The first element is: {}", first);
    }

    println!("--------------------------------");

    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
        println!("{:?}", v);

        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 30;
        }
        println!("{:?}", v);
    }

    println!("---------------------------");

    {
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Float(10.12),
            SpreadsheetCell::Text(String::from("blue")),
        ];
        println!("{:?}", row);
    }
}
