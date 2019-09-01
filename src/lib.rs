// Copyright 2019 Bernardo Ferrari.
// Licensed under the MIT license <https://opensource.org/licenses/MIT>.

use std::fs::read_to_string;
use std::path::Path;
use clipboard::{ClipboardContext, ClipboardProvider};
use structopt::StructOpt;
use colored::*;

/// Welcome to Changelog Disaster Mitigator!
/// This tool helps you make sure your changelog messages won't confuse your users once Play Store modifies it.
/// You might supply an input path (via --path), only an input (via --input) or nothing, in which case it will read the clipboard.
/// You might also supply the size, in chars, that the screen will comport. In a Galaxy S9, it is 56 in portrait mode and 96 in landscape. This changes depending on screen size and font size settings. If you supply a value less than 0.
/// Examples:
/// cdmitigator -i "We update the Uber app as often as possible" -s 56
/// cdmitigator -p changelog.txt
/// cdmitigator -s96
#[derive(StructOpt, Debug)]
pub struct Cli {
    /// The path to the file that is going be read
    #[structopt(short, long, parse(from_os_str))]
    path: Option<std::path::PathBuf>,

    /// The input that is going to be read
    #[structopt(short, long)]
    input: Option<String>,

    /// The total size in chars for the output. Usually ranges from 50 to 110.
    #[structopt(short, long, default_value = "60")]
    size: usize,
}

fn make_new_string(description: &str, start_size: usize, end_size: usize) -> (String, ColoredString) {
    let (end_index, _) = description.char_indices().rev().nth(end_size).unwrap();
    let start = &description[..start_size];
    let end = &description[end_index..];

    let error = if end.contains("https://") || end.contains("http://") {
        // in Medium case, the url can become https://reddit.com or dit.com/r/MediumApp/. Both weird.
        String::from("[WARNING!] Web address detected. Play Store might shrink it.").yellow()
    } else {
        String::from("[Ok]").green()
    };

    ([start.trim_end(), "...", end.trim_start()].concat(), error)
}

fn read_clipboard() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.get_contents().expect("Failed to open clipboard")
}

fn read_file(path: &Path) -> String {
    read_to_string(path).unwrap_or_else(|_| panic!("Failed to open {}", path.display()))
}

pub fn retrieve_input() -> (String, usize) {
    let opt = Cli::from_args();

    // tries to find a path input, else a standard input, else just read the clipboard.
    return (match opt.path {
        Some(path) => read_file(&path),
        None => match opt.input {
            Some(input) => input,
            None => read_clipboard()
        }
    }, opt.size);
}

pub fn play_modifier(description: &str, size: usize) -> (String, ColoredString) {
    let len = description.len();

    if len < size {
        (description.parse().unwrap(), String::from("[Ok] Returning the same input.").green())
    } else if description[..len / 2].contains('\n') {
        // in changelogs with newlines in the first half of the string, it was detected that
        // play store shrinks it differently. This tries to mimic it.
        let one_third_size = size / 3;
        make_new_string(description, one_third_size * 2, one_third_size)
    } else {
        // the standard split.
        make_new_string(description, size / 2, size / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let description = "We update the Uber app as often as possible to help make it faster and more reliable for you. This version includes several bug fixes and performance improvements.

        Love the app? Rate us! Your feedback helps us to improve the Uber app.
        Have a question? Tap Help in the Uber app or visit help.uber.com.";
        let (value, _) = play_modifier(&description, 60);

        assert_eq!(
            value,
            "We update the Uber app as ofte...ber app or visit help.uber.com."
        );
    }

    #[test]
    fn two_result() {
        let description = "Bug fixes and improvements

        We'd like to invite you to help shape the future of the app by providing valuable feedback. Come join our Android community here https://www.reddit.com/r/MediumApp/.";
        let (value, _) = play_modifier(&description, 60);

        assert_eq!(
            value,
            "Bug fixes and improvements\n\n        We\'d...dit.com/r/MediumApp/."
        );
    }
}
