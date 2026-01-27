mod ch1;

use crate::ch1::chapter1::spawn_thread;
use crate::ch1::chapter1::capture_values;

fn main() {
    spawn_thread();
    capture_values();
}