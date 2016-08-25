// Based on the final version of the multiprint client/server/shared queue example.
// https://github.com/broesamle/RustWorkshop/blob/master/minimals/multiprint.md

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let mut threads = Vec::new();
    let queue: Vec<(u64, u64)> = Vec::new();
    let printqueue_mutex_arc = Arc::new(Mutex::new(queue));
    let serverqueue = printqueue_mutex_arc.clone();
    let server = thread::spawn(move || {
        loop {
            if let Ok(mut guard) = serverqueue.try_lock() {
                if let Some((x, y)) = (*guard).pop() {
                    println!("position {}, {}", x, y);
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    });
    for num in 0..10 {
        let clientqueue = printqueue_mutex_arc.clone();
        let handle = thread::spawn(move || {
            let mut i = 0;
            loop {
                if let Ok(mut guard) = clientqueue.try_lock() {
                    println!("putting: {:?}", (i, num));
                    (*guard).push((i, num));
                    i += 1;
                }
                thread::sleep(Duration::from_millis(500*(num+1)));
            }
        });
        threads.push(handle);
        println!("Started thread number {:?}.", num);
    }
    for num in (0..10).rev() {
        let thr = threads.remove(num);
        let joinresult = thr.join();
        println!("Joined thread number {:?}, {:?}.", num, joinresult);
    }
    let _ = server.join();
}
