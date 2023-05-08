use clap::{App, Arg};

fn main() { 
    let matches = App::new("echor") // name of new app is "echor"
        .version("0.1.0")
        .author("Marc Gris - gris.marc@gmail.com")
        .about("A Rust equivalent to bash echo command")
        
        // positional arguments to parse 
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text to be printed out by echor")
                .required(true)
                .min_values(1),
        )

        // define optional arg 'n'  
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )


        .get_matches(); //  parse the arguments
        
    // here we can use .unwrap() because we are sure 
    // that it won't be 'none' (the argument is required...)
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    
    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending);
}
