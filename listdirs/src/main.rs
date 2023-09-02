use std::fs;
use std::sync::mpsc;
use std::thread;

/// Spawn a thread for each encountered directory in str, and prints its name and thread ID.
fn recursive_scan_dir(start: &str, tx: mpsc::Sender<thread::JoinHandle<()>>) {
    let entries = fs::read_dir(start).unwrap();

    for entry in entries {
        let e = entry.unwrap().path();

        if e.is_dir() {
            let tx1 = tx.clone();

            thread::spawn(move || {
                println!("directory: {}, thread: {}", e.display(), thread_id::get());
                recursive_scan_dir(e.as_path().to_str().unwrap(), tx1);
            });
        }
    }
}

/// Starts recursing directories with threads, and wait for them to finish.
fn main() {
    let (tx, rx): (
        mpsc::Sender<thread::JoinHandle<()>>,
        mpsc::Receiver<thread::JoinHandle<()>>,
    ) = mpsc::channel();

    recursive_scan_dir("./", tx);

    for received in rx {
        received.join().unwrap();
    }
}
