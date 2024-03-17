use std::io::{self, BufRead, Error, IsTerminal, Read, ErrorKind};
use std::env;
use colors::{cprint, Color};
use colored::*;
mod colors;

fn main() -> io::Result<()> {
    
    let args = env::args().collect::<Vec<String>>();
    let search_term = match args.get(1) {
        Some(arg) => arg,
        None => {
            eprintln!("Error: Missing search term. Please provide a search term as the first argument.");
            return Err(Error::new(ErrorKind::InvalidInput, "Missing search term"));
        }
    };
    match rgrep(search_term) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn rgrep(search_string: &str) -> Result<(), Error> {

    let data = get_piped_data()?;

    // enumerate through lines 
    for (i, line) in data.lines().enumerate() {
        // check if line contains search string
        if line.contains(search_string) {
            // if so, print line index
            print!("{}: ", i+1);
            let mut prev = 0;
            // print everything until first match in normal color, then print match in color and reverse color to normal
            for (index, string) in line.match_indices(search_string) {
                print!("{}", &line[prev..index]);
                colors::cprint(string.to_string(), colors::Color::Green);
                // TODO: make function for changing color without printing
                colors::cprint("".to_string(), colors::Color::White);
                prev = index+string.len();
            }
            // print remaining data from this line
            print!("{}", &line[prev..]);
            println!();
        }
    }
    Ok(())

}

fn get_piped_data() -> Result<String, Error> {
    // get piped input
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    if stdin.is_terminal() {
        eprint!("No piped data found");
        return Err(Error::new(io::ErrorKind::InvalidInput, "No piped data found"));
    } else {
        let _ = handle.read_to_string(&mut buffer);
        return Ok(buffer);
    } 
}
