mod ch1;

use crate::ch1::chapter1::spawn_thread;
use crate::ch1::chapter1::capture_values;
use crate::ch1::chapter1::get_value_back_from_thread;
use crate::ch1::chapter1::scoped_threads;
use crate::ch1::chapter1::share_ownership_example_one;
use crate::ch1::chapter1::share_ownership_example_two;

fn main() {
    spawn_thread();
    capture_values();
    get_value_back_from_thread();
    scoped_threads();
    share_ownership_example_one();
    share_ownership_example_two();
}