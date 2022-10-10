use std::thread;
use std::time::Duration;

fn thread_example() {
    let secondary_thread = thread::spawn(|| {
        for i in 1..5 {
            // print i, then sleep thread for 2 milliseconds
            println!("Secondary Thread Prints {}", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..5 {
        // print i, then sleep thread for 2 milliseconds
        println!("Main Thread Prints {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    secondary_thread.join().unwrap();
}

use std::sync::mpsc;
fn message_passing_example() {
    // create a channel
    let (sender, receiver) = mpsc::channel();
    // create a new thread, create a value, send the value using the channel to main thread
    thread::spawn(move || {
        let val = String::from("I was created in the child thread, will be sent to main thread");
        sender.send(val).unwrap();
    });
    // receive and print the message from the child thread
    let received = receiver.recv().unwrap();
    println!("I have received this message from the child thread: {}", received);
}

use std::sync::{Arc, Mutex};
fn shared_state_example() {
    // create the mutex to have shared state
    // Wrap it in an Arc object to share safely
    let shared_state = Arc::new(Mutex::new(0));
    // create a vector to hold all the threads
    let mut threads = vec![];
    // loop 16 times, create a thread on each loop, that uses the mutex
    for _ in 0..16 {
        // create an atomic copy of the shared state
        let shared_state = Arc::clone(&shared_state);
        let child_thread = thread::spawn(move || {
            // lock the shared state in this thread
            let mut num = shared_state.lock().unwrap();
            // mutate the shared_state
            *num += 1;
        });
        // push thread into vector
        threads.push(child_thread);
    }
    // make sure to wait for all threads to complete
    for child_thread in threads {
        child_thread.join().unwrap();
    }
    // the lock shared state and print it in the main thread
    println!("Result: {}", *shared_state.lock().unwrap());
}



fn main() {
    thread_example();
    message_passing_example();
    shared_state_example();
}