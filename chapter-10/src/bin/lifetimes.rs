use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("attention please; {announcement}");
        self.part
    }
}

fn main() {
    let x = 5;
    let r = &x;
    println!("r: {r}");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // Valid till the end of the program
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");  // Valid till the end of this scope
        // Also valid till the end of this scope because it uses the shortest lifetime between
        // string1 and string2
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // This code is invalid because longest() cannot guarantee that a reference to string1 will be
    // the returned value. It could be string2 or string1, longest doesn't know. Because of this
    // the lifetime of result has to be the smaller lifetime of both string1 and string2, meaning
    // results lifetime ends when the inner scope ends
    //
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");

    let novel = String::from(
        "Call me Ishmael. Some years ago..."
    );
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn return_first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// The return from this function is a reference to a value created in this function, that means when
// the function ends the `result` variable will be deallocated making the reference to result
// invalid. The solution to this would be to return an owned data type and move the ownership of the
// str from the function to the caller
//
// fn invalid_longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("a really long string");
//     result.as_str()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}