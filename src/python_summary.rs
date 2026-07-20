#![warn(clippy::pedantic)]
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

pub fn run_python_summarizer(input: &String, paths: String, summary_length: usize) -> String {
    // Read input file
    let input_file = File::open(input).unwrap();
    let input_reader = BufReader::new(input_file);
    let mut text = String::new();

    // Push file lines to string
    for line in input_reader.lines() {
        match line {
            Ok(line) => {
                text.push_str(&line);
            }
            Err(err) => {
                println!("[ ERROR ] : {}", err);
            }
        }
    }

    // Read ~/.obsidian_summarizer_paths.env
    let mut venv_path = String::new();
    let mut script_path = String::new();
    let paths_file = File::open(&paths);
    match paths_file {
        Ok(file) => {
            let paths_reader = BufReader::new(file);
            for line in paths_reader.lines() {
                match line {
                    Ok(line) => {
                        if line.ends_with("venv") {
                            venv_path.push_str(&line);
                        } else if line.ends_with("summary.py") {
                            script_path.push_str(&line);
                        }
                    }
                    Err(err) => {
                        println!("[ ERROR ] : {}", err);
                    }
                }
            }
        }
        Err(_err) => {
            panic!("Error opening .obsidian_summarizer_paths.env")
        }
    }

    // Run Python from the Virtual Environment
    #[cfg(target_os = "windows")]
    let python = venv_path + "/Scripts/python.exe";
    #[cfg(not(target_os = "windows"))]
    let python = venv_path + "/bin/python";

    // Run the Python Summary Script
    let mut run_python = Command::new(python)
        .arg(script_path)
        .arg(input)
        .arg(summary_length.to_string())
        .spawn()
        .expect("Could not run Python summary script");

    // Wait for the Python Summary script to finish before continuing
    let _result = run_python
        .wait()
        .expect("Could not wait for Python script to complete");

    // Read in the Python script-generated Summary
    let summary_file = File::open("summary.txt").unwrap();
    let summary_reader = BufReader::new(summary_file);
    let mut summary = String::new();

    // Push file lines to string
    for line in summary_reader.lines() {
        match line {
            Ok(line) => {
                summary.push_str(&line);
            }
            Err(err) => {
                println!("[ ERROR ] : {}", err);
            }
        }
    }
    summary
}
