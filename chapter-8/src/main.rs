use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    if let Some(third) = third {
        println!("The third element is {}", third);
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100).unwrap();

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    // This is an interesting way to convert a &str to a String
    let s = str::to_string(data);

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used

    println!("{s3}");

    // The + operator uses the add method. Does that mean this is possible?
    // Answer: Yes if we import the trait std::ops::Add;
    use std::ops::Add;

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = String::add(s1, &s2);

    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let s1 = String::from("hello");
    // let h = s1[0];  // Strings cannot be accessed by index in rust
    let h = &s1[..=0];  // Creating a &str slice from a String is valid though

    // Using &str slices isn't always a good idea because of localization and how Grapheme Clusters
    // work
    println!("{h}");

    let hello = "Здравствуйте";  // Each character in this string is 2 bytes
    let s = &hello[0..4];  // So slicing 4 bytes is fine

    println!("{s}");

    // let s = &hello[0..3];  // This would error as we can't slice in the middle of a character

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    // This still doesn't work with chars, the standard library doesn't have a way to get Graphemes
    for c in "नमस्ते".chars() {
        println!("{c}");
    }

    // Using unicode_segmentation::UnicodeSegmentation we can add a graphemes method to &str
    // which works as expected
    for g in "नमस्ते".graphemes(true) {
        println!("- {g}");
    }

    for g in "hello".graphemes(true) {
        println!("- {g}");
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    dbg!(score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are invalid at this point since their ownership was moved into
    // the hash map. Try using them and see what compiler error you get!
    // println!("{field_name}");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
