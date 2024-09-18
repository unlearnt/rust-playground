trait Eq {
    fn eq(&self, other: &Self) -> bool;
}

impl Eq for i32 {
    fn eq(&self, other: &i32) -> bool {
        *self == *other
    }
}

// Basic Traits
fn print_equality<T: Eq>(a: &T, b: &T) {
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
        format!("{} is {} years old", self.name, self.age)
    }
}

// Marker Traits
// trait Copy {
//     // fn copy(&self) -> Self;
// }
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// impl Copy for Point {}

// Generic Traits with Associated Types
trait Container {
    type Item;
    fn get(&self, index:usize) -> Option<&Self::Item>;
}

struct List<T> {
    elements: Vec<T>,
}

impl<T> Container for List<T> {
    type Item = T;
    fn get(&self, index:usize) -> Option<&Self::Item> {
        self.elements.get(index)
    }
}

//  Operator Overloading Traits
use std::ops::Add;
struct Matrix {
    elements: Vec<Vec<u32>>,
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, other: Matrix) -> Matrix {

        let mut result = self.elements.clone();

        for (i, row) in self.elements.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                result[i][j] += other.elements[i][j];
            }
        }
        Matrix { elements: result }
    }
}

// Trait Objects for Dynamic Dispatch
trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f32,
}

struct Square {
    side: f32,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius: {}", self.radius);
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square with width: {}", self.side);
    }
}

fn display_shape(shape: &impl Drawable){
    shape.draw();
}

fn main() {
    let x = 5;
    let y = 10;
    assert!(Eq::eq(&x, &y) == false);

    print_equality(&x, &y);

    let person = Person { name: String::from("John"), age: 18 };
    println!("{:?}", person.display());

    // let point = Point { x: 2, y: 4 };
    // let point_copy = &point.copy();
    // println!("{:?}", point_copy);

    let list = List{elements: vec![1,2,3]};
    let first = list.get(0);
    println!("{:?}", first);

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in animals {
        animal.speak();
    }

    let circle = Circle{radius: 5.0};
    let square = Square{side: 10.0};

    display_shape(&circle);
    display_shape(&square);
}
