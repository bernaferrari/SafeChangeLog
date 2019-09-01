// Copyright 2019 Bernardo Ferrari.
// Licensed under the MIT license <https://opensource.org/licenses/MIT>.

use colored::*;

use cdmitigator::{Event, event_to_str, play_modifier, retrieve_input};

fn print_result(description: &str, size: usize, bad_word_detector: bool) -> Event {
    let (result, event) = play_modifier(description, size, bad_word_detector);

    let localized_message = event_to_str(&event);

    if localized_message.is_empty() {
        println!("[{}] {}", size, result);
    } else {
        println!("[{}] {}\n{}", size, result, localized_message);
    }

    return event;
}

fn main() {
    let (description, size) = retrieve_input();

    if size <= 0 {
        // this will be used to say there was an error, so user don't need to scroll up every time.
        let mut error_event = Event::Nothing;

        for i in (56..110).step_by(2) {
            let result = print_result(&description, i, true);

            match result {
                Event::Ok => break,
                Event::Error(_, _) => error_event = result,
                _ => (),
            }
        }

        match error_event {
            Event::Error(_, _) => println!("{}", "There was a problem!".red()),
            _ => (),
        }
    } else {
        print_result(&description, size, true);
    }
}
