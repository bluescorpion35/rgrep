// Imports. BufRead is for reading files into a buffer
use std::process::exit;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

// Use various enums for options
use crate::CLOption::{NoHighlight, PrintUsage, ShowLineNumbers, UseRegex};

// Read the file line by line into a vec
fn read_file(path: &String) -> Result<Vec<String>, Box<dyn Error>> {
    // Open the file
    let file = File::open(path);
    // Unwrap the file, match statements like this are to avoid .expect() and .unwrap() for better
    // error handling
    let file_unwrapped = match file {
        Ok(file_unwrapped_result) => file_unwrapped_result,
        Err(_error) => return Err("No such file or directory".into()),
    };
    // Read the file into a buffer
    let buf = BufReader::new(file_unwrapped);
    // Convert the buf into a vector and unwrap it
    match buf.lines().collect() {
        Ok(file) => Ok(file),
        Err(error) => Err(error.into())
    }
}
// Parse the arguments. Should have used clap...
fn parse_args(args: Vec<String>) -> Result<(Vec<String>, Vec<CLOption>), Box<dyn Error>> {
    // Initialise vars
    let mut args_parsed: Vec<&String> = Vec::new();
    let mut arg_num = 0;
    let mut options: Vec<CLOption> = vec![];
    // If it is the first arg, skip. If not check if it starts with - parses it.
    for arg in &args {
        if arg_num == 0 {
            arg_num += 1;
            continue;
        } else if arg.starts_with('-') {
            // Parse options. Another unwrap
            match CLOption::parse_options(arg) {
                Ok(option) => options.extend(option),
                Err(error) => return Err(error),
            }
            // If it does not start with - then push it as a regular arg, to path and pattern
        } else {
            args_parsed.push(arg)
        }
        arg_num += 1;
    }
    // If -h is used, this is ignored (-h is prioritised). Will kill the program if the args are
    // wrong
    if args_parsed.len() != 2 && !options.contains(&PrintUsage) {
        return Err("Invalid usage. Use -h for usage".into());
    }
    // Return the args as a Vec<String>
    Ok((args_parsed.iter().map(|x| x.to_string()).collect(), options))
}
// CLI option enums. Debug is for :? in print. I have no clue what PartialEq is for
#[derive(PartialEq, Debug)]
enum CLOption {
    ShowLineNumbers,
    NoHighlight,
    PrintUsage,
    UseRegex,
}
// Parse the options
impl CLOption {
    fn parse_options(arg: &str) -> Result<Vec<CLOption>, Box<dyn Error>> {
        // Declares the options. If invalid is set to true the args get returned as invalid
        let mut options: Vec<CLOption> = Vec::new();
        let mut invalid = false;
        // Repeat for each letter. Use a match to check if it is valid
        for char in arg.chars() {
            match char {
                'l' => options.push(ShowLineNumbers),
                'h' => options.push(PrintUsage),
                'n' => options.push(NoHighlight),
                'r' => options.push(UseRegex),
                '-' => (),
                _ => invalid = true,
            };
        }
        // Checks if the args are invalid and error or args based on that
        if invalid {
            Err(format!("Invalid Option: {}", arg).into())
        } else {
            Ok(options)
        }
    }
}

/*                                     Main function                                     */
// Returns error type, haven't figured it out yet
fn main() -> Result<(), Box<dyn Error>> {
    // Collect and parse args
    let args: Vec<String> = std::env::args().collect();
    let args_tup = parse_args(args);
    // Yet another error handling match
    let args_parsed = match args_tup {
        Ok(args) => args,
        Err(error) => return Err(error),
    };

    // args are returned as a tuple, the first part for the args, and the second part
    // for the options
    let options = args_parsed.1;
    let args = args_parsed.0;

    // Regex is not implemented and probably never will be
    if options.contains(&UseRegex) {
        return Err("Regex not implemented".into());
    }

    // Print usage and exit
    if options.contains(&PrintUsage) {
        println!("Usage: grepc <pattern> <path> [OPTIONS]");
        print!(
            "Options:\n-l: Show line numbers\n-n: \
        Don't highlight pattern\n-r Use regex (unimplemented)\
        \n-h: Print this usage message"
        );
        exit(0);
    }

    // Assign args to their own variables
    let path = &args[1];
    let pattern = &args[0];

    // Read the file path into a vec of strings
    let file = read_file(path);
    // Unwrap match
    let file_unwrapped: Vec<String> = match file {
        Ok(file_result_unwrapped) => file_result_unwrapped,
        Err(error) => return Err(error),
    };
    // If the text is highlighted then add red ANSI codes to strings
    // They will go before and after the pattern
    let mut highlight = ("", "");
    // If the options don't contain NoHighlight
    if !options.contains(&NoHighlight) {
        highlight.0 = "\x1b[1;31m";
        highlight.1 = "\x1b[0m";
    }
    // Iterate over the lines. Keep track of the line number in case -l was passed
    let mut line_number = 1;
    for line in file_unwrapped {
        // Check if the line contains the pattern
        if line.contains(pattern) {
            // A horrible if statement with an equally horrible println inside the first one
            if options.contains(&ShowLineNumbers) {
                println!(
                    // Print line number and also the text with the pattern replaced
                    "{}| {}",
                    line_number - 1,
                    line.replace(
                        pattern,
                        // Format the pattern to include the highlight. If -n was passed
                        // then highlight will be ("", "")
                        &format!("{}{}{}", highlight.0, &pattern, highlight.1)
                    )
                );
            } else {
                // Almost the same as the above but without line numbers
                println!(
                    "{}",
                    line.replace(
                        pattern,
                        &format!("{}{}{}", highlight.0, &pattern, highlight.1)
                    )
                );
            }
        }
        line_number += 1;
    }
    // Main can't return for some reason, it would be nice to return the matching
    Ok(())
}
