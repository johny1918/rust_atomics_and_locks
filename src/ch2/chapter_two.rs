use std::{sync::atomic::{AtomicBool, Ordering::Relaxed}, thread, time::Duration};

pub fn example_stop_flag() {
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