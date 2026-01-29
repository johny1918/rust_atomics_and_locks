use std::{sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed}, thread, time::Duration};


/*
    Example of using atomic operations, this example shows
    use of AtomicBool and store and load
*/
pub fn example_atomic() {
    static STOP: AtomicBool = AtomicBool::new(false);
    
    // spawn a thread that will change STOP value after 2s
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        STOP.store(true, Relaxed);
    });


    //create a thread
    let background_thread = thread::spawn(|| {
        //loads value of STOP in current thread
        while !STOP.load(Relaxed) {
            println!("Doing some work!");
        }
    });


    //capture stdin input from terminal
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unkown command: {:?}", cmd),
        }
    }

    STOP.store(true,Relaxed);
    background_thread.join().unwrap();

}

/*
    This example show use of AtomicUsize where two threads runs
    concurrently showing progress of first thread.
*/
pub fn example_atomic_show_progress() {
    let status = AtomicUsize::new(0);
    //create thread scope and run the main loop inside it so both run concurrently
    thread::scope(|s| {
        //spawn thread and store value of i + 1
        s.spawn(|| {
            for i in 0..100 {
                println!("i = {}", i);
                thread::sleep(Duration::from_millis(400));
                status.store(i + 1, Relaxed);
            }
        });

        //main thread shows status updates, every second
        loop {
            let n = status.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {}/100 done", n);
            thread::sleep(Duration::from_millis(800));
        }
    });

    println!("Done!");
}