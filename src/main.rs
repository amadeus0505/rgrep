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
    
    // find all indices of 
    let indices: Vec<_> = data.match_indices(&config.search_string).collect();

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
        return Err(Error::new(io::ErrorKind::InvalidInput, "No piped data found"));
    } else {
        let _ = handle.read_to_string(&mut buffer);
        return Ok(buffer);
    }
}
