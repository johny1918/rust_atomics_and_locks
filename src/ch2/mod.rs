pub mod chapter_two;

use chapter_two::example_atomic;
use chapter_two::example_atomic_show_progress;
use chapter_two::example_atomic_syncronization_using_park;
use chapter_two::example_of_fetch_add_from_multiple_threads;

pub fn chapter_two_example_container() {
    //commenting because its endless loop waiting for user input
    //example_atomic();
    //commenting because its taking couple of seconds to execute
    //example_atomic_show_progress();
    //example_atomic_syncronization_using_park();
    example_of_fetch_add_from_multiple_threads();
}