use std::thread;
use std::sync::atomic::{AtomicBool,  AtomicU64, AtomicI32};
use std::sync::atomic::Ordering::{Relaxed, Acquire, Release};
use std::time::Duration;

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);


//happens-before relationship
pub fn happens_before_relationship() {
    
    thread::scope(|t| {
        t.spawn(|| {
            a();
        });
        t.spawn(|| {
            b();
        });
    });

    
    
}

fn a() {
    X.store(10, Relaxed);
    Y.store(20, Relaxed);
}

fn b() {
    let y = Y.load(Relaxed);
    let x = X.load(Relaxed);
    println!("{} {}", x, y);
}

//release and acquire ordering
pub fn release_and_aquire() {
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        READY.store(true, Release); // everything from before this store
    });

    while !READY.load(Acquire) { // is visible after this load 'true'
        thread::sleep(Duration::from_millis(100));
        println!("waiting");
    }
    println!("{}", DATA.load(Relaxed));
}