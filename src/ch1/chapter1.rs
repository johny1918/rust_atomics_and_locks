use std::thread;


/*
    Basic example of spawn thread
*/
pub fn spawn_thread() {
    thread::spawn(f);
    thread::spawn(f);

    println!("Hello from the main thread.");
}

fn f() {
    println!("Hello from another thread");
    let id = thread::current().id();
    println!("This thread have id: {:?}", id);
}