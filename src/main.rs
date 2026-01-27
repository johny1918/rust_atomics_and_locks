mod ch1;

use crate::ch1::chapter1::spawn_thread;
use crate::ch1::chapter1::capture_values;
use crate::ch1::chapter1::get_value_back_from_thread;

fn main() {
    spawn_thread();
    capture_values();
    get_value_back_from_thread();
}