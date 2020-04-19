// TODO: write tests

pub struct Config {
    bin_name: String,
    args: Vec<String>,
    valid_data: bool,
}

impl Config {
    pub fn new(input_args: &[String]) -> Result<Config, &'static str> {
        if input_args.len() < 2 {
            return Err("Not enough arguments");
        }

        let bin_name = input_args[0].clone();
        let args = input_args[1..].to_vec();
        let mut valid_data = true;

        if args.contains(&String::from("h")) {
            show_help();
            valid_data = false;

        } else if args.contains(&String::from("v")) {
            show_about();
            valid_data = false;
        }

        Ok(Config {bin_name: bin_name, args: args, valid_data: valid_data} )
    }

    pub fn is_valid(&self) -> bool {
        self.valid_data
    }

    pub fn user_data(&self) -> String {
        self.args[0].clone()
    }

    pub fn binary(&self) -> String {
        self.bin_name.clone()
    }
}

fn show_help() {
    println!("\nUsage: [temperature][unit]");
    println!("c: unit is celsius");
    println!("f: unit is farenheit");
    println!("k: unit is kelvin");
}

fn show_about() {
    
    print!(" 
  _                            
 | |                           
 | |_ ___ _ __ ___  ___   ___ 
 | __/ _ \\ '_ ` _ \\| '_\\ /  _|
 | ||  __/ | | | | | |_)|  (_ 
  \\__\\___|_| |_| |_| .__/\\___|
                   | |         
                   |_|\n\n");

    println!("tempc v0.2 by Nick.");

}