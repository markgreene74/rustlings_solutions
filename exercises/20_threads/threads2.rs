// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

// https://doc.rust-lang.org/std/sync/
// https://doc.rust-lang.org/book/ch16-03-shared-state.html
// https://doc.rust-lang.org/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            //status_shared.jobs_completed += 1;
            status_shared.lock().unwrap().jobs_completed += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete
    for handle in handles {
        handle.join().unwrap();
        // println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }

    // TODO: Print the value of `JobStatus.jobs_completed`
    println!("jobs completed {}", status.lock().unwrap().jobs_completed);
}
