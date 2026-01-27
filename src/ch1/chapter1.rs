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

/*
    Spawn a thread and move the ownership of numbers while capture the values
*/
pub fn capture_values() {
    let numbers = vec![1,2,3];
    thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
    }).join().unwrap();
}

/*
    Spawn a thread and move it, then return its value to main thread by using join()
*/
pub fn get_value_back_from_thread() {
    let numbers = Vec::from_iter(0..=1000);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len // Value returned by moved thread
    });

    let average = t.join().unwrap();

    println!("average: {}", average);
}