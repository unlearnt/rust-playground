use std::fs;
use std::thread;
use std::time::Duration;
use rand::Rng;
fn add(x: i32, y: i32) -> i32 {
    x + y
}

trait Copy {}
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

impl Copy for Point {}

fn count_up_from(start: i32) -> impl Iterator<Item = i32>{
    start..
}

fn sum_iterator<I: Iterator<Item = i32>>(iter: I) -> i32{
    iter.fold(0, |acc, x| acc + x)
}

// Function as Arguments
fn apply_twice<F>(f: F, x:i32) -> i32
where F:Fn(i32) -> i32{
    f(f(x))
}

// Nested Function
fn outer(x: i32) -> i32 {
//     Define a nested function
    fn inner(y: i32) -> i32 {
    y * 2
    }

    inner(x)
}

trait Eq {
    fn eq(&self, other: &Self) -> bool;
}

impl Eq for i32{
    fn eq(&self, other: &i32) -> bool {
        *self == *other
    }
}

fn print_equality<T:Eq>(a:&T, b:&T){
    println!("Are they equal? {}", a.eq(b));
}

trait Display {
    fn display(&self) -> String;
}

struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn display(&self) -> String {
        format!("{} is {} years old.", self.name, self.age)
    }
}

fn main() {
    //sample 1
    // Constant: Global and fixed compile-time value with explicit type
    const MAX_POINTS: u32 = 100_000;
    println!("Maximum points allowed: {}", MAX_POINTS); // Immutable variable: Local and can be shadowed

    let current_points = 50_000;
    println!("Current points: {}", current_points);

    {
        // Shadowing the immutable variable with a new value
        let current_points = 75_000;
        println!("Updated points after shadowing: {}", current_points); // Attempting to reassign either will cause a compile-time error // Uncommenting either line below will result in a compiler error
    }

    println!("Updated points after shadowing: {}", current_points); // Attempting to reassign either will cause a compile-time error // Uncommenting either line below will result in a compiler error


    // MAX_POINTS = 120_000; // Error: cannot assign twice to immutable variable
    // current_points = 100_000; // Error: cannot assign twice to immutable variable

    let add_two = |x: i32| x + 2;
    let test_add_two = add_two(5);
    println!("test_add_two: {}", test_add_two);

    println!("The sum of 3 and 4 is {}", add(3, 4));

    let point = Point { x: 5, y: 10 };
    println!("The distance from the origin is {}", point.distance_from_origin());

    println!("3 plus 2 is {}", add_two(3));

    let mut generator = count_up_from(5);
    println!("{}", generator.next().unwrap()); // Prints 5
    println!("{}", generator.next().unwrap()); // Prints 6

    let v = vec![1, 2, 3, 4];

    let result = sum_iterator(v.into_iter());

    println!("The sum is: {}", result);

    let double = |x| 2 * x;
    let y = apply_twice(double, 5);
    println!("apply_twice: {}", y);

    let input = 5;
    let result = outer(input);
    println!("{}", result);


    // sample 2
    let name = "Alice";
    println!("Hello, {}", name);

    // Read from a file
    let data = fs::read_to_string("hello.txt").expect("Unable to read file");
    println!("data: {}", data);

    // Spawn a new thread and sleep
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));
        println!("Thread awake!");
    });

    // Working with collections
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    println!("{:?}", vec);


    //sample 3
    let mut rng = rand::thread_rng();

    let n = rng.gen_range(-5..5);

    if n < 0 {
        println!("n is less than 0 : {}", n);
    } else if n == 0 {
        println!("ZERO");
    } else {
        println!("n: {}", n);
    }

    // sample 4
    let mut i = 0;

    loop {
        println!("falcon");

        i += 1;
        if i == 5 {
            break;
        }
    }

    // sample 5
    let mut x = 1;

    while x <= 10 {
        println!("{}", x);

        x+=1;
    }

    println!("x: {}", x);

    // sample 6
    let vals = [1,2,3,4,5,6,7,8,9,10];

    for e in vals{
        println!("{}", e);
    }
    println!();

    //sample 6
    let grades = ["A", "B", "C", "D", "E", "F", "FX"];
    for grade in grades{
        match grade {
            "A" | "B" | "C" | "D" | "E" | "F" => println!("passed"),
            "FX" => println!("failed"),
            _ => println!("unknown"),
        }
    }

    // sample 7
    let x = 5;
    let y = 10;
    assert!(Eq::eq(&x, &y) == false);

    // sample 8
    print_equality(&5, &5);

    //sample 9
    let person = Person {name: "Alice".to_string(), age: 25};
    println!("{:?}", person.display());
}