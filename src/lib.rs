use std::error::Error;

pub mod temperature;
pub mod cfg;

pub fn run(config: cfg::Config) -> Result<(), Box<dyn Error>> {

    if config.is_valid() {
        let input_temp = temperature::Temperature::new(config.user_data())?;

        let (output_temp_1, output_temp_2) = match input_temp.unit() {
            temperature::Unit::Kelvin => (input_temp.to_fahrenheit(), input_temp.to_celcius()),
            temperature::Unit::Celcius => (input_temp.to_kelvin(), input_temp.to_fahrenheit()),
            temperature::Unit::Fahrenheit => (input_temp.to_kelvin(), input_temp.to_celcius()),
        };

        println!("{} {}", output_temp_1, output_temp_2);
    }

    Ok(())
}
