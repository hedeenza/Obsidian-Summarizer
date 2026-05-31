use clap::Parser;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, self, Write};
use std::process::ExitCode;

mod stop_words;
use crate::stop_words::clean_stop_words;

mod python_summary;
use crate::python_summary::run_python_summarizer;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CLI {
    /// Input text file to summarize
    #[arg(short, long)]
    input: String,
    
    /// Input text file to summarize
    #[arg(short, long)]
    output: String,

    /// Number of sentences in Summary
    #[arg(short, long)]
    summary_length: u8,
}


fn main() -> ExitCode {

    // Parse CLI arguments
    let args = CLI::parse();

    // Read file
    let summary = run_python_summarizer(&args.input, args.summary_length);

    // Create an empty string to contain the "linked" text.
    let mut linked_text = String::new();

    // Create Regex pattern to match words that start with a capital letter
    let capital_detect = Regex::new(r"^[A-Z]").unwrap();

    // Split text on white space and collect into a vector
    let mut text_vec: Vec<&str> = summary.split(" ").collect();
    
    // The window will be 4 wide, so add 3 "trailing filler" values so the last word can fall into place
    text_vec.push("trailing filler");
    text_vec.push("trailing filler");
    text_vec.push("trailing filler");

    // Create a HashSet to contain a unique set of all words that have previously been linked
    // This will help prevent the same terms from getting linked over and over again
    let mut previously_linked: HashSet<Vec<Vec<String>>> = HashSet::new();

    // Create a window to examien the text vector 4 words at a time
    let mut chunks = text_vec.windows(4);

    // While there are still viable "windows"...
    while let Some(index) = chunks.next() {
        // Create values to stand in for the true/false results of matches to keep if statements cleaner
        let w = capital_detect.is_match(&index[0]);
        let x = capital_detect.is_match(&index[1]);
        let y = capital_detect.is_match(&index[2]);
        let z = capital_detect.is_match(&index[3]);

        // Create vectors to hold variants of the linked items to prevent them from being linked
        // again if they appear again in other written contexts
        let mut outer_comma = Vec::new();
        let mut comma_vec = Vec::new();
        let mut outer_strip_comma = Vec::new();
        let mut strip_comma_vec = Vec::new();
        let mut outer_period = Vec::new();
        let mut period_vec = Vec::new();
        let mut outer_strip_period = Vec::new();
        let mut strip_period_vec = Vec::new();
        let mut outer_possessive = Vec::new();
        let mut possessive_vec = Vec::new();
        let mut outer_strip_possessive = Vec::new();
        let mut strip_possessive_vec = Vec::new();

        // If all 4 slots contain capital words...
        if w & x & y & z | w & (index[1] == "of") & y & z | w & x & (index[2] == "of") & z {
            // Create an empty vector to hold the values to compare to
            let mut tester = Vec::new();
            // Push the values in the window to the testing vector
            let slice: Vec<String> = vec![
                index[0].to_string(),
                index[1].to_string(),
                index[2].to_string(),
                index[3].to_string(),
            ];
            tester.push(slice.clone());

            // If this has already been linked, move the window to skip it
            if previously_linked.contains(&tester) {
                for word in slice {
                    linked_text.push_str(&word);
                    linked_text.push(' ');
                }
                chunks.next();
                chunks.next();
                chunks.next();
            // Otherwise...
            } else {
                // For each 
                for (i, _content) in slice.clone().into_iter().enumerate() {
                    // Add the opening linking brackets, [[, to the first word
                    if i == 0 {
                        let linked_word = format!("[[{}", index[i]);
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked_text.push_str(&(linked_line + " "));
                    // Add the closing linking brackets, ]], to the last word
                    } else if i == slice.len() - 1 {
                        // If there's a possessive, add the closing linking brackets before the 's
                        if index[i].contains("'s") {
                            let stripped_word = index[i].replace("'s", "");
                            let linked_wordi = format!("{}]]'s", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        // If there's a comma, add the closing linking brackets before the comma
                        } else if index[i].contains(",") {
                            let stripped_word = index[i].replace(",", "");
                            let linked_wordi = format!("{}]],", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        // If there's a period, add the closing linking brackets before the period
                        } else if index[i].contains(".") {
                            let stripped_word = index[i].replace(".", "");
                            let linked_wordi = format!("{}]].", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        // Otherwise, link the last word normally
                        } else {
                            let linked_wordi = format!("{}]]", index[i]);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        }
                    // Otherwise don't add any linking brackets to the word
                    } else {
                        let linked_word = format!("{}", index[i]);
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked_text.push_str(&(linked_line + " "));
                    }

                    let strip_comma = index[i].replace(",", "");
                    strip_comma_vec.push(strip_comma);
                    let strip_period = index[i].replace(".", "");
                    strip_period_vec.push(strip_period);
                    let strip_possessive = index[i].replace("'s", "");
                    strip_possessive_vec.push(strip_possessive);
                }

                // Add the tester contents to the previously linked items
                previously_linked.insert(tester);
                // Move the window past all the words in this window
                chunks.next();
                chunks.next();
                chunks.next();

                let linked_comma = format!("{} {},", index[0], index[1]);
                comma_vec.push(linked_comma);
                outer_comma.push(comma_vec);

                let linked_period = format!("{} {}.", index[0], index[1]);
                period_vec.push(linked_period);
                outer_period.push(period_vec);

                let linked_possessive = format!("{} {}'s", index[0], index[1]);
                possessive_vec.push(linked_possessive);
                outer_possessive.push(possessive_vec);

                outer_strip_comma.push(strip_comma_vec);
                outer_strip_period.push(strip_period_vec);
                outer_strip_possessive.push(strip_possessive_vec);

                previously_linked.insert(outer_comma);
                previously_linked.insert(outer_strip_comma);
                previously_linked.insert(outer_period);
                previously_linked.insert(outer_strip_period);
                previously_linked.insert(outer_possessive);
                previously_linked.insert(outer_strip_possessive);

            }
        // If the first 3 slots contain capital words...
        } else if w & x & y | w & (index[1] == "of") & y | w & (index[1] == "the") & y | w & (index[1] == "for") & y {
            let mut tester = Vec::new();
            let slice: Vec<String> = vec![
                index[0].to_string(),
                index[1].to_string(),
                index[2].to_string(),
            ];
            tester.push(slice.clone());
            if previously_linked.contains(&tester) {
                for word in slice {
                    linked_text.push_str(&word);
                    linked_text.push(' ');
                }
                chunks.next();
                chunks.next();
            } else {
                for (i, _content) in slice.clone().into_iter().enumerate() {
                    if i == 0 {
                        let linked_word = format!("[[{}", index[i]);
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked_text.push_str(&(linked_line + " "));
                    } else if i == slice.len() - 1 {
                        if index[i].contains("'s") {
                            let stripped_word = index[i].replace("'s", "");
                            let linked_wordi = format!("{}]]'s", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        } else if index[i].contains(",") {
                            let stripped_word = index[i].replace(",", "");
                            let linked_wordi = format!("{}]],", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        } else if index[0].ends_with(".") {
                            let stripped_word = index[0].replace(".", "");
                            let linked_word0 = format!("[[{}]].", stripped_word);
                            let linked_line0 = index[0].replacen(index[0], &linked_word0, 1);
                            linked_text.push_str(&(linked_line0 + " "));
                        } else {
                            let linked_wordi = format!("{}]]", index[i]);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        }
                    } else {
                        let linked_word = format!("{}", index[i]);
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked_text.push_str(&(linked_line + " "));
                    }
                    let strip_comma = index[i].replace(",", "");
                    strip_comma_vec.push(strip_comma);
                    let strip_period = index[i].replace(".", "");
                    strip_period_vec.push(strip_period);
                    let strip_possessive = index[i].replace("'s", "");
                    strip_possessive_vec.push(strip_possessive);
                }
                previously_linked.insert(tester);
                chunks.next();
                chunks.next();

                let linked_comma = format!("{} {},", index[0], index[1]);
                comma_vec.push(linked_comma);
                outer_comma.push(comma_vec);

                let linked_period = format!("{} {}.", index[0], index[1]);
                period_vec.push(linked_period);
                outer_period.push(period_vec);

                let linked_possessive = format!("{} {}'s", index[0], index[1]);
                possessive_vec.push(linked_possessive);
                outer_possessive.push(possessive_vec);

                outer_strip_comma.push(strip_comma_vec);
                outer_strip_period.push(strip_period_vec);
                outer_strip_possessive.push(strip_possessive_vec);

                previously_linked.insert(outer_comma);
                previously_linked.insert(outer_strip_comma);
                previously_linked.insert(outer_period);
                previously_linked.insert(outer_strip_period);
                previously_linked.insert(outer_possessive);
                previously_linked.insert(outer_strip_possessive);
            }
        // If the first 2 slots contain capital words...
        } else if w & x {
            let mut tester = Vec::new();
            let slice: Vec<String> = vec![index[0].to_string(), index[1].to_string()];
            tester.push(slice.clone());
            if previously_linked.contains(&tester) {
                for word in slice {
                    linked_text.push_str(&word);
                    linked_text.push(' ');
                }
                chunks.next();
            } else {

                for (i, _content) in slice.clone().into_iter().enumerate() {
                    if i == 0 {
                        let linked_word = format!("[[{}", index[i]);
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked_text.push_str(&(linked_line + " "));
                    } else if i == slice.len() - 1 {
                        if index[i].contains("'s") {
                            let stripped_word = index[i].replace("'s", "");
                            let linked_wordi = format!("{}]]'s", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        } else if index[i].contains(",") {
                            let stripped_word = index[i].replace(",", "");
                            let linked_wordi = format!("{}]],", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        } else if index[0].ends_with(".") {
                            let stripped_word = index[0].replace(".", "");
                            let linked_word0 = format!("[[{}]].", stripped_word);
                            let linked_line0 = index[0].replacen(index[0], &linked_word0, 1);
                            linked_text.push_str(&(linked_line0 + " "));
                        } else {
                            let linked_wordi = format!("{}]]", index[i]);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        }
                    } else {
                        let linked_word = format!("{}", index[i]);
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked_text.push_str(&(linked_line + " "));
                    }

                    let strip_comma = index[i].replace(",", "");
                    strip_comma_vec.push(strip_comma);
                    let strip_period = index[i].replace(".", "");
                    strip_period_vec.push(strip_period);
                    let strip_possessive = index[i].replace("'s", "");
                    strip_possessive_vec.push(strip_possessive);
                }

                chunks.next();

                let linked_comma = format!("{} {},", index[0], index[1]);
                comma_vec.push(linked_comma);
                outer_comma.push(comma_vec);

                let linked_period = format!("{} {}.", index[0], index[1]);
                period_vec.push(linked_period);
                outer_period.push(period_vec);

                let linked_possessive = format!("{} {}'s", index[0], index[1]);
                possessive_vec.push(linked_possessive);
                outer_possessive.push(possessive_vec);

                outer_strip_comma.push(strip_comma_vec);
                outer_strip_period.push(strip_period_vec);
                outer_strip_possessive.push(strip_possessive_vec);

                previously_linked.insert(outer_comma);
                previously_linked.insert(outer_strip_comma);
                previously_linked.insert(outer_period);
                previously_linked.insert(outer_strip_period);
                previously_linked.insert(outer_possessive);
                previously_linked.insert(outer_strip_possessive);
            }
        // If only the first slots contains a capital word...
        } else if w {
            let mut tester = Vec::new();
            let slice: Vec<String> = vec![index[0].to_string()];
            tester.push(slice.clone());

            if previously_linked.contains(&tester) {
                for word in slice {
                    linked_text.push_str(&word);
                    linked_text.push(' ');
                }
                chunks.next();
            } else {

                if index[0].contains("'s") {
                    let stripped_word = index[0].replace("'s", "");
                    let linked_word0 = format!("[[{}]]'s", stripped_word);
                    let linked_line0 = index[0].replacen(index[0], &linked_word0, 1);
                    linked_text.push_str(&(linked_line0 + " "));
                } else if index[0].contains(",") {
                    let stripped_word = index[0].replace(",", "");
                    let linked_word0 = format!("[[{}]],", stripped_word);
                    let linked_line0 = index[0].replacen(index[0], &linked_word0, 1);
                    linked_text.push_str(&(linked_line0 + " "));
                } else if index[0].ends_with(".") {
                    let stripped_word = index[0].replace(".", "");
                    let linked_word0 = format!("[[{}]].", stripped_word);
                    let linked_line0 = index[0].replacen(index[0], &linked_word0, 1);
                    linked_text.push_str(&(linked_line0 + " "));
                } else {
                    let linked_word0 = format!("[[{}]]", index[0]);
                    let linked_line0 = index[0].replacen(index[0], &linked_word0, 1);
                    linked_text.push_str(&(linked_line0 + " "));
                }

                let linked_comma = format!("{},", index[0]);
                comma_vec.push(linked_comma);
                outer_comma.push(comma_vec);

                let strip_comma = index[0].replace(",", "");
                let linked_strip_comma = format!("{}", strip_comma);
                strip_comma_vec.push(linked_strip_comma);
                outer_strip_comma.push(strip_comma_vec);

                let linked_period = format!("{}.", index[0]);
                period_vec.push(linked_period);
                outer_period.push(period_vec);

                let strip_period = index[0].replace(".", "");
                let linked_strip_period = format!("{}", strip_period);
                strip_period_vec.push(linked_strip_period);
                outer_strip_period.push(strip_period_vec);

                let linked_possessive = format!("{}'s", index[0]);
                possessive_vec.push(linked_possessive);
                outer_possessive.push(possessive_vec);

                let strip_possessive = index[0].replace("'s", "");
                let linked_strip_possessive = format!("{}", strip_possessive);
                strip_possessive_vec.push(linked_strip_possessive);
                outer_strip_possessive.push(strip_possessive_vec);

                previously_linked.insert(tester);
                previously_linked.insert(outer_comma);
                previously_linked.insert(outer_strip_comma);
                previously_linked.insert(outer_period);
                previously_linked.insert(outer_strip_period);
                previously_linked.insert(outer_possessive);
                previously_linked.insert(outer_strip_possessive);
            }
        // If no slots contain capital words...
        } else {
            linked_text.push_str(&(index[0].to_owned() + " "));
        }
    }

    // Clean out the "stop words"
    clean_stop_words(&mut linked_text);

    // Print Preview of the Linked Summary
    println!("Summary:\n{}\n", linked_text);

    // Ask if User wants to save output to file
    let mut save_choice = String::new();
    println!("\nSave to file? (y/n)");
    io::stdin().read_line(&mut save_choice).expect("Failed to read line");

    // If the user does want to save, write to file
    if save_choice.trim() == "y" {
        // Create the output file
        let output_name = format!("{}.md", &args.output);
        let mut output_file = File::create(output_name).expect("Could not create output file");
        // Write Properties Header
        let _write_header = writeln!(output_file, "{}", "---\ntitle: \nauthor: \npublication-date: \naccess-date: \nlink: \ntags: \n---\n");
        // Write Output File Name as Document Title
        let title = format!("# {}", args.output);
        let _write_title = writeln!(output_file, "{}", title);
        // Write the Linked Summary
        let linked_summary = format!("## Summary:\n{}\n", linked_text);
        let _write_summary = writeln!(output_file, "{}", linked_summary);
        // Write the Original Text to the Output
        let input_file = File::open(&args.input).unwrap();
        let input_reader = BufReader::new(input_file);
        let _write_original_header = writeln!(output_file, "{}", "## Original Text:");
        for line in input_reader.lines() {
            match line {
                Ok(line) => { let _write_original = writeln!(output_file, "{}", line); }
                Err(err) => { println!("[ ERROR ] : {}", err); }
            }
        }
        // Exit with Success
        ExitCode::from(0)
    // If user does not want to save, exit cleanly
    } else if save_choice.trim() == "n" {
        // Exit with Success
        ExitCode::from(0)
    // If input is invalid, exit with failure
    } else {
        // Exit with Failure
        println!("Invalid Input");
        ExitCode::from(1)
    }

}
