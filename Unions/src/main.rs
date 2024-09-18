union MyUnion {
    i: i32,
    f: f32,
}

enum Color {
    Red,
    Green,
    Blue,
}

union MyUnion {
    c: Color,
    i: i32,
}

struct Point {
    x: i32,
    y: i32,
}

union MyUnion {
    p: Point,
    i: i32,
}

union MyUnion {
    a: [i32; 4],
    i: i32,
}

fn main() {
    let mut u = MyUnion { i: 10 };

    unsafe {
        println!("u.i = {}", u.i);

        u.f = 3.14; // Reassign to store a float
        println!("u.f = {}", u.f); // Now safe to access f
    }
}
