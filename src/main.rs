// Copyright 2019 Bernardo Ferrari.
// Licensed under the MIT license <https://opensource.org/licenses/MIT>.

use cdmitigator::{play_modifier, retrieve_input};
use rand::Rng;

fn main() {
    let (description, mut size) = retrieve_input();

    // generate a random size if it is equal or less than zero
    if size <= 0 {
        let mut rng = rand::thread_rng();
        size = rng.gen_range(46, 120);
    }

    let (result, error) = play_modifier(&description, size);
    println!("{}\n{}", result, error);
}
