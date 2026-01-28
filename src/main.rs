mod ch1;

use crate::ch1::chapter1::spawn_thread;
use crate::ch1::chapter1::capture_values;
use crate::ch1::chapter1::get_value_back_from_thread;
use crate::ch1::chapter1::scoped_threads;
use crate::ch1::chapter1::share_ownership_example_one;
use crate::ch1::chapter1::share_ownership_example_two;
use crate::ch1::chapter1::share_ownership_example_ref_counter;
use crate::ch1::chapter1::share_ownership_example_ref_counter_thread_safe;
use crate::ch1::chapter1::using_cell;
use crate::ch1::chapter1::using_ref_cell;
use crate::ch1::chapter1::using_mutex;
use crate::ch1::chapter1::thread_parking_example;


fn main() {
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
    thread_parking_example();
}