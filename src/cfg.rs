pub struct Config {
    bin_name: String,
    args: Vec<String>,
    input_data_ok: bool,
}

impl Config {
    pub fn new(input_args: &[String]) -> Result<Config, &'static str> {
        if input_args.len() < 2 {
            return Err("Not enough arguments");
        }

        let bin_name = input_args[0].clone();
        let args = input_args[1..].to_vec();
        let mut input_data_ok = true;

        if args.contains(&String::from("h")) {
            show_help();
            input_data_ok = false;

        } else if args.contains(&String::from("v")) {
            show_about();
            input_data_ok = false;
        }

        Ok(Config {bin_name: bin_name, args: args, input_data_ok: input_data_ok} )
    }

    pub fn input_data_ok(&self) -> bool {
        self.input_data_ok
    }

    pub fn input_data(&self) -> String {
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

mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_new() {
        let mut args: Vec<String> = Vec::new();

        args.push("too few args".to_string());
        assert!(Config::new(&args).is_err());

        args.push("correct number of args is >= 2".to_string());
        assert!(!Config::new(&args).is_err());
    }

    #[test]
    fn test_helper_opts() -> Result<(), Box<dyn Error>> {
        let mut args: Vec<String> = Vec::new();
        args.push("path/to/bin".to_string());
        args.push("h".to_string());

        let config = Config::new(&args)?;
        assert!(!config.input_data_ok());

        let mut args: Vec<String> = Vec::new();
        args.push("path/to/bin".to_string());
        args.push("v".to_string());

        let config = Config::new(&args)?;
        assert!(!config.input_data_ok());

        Ok(())
    }

    #[test]
    fn test_data() -> Result<(), Box<dyn Error>> {
        let mut args: Vec<String> = Vec::new();
        args.push("path/to/bin".to_string());
        args.push("some_arg".to_string());

        let config = Config::new(&args)?;
        assert!(config.input_data_ok());
        assert!(args[1] == config.input_data());

        Ok(())
    }
}