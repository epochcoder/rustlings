// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus {
        jobs_completed: 0
    }));

    let status_shared = status.clone();

    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut job_status = status_shared.lock().unwrap();

            job_status.jobs_completed += 1;
            //println!("completed job ... {} ", job_status.jobs_completed);
        }
    });

    let mut count = 0;
    while status.lock().unwrap().jobs_completed < 10 {
        count += 1;
        println!("Waiting...");
        thread::sleep(Duration::from_millis(500));
    }
}
