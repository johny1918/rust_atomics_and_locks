use std::{sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering::Relaxed}, thread, time::{Duration, Instant}};


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


/*
    Example of atomic operation with thread park() and unpark()
    This time main thread gets unpark() on every update of i
*/
pub fn example_atomic_syncronization_using_park() {
    let status = AtomicUsize::new(0);

    let main_thread = thread::current();

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100 {
                thread::sleep(Duration::from_millis(100));
                status.store(i+1, Relaxed);
                main_thread.unpark();
            }
        });
        
        loop {
            let n = status.load(Relaxed);
            if n == 100 { break ;}
            println!("Working.. {n}/100 done");
            thread::park_timeout(Duration::from_secs(1));
        }
    });
    println!("Done");
}

/*
    Example of reporting progress from multiple threads
    using fetch_add on atomic values.
*/
pub fn example_of_fetch_add_from_multiple_threads() {
    let status = &AtomicUsize::new(0);

    //create thread scope
    thread::scope(|s| {
        //spawn 4 threads
        for t in 0..4 {
            //move threads
            s.spawn(move || {
                for i in 0..25 {
                    println!("thread {} (id {:?}) works", t, thread::current().id());
                    thread::sleep(Duration::from_millis(t * 100 + i));
                    //add 1 to status atomic
                    status.fetch_add(1, Relaxed);
                }
            });
        }

        loop {
            let n = status.load(Relaxed);
            if n == 100 {break ;}
            println!("Working.. {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("Done")
}

/*
    Example of atomic operations with 
    fetch_add - used to add value to atomic value
    fetch_max - used to determin max time from atomic value
*/
pub fn example_atomic_operations_total_and_max_time() {
    let status = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    //create thread scope
    thread::scope(|s| {
        for t in 0..4 {
            //spawn 4 copy threads
            s.spawn(move || {
                for i in 0..25 {
                    //get time
                    let start = Instant::now();
                    thread::sleep(Duration::from_millis(t + 300 + i));
                    //get elapsed time since start
                    let time_taken = start.elapsed().as_micros() as u64;
                    //add 1 to atomic value
                    status.fetch_add(1, Relaxed);
                    // add time taken to previous time_taken value
                    total_time.fetch_add(time_taken, Relaxed);
                    // get max value from time taken
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }

        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));
            let n = status.load(Relaxed);
            if n == 100 {break ;}
            if n == 0 {
                println!("Working.. nothing done yet.");
            }
            else {
                println!(
                    "Working.. {n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time
                );
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("Done");
}

/*
    Example of using compare_exchange on atomic operations
*/
fn increment(a: &AtomicU64) {
    let mut current = a.load(Relaxed);

    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(_) => return,
            Err(v) => current = v,
        }
    }
}

/*
    Example showing how to generate a raondomly key thats is 
    generated just one time per run of program.
    This is known as lazy one-time initialization
 */
fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Relaxed);
    if key == 0 {
        let new_key = 1; // here we generate a new key
        match KEY.compare_exchange(0, new_key, Relaxed, Relaxed) {
            Ok(_) => new_key,
            Err(k) => k,
        }
    }
    else {
        key
    }
}




/*
    Summary of chapter 2
        • Atomic operations are indivisable; they have either fully completed, or they
        haven’t happened yet.
        • Atomic operations in Rust are done through the atomic types in
        std::sync::atomic, such as AtomicI32.
        • Not all atomic types are available on all platforms.
        • The relative ordering of atomic operations is tricky when multiple variables are
        involved. More in Chapter 3.
        • Simple loads and stores are nice for very basic inter-thread communication, like
        stop flags and status reporting.
        • Lazy initialization can be done as a race, without causing a data race.
        • Fetch-and-modify operations allow for a small set of basic atomic modifications
        that are especially useful when multiple threads are modifying the same atomic
        variable.
        • Atomic addition and subtraction silently wrap around on overflow.
        • Compare-and-exchange operations are the most flexible and general, and a
        building block for making any other atomic operation.
        • A weak compare-and-exchange operation can be slightly more efficient.
*/