use std::thread;
use std::sync::atomic::Ordering::{Relaxed, Acquire, Release};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64};
use std::time::Duration;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::fence;

//Happens-Before Relationship
// static X: AtomicI32 = AtomicI32::new(0);
// static Y: AtomicI32 = AtomicI32::new(0);
//
// fn a(){
//     X.store(10, Relaxed);
//     Y.store(20, Relaxed);
// }
//
// fn b(){
//     let y = Y.load(Relaxed);
//     let x = X.load(Relaxed);
//     println!("{x} {y}");
// }

// static X: AtomicI32 = AtomicI32::new(0);

// fn f() {
//     let x = X.load(Relaxed);
//     println!("{}", x);
//     assert!(x == 1 || x == 2);
// }

// fn a() {
//     X.fetch_add(5, Relaxed);
//     X.fetch_add(10, Relaxed);
// }
//
// fn a1() {
//     X.fetch_add(5, Relaxed);
// }
// fn a2() {
//     X.fetch_add(10, Relaxed);
// }
//
// fn b() {
//     let a = X.load(Relaxed);
//     let b = X.load(Relaxed);
//     let c = X.load(Relaxed);
//     let d = X.load(Relaxed);
//     println!("{a} {b} {c} {d}");
// }

// fn main() {
    // X.store(1, Relaxed);
    // let t = thread::spawn(f);
    // X.store(2, Relaxed);
    // t.join().unwrap();
    // println!("X {}", X.load(Relaxed));
    // // println!("{}", test);
    // X.store(3, Relaxed);

//     a();
//     b();
// }

// static X: AtomicI32 = AtomicI32::new(0);
// static Y: AtomicI32 = AtomicI32::new(0);
// fn main() {
//     let a = thread::spawn(|| {
//         let x = X.load(Relaxed);
//         Y.store(x, Relaxed);
//     });
//     let b = thread::spawn(|| {
//         let y = Y.load(Relaxed);
//         X.store(y, Relaxed);
//     });
//     a.join().unwrap();
//     b.join().unwrap();
//     assert_eq!(X.load(Relaxed), 0); // Might fail?
//     assert_eq!(Y.load(Relaxed), 0); // Might fail?
// }

// static DATA: AtomicU64 = AtomicU64::new(0);
// static READY: AtomicBool = AtomicBool::new(false);

// fn get_data() -> &'static Data {
//     static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());
//
//     let mut p = PTR.load(Acquire);
//
//     if p.is_null() {
//         p = Box::into_raw(Box::new(generate_data()));
//         if let Err(e) = PTR.compare_exchange(
//             std::ptr::null_mut(), p, Release, Acquire
//         ) {
//             // Safety: p comes from Box::into_raw right above,
//             // and wasn't shared with any other thread.
//             drop(unsafe { Box::from_raw(p) });
//             p = e;
//         }
//     }
//     // Safety: p is not null and points to a properly initialized value.
//     unsafe { &*p }
// }

// fn main() {
//     thread::spawn(|| {
//         DATA.store(123, Relaxed);
//         READY.store(true, Release); // Everything from before this store ..
//         println!("Stored!");
//     });
//
//     while !READY.load(Acquire) {
//         thread::sleep(Duration::from_millis(100));
//         println!("waiting...");
//     }
//     println!("{}", DATA.load(Relaxed));
// }

static mut DATA: [u64; 10] = [0; 10];

const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn some_calculation(num : u64) -> u64 {
    println!("Calculating some number of {}", num);
    thread::sleep(Duration::from_millis(500));
    return num+1;
}

fn main() {
    for i in 0..10 {
        thread::spawn(move || {
            println!("spawning thread {}", i);
            let data = some_calculation(i as u64);
            unsafe { DATA[i] = data };
            READY[i].store(true, Release);
        });
    }

    thread::sleep(Duration::from_millis(2000));
    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Relaxed));
    if ready.contains(&true) {
        fence(Acquire);
        for i in 0..10 {
            if ready[i] {
                println!("READY data{i} = {}", unsafe { DATA[i] });
            }
        }
    }

}