use std::io::{self, Error, IsTerminal, Read};
use std::{env, process};
mod colors;

struct Config {
    search_string: String,
    from_file: bool,
}

fn main(){

    // get command line arguments
    let args = env::args().collect::<Vec<String>>();
    // 
    let search_term = match args.get(1) {
        Some(arg) => arg,
        None => {
            eprintln!("Error: Missing search term. Please provide a search term as the first argument.");
            process::exit(1);
        }
    };
    let config = Config {
        search_string: search_term.to_string(),
        from_file: false,
    };
    if let Err(e) = rgrep(config){
        eprintln!("Error: {}", e);
    }
    
}

fn rgrep(config: Config) -> Result<(), Error> {
    let data = get_piped_data()?;

    // enumerate through lines 
    for (i, line) in data.lines().enumerate() {
        // check if line contains search string
        if line.contains(&config.search_string) {
            // if so, print line index
            print!("{}: ", i+1);
            let mut prev = 0;
            // print everything until first match in normal color, then print match in color and reverse color to normal
            for (index, string) in line.match_indices(&config.search_string) {
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
        return Err(Error::new(io::ErrorKind::InvalidInput, "No piped data found"));
    } else {
        let _ = handle.read_to_string(&mut buffer);
        return Ok(buffer);
    }
}
