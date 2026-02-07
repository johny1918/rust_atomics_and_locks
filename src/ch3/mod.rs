pub mod chapter_three;

use chapter_three::happens_before_relationship;
use chapter_three::release_and_aquire;

pub fn chapter_three_example_container() {
    happens_before_relationship();
    release_and_aquire();
}