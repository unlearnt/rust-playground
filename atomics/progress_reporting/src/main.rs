use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;
use std::sync::atomic::AtomicU64;
// use std::sync::atomic::AtomicU32;
use std::time::Instant;
use rand::Rng;

fn generate_random_key() -> u64 {
    rand::thread_rng().random()
    // allocate_new_id()
}

// fn allocate_new_id() -> u64 {
//     static NEXT_ID: AtomicU64 = AtomicU64::new(0);
//     let mut id = NEXT_ID.load(Relaxed);
//     loop {
//         assert!(id < 1000, "too many IDs!");
//         match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
//             Ok(_) => return id,
//             Err(v) => id = v,
//         }
//     }
// }

fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Relaxed);
    if key == 0 {
        let new_key = generate_random_key();
        match KEY.compare_exchange(0, new_key, Relaxed, Relaxed) {
            Ok(_) => new_key,
            Err(k) => k,
        }
    } else {
        key
    }
}
fn process_item(i :usize) {
    let key = get_key();
    println!("Process item {i} and key: {key}");
    thread::sleep(Duration::from_millis(1000));
}

// fn get_x() -> u64 {
//     static X: AtomicU64 = AtomicU64::new(0);
//     let mux x = X.load(Relaxed);
//     if x == 0 {
//         x = calculate_x();
//         X.store(x, Relaxed);
//     }
//     x
// }

// part1
// fn main() {
//     let num_done = AtomicUsize::new(0);
//
//     thread::scope(|s| {
//         s.spawn(|| {
//             for i in 0..100 {
//                 process_item(i);
//                 num_done.fetch_add(1, Relaxed);
//             }
//         });
//
//         loop{
//             let n = num_done.load(Relaxed);
//             if n == 100 { break; }
//             println!("Working.. {n}/100 done");
//             thread::sleep(Duration::from_secs(1));
//         }
//
//         println!("Done!");
//     })
// }

// part2
// fn main() {
//     let num_done = &AtomicUsize::new(0);
//
//     thread::scope(|s| {
//         // Four background threads to process all 100 items, 25 each.
//         for t in 0..4 {
//             s.spawn(move || {
//                for i in 0..25 {
//                    process_item(t * 25 + i); // Assuming this takes some time.
//                    num_done.fetch_add(1, Relaxed);
//                }
//             });
//         }
//
//         // The main thread shows status updates, every second.
//         loop {
//             let n = num_done.load(Relaxed);
//             if n == 100 { break; }
//             println!("Working.. {n}/100 done");
//             thread::sleep(Duration::from_secs(1));
//         }
//
//     })
// }

// part3
fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        // Four background threads to process all 100 items, 25 each.
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 + i); // Assuming this takes some time.
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }

        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);
            if n == 100 { break; }
            if n == 0 {
                println!("Working.. nothing done yet.");
            } else {
                println!(
                    "Working.. {n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time,
                );
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("Done!")
}