
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

// Tuple Structs
struct Color(i32, i32, i32);

// Unit-Like Structs
struct Empty;
impl Empty {
    fn new() -> Self{
        Empty
    }
}

// Definition of the student struct
struct Student {
    id: i32,
    name: String,
    major: String,
    year: i32,
}

impl Student {
    fn new(id:i32, name: &str, major: &str, year: i32) -> Self {
        Self {
            id,
            name: name.to_string(),
            major: major.to_string(),
            year,
        }
    }

    fn calculate_gpa(&self, grades:&[i32]) -> f64{
        let sum: i32 = grades.iter().sum();
        let num_grades: f64 = grades.len() as f64;
        sum as f64 / num_grades
    }

    fn display(&self) {
        println!("Student ID: {}, Name: {}, Major: {}, Year: {}", self.id,
                 self.name, self.major, self.year);
    }

}

struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    fn say_hello(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name,  self.age);
    }
}

// Custom struct
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle {width,height}
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Nested struct
struct InnerStruct {
    field3: i32,
    field4: i32,
}

impl InnerStruct {
    fn new(field3: i32, field4: i32) -> Self {
        Self {field3,field4,}
    }

    fn sum(&self) -> i32{
        self.field3 + self.field4
    }
}

struct OuterStruct {
    field1: i32,
    field2: i32,
    nested: InnerStruct,
}

impl OuterStruct {
    fn new(field1: i32, field2: i32, nested: InnerStruct) -> Self {
        Self {field1,field2,nested,}
    }

    fn details(&self){
        println!("field1: {}, field2: {}", self.field1, self.field2);
        println!("Nested field3: {}, field4: {}", self.nested.field3, self.nested.field4);
    }
}

// Struct Inheritance
struct BaseStruct{
    field1: i32,
    field2: i32,
}

impl BaseStruct {
    fn sum(&self) -> i32{
        self.field2 + self.field1
    }
}

struct DerivedStruct {
    base: BaseStruct,
    field3:i32,
}

// Using Traits for Inheritance-like Behavior
trait BaseTrait {
    fn sum(&self) -> i32;
}

struct BaseStruct2 {
    field1: i32,
    field2: i32,
}

impl BaseTrait for BaseStruct2 {
    fn sum(&self) -> i32 {
        self.field2 + self.field1
    }
}

struct DerivedStruct2<'a> {
    base: &'a dyn BaseTrait,
    field3: i32,
}

fn main() {
    let p = Point { x: 10, y: 20 };

    let distance = p.distance_from_origin();
    println!("Distance from origin: {}", distance);

    let white = Color(255, 255, 255);
    println!("White RGB is ({}, {}, {})", white.0, white.1, white.2);

    let _empty = Empty::new();

    // Creating instances of Student
    let s1 = Student::new(1, "Alice", "Computer Science", 3);
    let s2 = Student::new(2, "Bob", "Physics", 4);
    let s3 = Student::new(3, "Charlie", "Chemistry", 2);
    // Example of calculating GPA
    let s1_gpa = s1.calculate_gpa(&[90, 95, 80]);
    println!("GPA of {}: {}", s1.name, s1_gpa);
    // Displaying student information
    s1.display();
    s2.display();
    s3.display();

    let p1 = Person::new("Alice", 30);
    let p2 = Person::new("Bob", 40);

    p1.say_hello();
    p2.say_hello();

    let r1 = Rectangle::new(10, 20);
    let r2 = Rectangle::new(5, 15);
    println!("The width of r1 is {}, and the height is {}", r1.width, r1.height);
    println!("The area of r1 is {}", r1.area());
    println!("The area of r2 is {}", r2.area());

    let inner = InnerStruct::new(10, 20);
    let outer = OuterStruct::new(1, 2, inner);
    outer.details();
    println!("The sum of nested fields: {}", outer.nested.sum());

    let base = BaseStruct { field1: 1, field2: 2 };
    let derived = DerivedStruct { base, field3: 3 };
    println!("The value of field1 is {}", derived.base.field1);
    println!("The value of field2 is {}", derived.base.field2);
    println!("The value of derived's field3 is {}", derived.field3);
    println!("The sum of base fields is {}", derived.base.sum());

    let base2 = BaseStruct2 { field1: 10, field2: 20 };
    let derived2 = DerivedStruct2 { base: &base2, field3: 5 };
    println!("The sum from BaseTrait is {}", derived2.base.sum());
    println!("Additional field in derived struct: {}", derived2.field3);
}
