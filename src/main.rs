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
    
    // find all indices of 
    let indices: Vec<_> = data.match_indices(&search_string).collect();

    let mut prev = 0;
    for (index, string) in indices{
        print!("{}", &data[prev..index]);
        colors::cprint(string.to_string(), colors::Color::Green);
        colors::cprint("".to_string(), colors::Color::White);
        prev = index+string.len();
    }
    // print remaining data
    print!("{}", &data[prev..]);
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
