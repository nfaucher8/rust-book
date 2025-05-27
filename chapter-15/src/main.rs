use std::cell::RefCell;
use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::{Rc, Weak};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(
            "Dropping CustomSmartPointer with data `{}`!",
            self.data
        );
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

#[derive(Debug)]
enum List3 {
    Cons(Rc<RefCell<i32>>, Rc<List3>),
    Nil,
}

#[derive(Debug)]
enum List4 {
    Cons(i32, RefCell<Rc<List4>>),
    Nil,
}

impl List4 {
    fn tail(&self) -> Option<&RefCell<Rc<List4>>> {
        match self {
            List4::Cons(_, item) => Some(item),
            List4::Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(
        1,
        Box::new(Cons(
            2,
            Box::new(Cons(
                3,
                Box::new(Nil)
            ))
        ))
    );

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let y = Box::new(x);
    assert_eq!(5, *y);

    let y = MyBox::new(x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    let a = Rc::new(List2::Cons(5, Rc::new(List2::Cons(10, Rc::new(List2::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = List2::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));


    {
        let c = List2::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(List3::Cons(Rc::clone(&value), Rc::new(List3::Nil)));
    let b = List3::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = List3::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

    let a = Rc::new(List4::Cons(5, RefCell::new(Rc::new(List4::Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(List4::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after creating b = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing b = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}