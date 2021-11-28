fn main() {
    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        println!("scorse: {:?}", scores);
    }

    println!("------------------------");

    {
        use std::collections::HashMap;

        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let scores: HashMap<&String, &i32> = teams.iter().zip(initial_scores.iter()).collect();
        println!("scores: {:?}", scores);
        println!("teams: {:?}, initial_scores: {:?}", teams, initial_scores);
    }

    println!("------------------------");

    {
        use std::collections::HashMap;

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(&field_name, &field_value);
        println!("map: {:?}", map);
        println!("name: {:?}, valaue: {:?}", field_name, field_value);
    }

    println!("------------------------");

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("yellow"), 50);

        let team_name = String::from("blue");
        let score = scores.get(&team_name);
        println!("score: {:?}", score);
        match score {
            Some(score) => println!("score: {}", score),
            None => println!("{} not exists!", &team_name),
        }

        let new_score = scores.get("purple");
        if let Some(score) = new_score {
            println!("score: {}", score);
        } else {
            println!("purple not exists!");
        }
    }

    println!("------------------------");

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    println!("------------------------");

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 500);
        println!("{:?}", scores);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("scores: {:?}", scores);
    }

    println!("------------------------");

    {
        use std::collections::HashMap;

        let text = "hello world wonderful world world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            println!("count: {}", count);
            *count += 1;
        }

        println!("map: {:?}", map);
    }

    println!("------------------------");

    {
        let v = vec![10, 90, 40, 33, 86, 11, 68, 44, 66, 44];
        v.insert()
    }
}
