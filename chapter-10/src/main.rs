use std::fmt::{Debug, Display};

struct Point<T> {
    x: T,
    y: T
}

struct Point2<T, U> {
    x: T,
    y: U
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

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y
        }
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!(
            "(Read more from {}...)",
            self.summarize_author()
        )
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!(
//             "{}, by {} ({})",
//             self.headline,
//             self.author,
//             self.location
//         )
//     }
// }

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Blanket trait implementation example
// impl<T: Display> ToString for T {
//
// }

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};

    // let wont_work = Point {x: 5, y: 4.0};

    let both_integer = Point2 {x: 5, y: 10};
    let both_float = Point2 {x: 1.0, y: 4.0};
    let integer_and_float = Point2 {x: 5, y: 4.0};

    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());

    let p1 = Point3 {x: 5, y: 10.4};
    let p2 = Point3 {x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from(
            "Penguins win the Stanley Cup Championship!"
        ),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        )
    };

    println!("New article available! {}", article.summarize());
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// In this function item1 and item2 can be different types that both implement the trait "Summary"
pub fn notify2(item1: &impl Summary, item2: &impl Summary) {

}

// In this function item1 and item2 must be the same time that implements the trait "Summary"
pub fn notify3<T: Summary>(item1: &T, item2: &T) {

}

// This function requires item type to implement both the Summary and Display traits
pub fn notify4(item: &(impl Summary + Display)) {

}

// This is the same as notify4 but using the trait bounds syntax
pub fn notify5<T: Summary + Display>(item: &T) {

}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    42
}

// Alternative trait bounds syntax
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    42
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably know, people",
        ),
        reply: false,
        retweet: false,
    }
}


// Cannot return different types that both implement the same thing
// fn returns_summarizable2(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!"
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//             hockey team in the NHL.",
//             )
//         };
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }