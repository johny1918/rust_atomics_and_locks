pub mod chapter1;

use chapter1::spawn_thread;
use chapter1::capture_values;
use chapter1::get_value_back_from_thread;
use chapter1::scoped_threads;
use chapter1::share_ownership_example_one;
use chapter1::share_ownership_example_two;
use chapter1::share_ownership_example_ref_counter;
use chapter1::share_ownership_example_ref_counter_thread_safe;
use chapter1::using_cell;
use chapter1::using_ref_cell;
use chapter1::using_mutex;
use chapter1::thread_parking_example;
use chapter1::condition_variable_on_thread_example;


pub fn chapter_one_example_container() {
    spawn_thread();
    capture_values();
    get_value_back_from_thread();
    scoped_threads();
    share_ownership_example_one();
    share_ownership_example_two();
    share_ownership_example_ref_counter();
    share_ownership_example_ref_counter_thread_safe();
    using_cell();
    using_ref_cell();
    using_mutex();
    //commented because its an infinite loop
    //thread_parking_example();
    //commented because its an infinite loop
    //condition_variable_on_thread_example();
}