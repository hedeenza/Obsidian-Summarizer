use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, ExitCode};

pub fn save_choice(
    decision: String,
    input_file: String,
    output_file: String,
    linked: String,
) -> ExitCode {
    // If the user does want to save, write to file
    if decision.trim() == "y" {
        // Create the output file
        let output_name = format!("{}.md", &output_file);
        let mut output_file = File::create(&output_name).expect("Could not create output file");
        // Write Properties Header
        let _write_header = writeln!(
            output_file,
            "{}",
            "---\ntitle: \nauthor: \npublication-date: \naccess-date: \nlink: \ntags: \n---\n"
        );
        // Write Output File Name as Document Title
        let title = format!("# {}", output_name);
        let _write_title = writeln!(output_file, "{}", title);
        // Write the Linked Summary
        let linked_summary = format!("## Summary:\n{}\n", linked);
        let _write_summary = writeln!(output_file, "{}", linked_summary);
        // Write the Original Text to the Output
        let input_file = File::open(&input_file).unwrap();
        let input_reader = BufReader::new(input_file);
        let _write_original_header = writeln!(output_file, "{}", "## Original Text:");
        for line in input_reader.lines() {
            match line {
                Ok(line) => {
                    let _write_original = writeln!(output_file, "{}", line);
                }
                Err(err) => {
                    println!("[ ERROR ] : {}", err);
                }
            }
        }
        // Remove the temporary "summary.txt" file
        let _remove = Command::new("rm")
            .arg("summary.txt")
            .spawn()
            .expect("Could not delete the temporary summary.txt file");
        // Exit with Success
        ExitCode::from(0)
        // If user does not want to save, exit cleanly
    } else if decision.trim() == "n" {
        // Remove the temporary "summary.txt" file
        let _remove = Command::new("rm")
            .arg("summary.txt")
            .spawn()
            .expect("Could not delete the temporary summary.txt file");
        // Exit with Success
        ExitCode::from(0)
        // If input is invalid, exit with failure
    } else {
        // Remove the temporary "summary.txt" file
        let _remove = Command::new("rm")
            .arg("summary.txt")
            .spawn()
            .expect("Could not delete the temporary summary.txt file");
        // Exit with Failure
        println!("Invalid Input");
        ExitCode::from(1)
    }
}
