// Crate imports
extern crate walkdir;
extern crate regex;
extern crate structopt;
extern crate failure;
extern crate exitfailure;

// Crates
use std::fs::File;
use walkdir::WalkDir;
use std::io::Read;
use regex::RegexBuilder;
use failure::ResultExt;
use exitfailure::ExitFailure;

// Mods & imports
mod parse_options;
use parse_options::options;
use crate::structopt::StructOpt;


fn extracted_files() -> Result<Vec::<(String, String)>, ExitFailure> {
    // Get options from CLI
    let options: options::Options = options::Options::from_args();



    // Create regex
    let key = RegexBuilder::new(&options.get_key())
        .case_insensitive(true)
        .build()
        .with_context(|_| format!("Invalid regex: {:?}", &options.get_key()))
        .unwrap();


    // Create filelist Vector, and WalkDir iterator objects to traverse through directories
    let mut data = Vec::<(String, String)>::new();
    for file in WalkDir::new(options.directory).into_iter()
        .filter_map(|file| file.ok()) { //WalkDir impls Iterator and IntoIter for into_iter() and filter_map()
        let filename = match file.path().is_file() {
            true => file.path().display().to_string(), // Returns filenames
            false => continue, // This is not a file; move on
        };
        println!("{}", filename);

        // use File struct to open file from given filename
        match File::open(&filename) {
            Ok(mut f) => {
                let mut content = String::new(); // set up String to hold file contents
                match f.read_to_string(&mut content) {
                    Ok(_) => println!("Successfully read file {}", &filename),
                    Err(e) => println!("Could not print file: {}", e), // File (likely) has
                    // non-UTF-8 data; will need to implement
                };
                f.read_to_string(&mut content).unwrap(); // Store file contents in String

                // Use regex to search through a slice of content
                // find() takes &str; only way to get this out of a String is to slice it.
                match key.find(&mut content[..]) {
                    Some(_m) => data.push((filename, content)),
                    None => continue,
                }


            },
            Err(e) => {
              println!("Error opening file {}: {}", filename, e);
            },
        }
    }

    Ok(data)
}

fn main() {
    let filelist = extracted_files().unwrap();

    // Check if no files matched the key
    match filelist.is_empty() {
        true => println!("No files matched the key :("),
        false => {
            println!("Found matches for the key");
            for i in filelist.into_iter() {
                println!("{}", i.0); // It's a tuple
            }
        },
    }

}
