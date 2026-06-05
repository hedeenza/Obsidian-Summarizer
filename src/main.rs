use clap::Parser;
use regex::Regex;
use std::collections::HashSet;
use std::io;
use std::env;
use std::process::ExitCode;

mod stop_words;
use crate::stop_words::clean_stop_words;

mod python_summary;
use crate::python_summary::run_python_summarizer;

mod link_entities;
use crate::link_entities::link_entities;

mod save_choice;
use crate::save_choice::save_choice;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input text file to summarize
    #[arg(short, long)]
    input: String,

    /// Output file name
    #[arg(short, long)]
    output: String,

    /// Number of sentences in Summary
    #[arg(short, long)]
    summary_length: u8,
    
    /// Group Tag, in CamelCase [default = None]
    #[arg(short, long, default_value = None)]
    group_tag: Option<String>,
}

fn main() -> ExitCode {
    // Parse CLI arguments
    let args = Cli::parse();

    // Set up to read the file containing Python venv and summary.py paths
    // Get user home directory
    let mut home_directory = String::new();
    match env::home_dir() {
        Some(path) => {
            home_directory.push_str(&path.display().to_string());
        }
        None => {
            panic!("Could not read your home directory");
        }
    }
    // Create the full path to the paths.env file
    let paths_env = home_directory + "/.obsidian_summarizer_paths.env";

    // Run the Python Summarizer script
    let summary = run_python_summarizer(&args.input, paths_env, args.summary_length);

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
    link_entities(
        text_vec,
        4,
        &mut linked_text,
        capital_detect,
        previously_linked,
    );

    // Clean out the "stop words"
    clean_stop_words(&mut linked_text);

    // Print Preview of the Linked Summary
    println!("Summary:\n{}\n", linked_text);

    // Ask if User wants to save output to file
    let mut decision = String::new();
    println!("\nSave to file? (y/n)");
    io::stdin()
        .read_line(&mut decision)
        .expect("Failed to read Answer");

    // Ask if User wants to save output to file
    // let mut group_tag = String::new();
    // println!("\nIf this is part of a group of files, please type the group tag (CamelCase)");
    // io::stdin()
    //     .read_line(&mut group_tag)
    //     .expect("Failed to read Group Tag");
    // let group_tag = group_tag.trim();
    // Handle Save Deicision
    save_choice(decision, args.input, args.output, linked_text, args.group_tag, summary)

}
