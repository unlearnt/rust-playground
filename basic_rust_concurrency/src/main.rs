use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::collections::VecDeque;
use std::sync::Condvar;

fn main() {

    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread.");
    t1.join().unwrap();
    t2.join().unwrap();

    // part 2
    let numbers = vec![1, 2, 3];
    thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
    }).join().unwrap();

    // part 3
    let numbers = Vec::from_iter(0..=1000);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();
    println!("average: {average}");

    //part 4
    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });

        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });

    // let mut numbers = vec![1, 2, 3];
    //
    // thread::scope(|s| {
    //     s.spawn(|| {
    //         numbers.push(4);
    //     })
    // });

    // let a = Arc::new([1,2,3]);
    // let b = a.clone();
    //
    // thread::spawn(move || dbg!(a));
    // thread::spawn(move || dbg!(b));

    // let a = Arc::new([1, 2, 3]);
    // let b = a.clone();
    // thread::spawn(move || {
    //     dbg!(b);
    // });
    //
    // dbg!(a);

    let a = Arc::new([1, 2, 3]);

    thread::spawn({
        let a = a.clone();
        move || {
            for n in a.to_vec() {
                println!("hello {n}");
            }
            dbg!(a);
        }
    });

    dbg!(a);

    // example 1
    // let n = Mutex::new(0);
    // thread::scope(|s| {
    //     for _ in 0..10 {
    //         s.spawn(|| {
    //             let mut guard = n.lock().unwrap();
    //             for _ in 0..100 {
    //                 *guard += 1;
    //                 println!("guard: {guard}");
    //             }
    //             drop(guard);
    //             thread::sleep(Duration::from_secs(1));
    //         });
    //     }
    // });
    // assert_eq!(n.into_inner().unwrap(), 1000);

    // example2
    // let queue = Mutex::new(VecDeque::new());
    //
    // thread::scope(|s| {
    //     let t = s.spawn(|| loop {
    //         let item = queue.lock().unwrap().pop_front();
    //         if let Some(item) = item {
    //             dbg!(item);
    //             // println!("item: {item}");
    //         } else {
    //             thread::park();
    //         }
    //     });
    //
    //     for i in 0.. {
    //         queue.lock().unwrap().push_back(i);
    //         t.thread().unpark();
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    //

    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();
                let item = loop {
                    if let Some(item) = q.pop_front() {
                        break item;
                    } else {
                        q = not_empty.wait(q).unwrap();
                    }
                };
                drop(q);
                dbg!(item);
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            // not_empty.notify_all();

            thread::sleep(Duration::from_secs(1));
        }

    });

}

fn f() {
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}