use std::{cell::{Cell, RefCell}, rc::Rc, sync::{Arc, Mutex}, thread};

static X:[i32; 3] = [1,2,3];

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

/*
    Scope guarantees that none of the threads spawn can outlive the scope
*/
pub fn scoped_threads() {
    println!("Scoped thread");
    let numbers = vec![1,2,3];
    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{}", n);
            }
        });
    })
}

/*
    Share ownership example 1
    Using static keyword
*/
pub fn share_ownership_example_one() {
    println!("Share ownership example 1");
    thread::spawn(|| dbg!(&X));
    thread::spawn(|| dbg!(&X));
}

/*
    Share ownership example 2
    Downside of leak() is that we are leaking memory, if we do it multiple times
    program will slowly run out of memory.
*/
pub fn share_ownership_example_two() {
    println!("Share ownership example 2");
    let x: &'static[i32; 3] = Box::leak(Box::new([1,2,3]));
    
    thread::spawn(move || dbg!(x));
    thread::spawn(move || dbg!(x));
}

/*
    Shared reference counting example 1
    Reference counting is used to RC will refer the same allocation,
    they share ownership.
    Not safe to share in a thread.
*/
pub fn share_ownership_example_ref_counter() {
    println!("Reference counter example 1");
    let a = Rc::new([1,2,3]);
    let b = a.clone();
    
    assert_eq!(a.as_ptr(), b.as_ptr());
}

/*
    Shared reference counting example 2
    Reference counting using Arc, is thread safe.
*/
pub fn share_ownership_example_ref_counter_thread_safe() {
    println!("Reference counter example 2");
    let a = Arc::new([1,2,3]);
    let b= a.clone();

    thread::spawn(move || {dbg!(a)});
    thread::spawn(move || {dbg!(b)});
}

/*
    Sharing reference without undefined behavior.
    Allow to cpy the value out if T is Copy.
    Replace value with another value as whole.
    Only safe for single-threaded access (no concurrent access).
*/

pub fn using_cell() {
    println!("Using cell example");
    let a = Cell::new(1);
    let b = Cell::new(1);
    thread::spawn(move || {
        for_cell(&a, &b)
    });
}

fn for_cell(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        println!("{} and {:?} and {}", before, b, after);
    }
}

/*
    RefCel allow to borrow content and modify it.
    Downside is runtime cost.
    Only safe for single-threaded access (no concurrent access).
*/
pub fn using_ref_cell() {
    println!("Using RefCel example");
    let a = RefCell::new(vec![1,2,3]);
    for_ref_cell(&a);
    println!("{:?}", a);
}

fn for_ref_cell(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(4);
}

/*
    Using Mutex allow to share a variable to multiple threads
    and access it and modify it using lock()
*/
pub fn using_mutex() {
    println!("Using mutex example");
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
            });
        }
    });

    assert_eq!(n.into_inner().unwrap(), 1000);
}