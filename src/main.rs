// Copyright 2019 Bernardo Ferrari.
// Licensed under the MIT license <https://opensource.org/licenses/MIT>.

use colored::*;

use safechangelog::{Event, event_to_str, play_modifier, retrieve_input};

fn print_result(description: &str, size: usize, distance: usize, errors_only: bool) -> Event {
    let (refactored_str, event) = play_modifier(description, size, distance);

    let localized_message = event_to_str(&event);

    if localized_message.is_empty() {
        if !errors_only {
            println!("[{}] {}", size, refactored_str);
        }
    } else {
        println!("[{}] {}\n{}", size, refactored_str, localized_message);
    }

    return event;
}

fn main() {
    let (description, size, distance, errors_only) = retrieve_input();

    if size <= 0 {
        // this will be used to say there was an error, so user don't need to scroll up every time.
        let mut error_event = Event::Nothing;

        for i in (56..110).step_by(1) {
            let result = print_result(&description, i, distance, errors_only);

            match result {
                Event::Shorter => {
                    error_event = result;
                    break;
                }
                Event::Error(_, _) => error_event = result,
                _ => (),
            }
        }

        match error_event {
            Event::Error(_, _) => println!("{}", "There was a problem!".red()),
            Event::Nothing => println!("{}", "Success!".green()),
            _ => (),
        }
    } else {
        print_result(&description, size, distance, errors_only);
    }
}
