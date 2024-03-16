use std::io::{self, Read};
use std::env;
use color_test::{cprint, Color};
use colored::*;
mod color_test;

fn main() -> io::Result<()> {
    // grep()?;
    rgrep();
    Ok(())
}

fn rgrep() {
    // get search string (user input; cmd-argument)
    let search_string = get_search_string();
    // get piped data string
    let data = get_piped_data();
    
    // find all indices of 
    let indices: Vec<_> = data.match_indices(&search_string).collect();

    let mut prev = 0;
    for (index, string) in indices{
        print!("{}", &data[prev..index]);
        color_test::cprint(string.to_string(), color_test::Color::Green);
        color_test::cprint("".to_string(), color_test::Color::White);
        prev = index+string.len();
    }
    // print remaining data
    print!("{}", &data[prev..]);


}

fn get_search_string() -> String {
    // get command line arguments
    let args: Vec<String> = env::args().collect();
    let search_string = match args.get(1){
        Some(str) => str,
        None => "",
    };
    return search_string.to_owned();
}

fn get_piped_data() -> String {
    // get piped input
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let _ = handle.read_to_string(&mut buffer);
    return buffer;
}

fn grep() -> io::Result<()>{
    let search_string = get_search_string();

    // get piped input
    let mut buffer = get_piped_data();

    let binding = buffer.clone();
    let mut indices: Vec<_> = binding.match_indices(&search_string).collect();



    indices.reverse();
    for (index, _str) in indices {
        buffer.insert_str(index+search_string.len(), "\u{1b}[0m");
        buffer.insert_str(index, "\u{1b}[1;31m");
    }


    println!("{}", buffer);
    println!("{}", "String".blue());
    Ok(())
}
