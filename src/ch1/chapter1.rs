use std::thread;


/*
    Basic example of spawn thread
*/
pub fn spawn_thread() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread.");

    /*
        .join() will wait for threads to finish even if main finish first.
     */
    t1.join().unwrap();
    t2.join().unwrap();
}

fn f() {
    println!("Hello from another thread");
    let id = thread::current().id();
    println!("This thread have id: {:?}", id);
}