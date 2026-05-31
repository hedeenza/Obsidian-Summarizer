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

mod link_entities;
use crate::link_entities::link_entities;

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

    // Run the Python Summarizer script
    let summary = run_python_summarizer(&args.input, args.summary_length);

    // Create an empty string to contain the "linked" text.
    let mut linked_text = String::new();
    
    // Create a HashSet to contain a unique set of all words that have previously been linked
    // This will help prevent the same terms from getting linked over and over again
    let previously_linked: HashSet<Vec<Vec<String>>> = HashSet::new();

    // Create Regex pattern to match words that start with a capital letter
    let capital_detect = Regex::new(r"^[A-Z]").unwrap();

    // Split text on white space and collect into a vector
    let mut text_vec: Vec<&str> = summary.split(" ").collect();
    
    // The window will be 4 wide, so add 3 "trailing filler" values so the last word can fall into place
    text_vec.push("trailing filler");
    text_vec.push("trailing filler");
    text_vec.push("trailing filler");

    // Wrap entities in Obsidian-style Links
    link_entities(text_vec, 4, &mut linked_text, capital_detect, previously_linked);

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
