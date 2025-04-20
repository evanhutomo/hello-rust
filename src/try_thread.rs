use std::thread::{self, JoinHandle};

pub fn coba_thread() {
    let handle: JoinHandle<()> = thread::spawn(|| {
        for i in 1..5 {
            println!("--- HI number {} from the SPAWNED THREAD ---", i);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    // main thread continues execution
    for i in 1..3 {
        println!("--- HI number {} from the MAIN THREAD ---", i);
        thread::sleep(std::time::Duration::from_millis(1));
    }

    // wait for the spawned thread to finish
    handle.join().unwrap();
}
