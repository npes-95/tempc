//TODO: implement display trait

const ABSOLUTE_ZERO_K: f32 = 0.;
const ABSOLUTE_ZERO_C: f32 = ABSOLUTE_ZERO_K - 273.15;
const ABSOLUTE_ZERO_F: f32 = ABSOLUTE_ZERO_C * 9./5. + 32.;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Unit {
    Celcius,
    Fahrenheit,
    Kelvin,
}

#[derive(Debug)]
pub struct Temperature {
    t: f32,
    unit: Unit,
}

impl Temperature {
    pub fn new(input: String) -> Result<Temperature, &'static str> {

        let mut input = String::from(input.trim());

        let unit = input.pop().unwrap();
        let t = input;

        let unit = match unit {
            'c' | 'C' => Unit::Celcius,
            'f' | 'F' => Unit::Fahrenheit,
            'k' | 'K' => Unit::Kelvin,
            _ => return Err("Could not parse unit."),
        }; 

        let t: f32 = match t.parse() {
            Ok(n) => n,
            Err(_) => return Err("Could not parse temperature."),
        };

        if Temperature::above_abs_zero(t, &unit) {
            Ok(Temperature {t, unit})
        } else {
            Err("Temperature below absolute zero.")
        }        
    }

    pub fn to_celcius(&self) -> Temperature {
        let tc = match self.unit {
            Unit::Fahrenheit => (self.t - 32.) * 5./9.,
            Unit::Kelvin => self.t - 273.15,
            Unit::Celcius => self.t
        };

        Temperature {t: tc, unit: Unit::Celcius}
    }
    
    pub fn to_kelvin(&self) -> Temperature {

        let tk = match self.unit {
            Unit::Fahrenheit => ((self.t - 32.) * 5./9.) + 273.15,
            Unit::Celcius => self.t + 273.15,
            Unit::Kelvin => self.t
        };

        Temperature {t: tk, unit: Unit::Kelvin}
    }

    pub fn to_fahrenheit(&self) -> Temperature {

        let tf = match self.unit {
            Unit::Kelvin => (self.t - 273.15) * 9./5. + 32.,
            Unit::Celcius => self.t * 9./5. + 32.,
            Unit::Fahrenheit => self.t
        };

        Temperature {t: tf, unit: Unit::Fahrenheit}
    }

    pub fn to_string(&self) -> String {
        let unit = match self.unit {
            Unit::Fahrenheit => 'f',
            Unit::Celcius => 'c',
            Unit::Kelvin => 'k',
        };

        format!("{}{}",self.t.to_string(), unit.to_string())
    }

    pub fn unit(&self) -> Unit {
        self.unit
    }
}

impl Temperature {
    fn above_abs_zero(t: f32, unit: &Unit) -> bool {
        match &unit {
            Unit::Fahrenheit => t >= ABSOLUTE_ZERO_F,
            Unit::Celcius => t >= ABSOLUTE_ZERO_F,
            Unit::Kelvin => t >= ABSOLUTE_ZERO_K,
        }
    }
}

impl PartialEq for Temperature {
    fn eq(&self, other: &Self) -> bool {
        self.unit == other.unit && self.t == other.t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_new() {
        // invalid unit
        assert!(Temperature::new(String::from("32z")).is_err());
        
        // invalid input
        assert!(Temperature::new(String::from("wefigyudgqdt%$34571234723gqwe87")).is_err());

        // out of bounds (below absolute zero)
        assert!(Temperature::new(String::from("-10000c")).is_err());
        assert!(Temperature::new(String::from("-10000f")).is_err());
        assert!(Temperature::new(String::from("-10000k")).is_err());

        // valid input
        assert!(!Temperature::new(String::from("30c")).is_err());
        assert!(!Temperature::new(String::from("30f")).is_err());
        assert!(!Temperature::new(String::from("30k")).is_err());
    }

    #[test]
    fn test_convert() -> Result<(), Box<dyn Error>> {
        let abs_zero_k = Temperature::new(String::from(ABSOLUTE_ZERO_K.to_string() + "k"))?;
        let abs_zero_f = Temperature::new(String::from(ABSOLUTE_ZERO_F.to_string() + "f"))?;
        let abs_zero_c = Temperature::new(String::from(ABSOLUTE_ZERO_C.to_string() + "c"))?;

        assert_eq!(abs_zero_f, abs_zero_c.to_fahrenheit());
        assert_eq!(abs_zero_f, abs_zero_k.to_fahrenheit());

        assert_eq!(abs_zero_c, abs_zero_f.to_celcius());
        assert_eq!(abs_zero_c, abs_zero_k.to_celcius());

        assert_eq!(abs_zero_k, abs_zero_c.to_kelvin());
        assert_eq!(abs_zero_k, abs_zero_f.to_kelvin());        

        Ok(())
    }

    #[test]
    fn test_to_string() -> Result<(), Box<dyn Error>> {
        let abs_zero_k = Temperature::new(String::from(ABSOLUTE_ZERO_K.to_string() + "k"))?;

        assert_eq!(abs_zero_k.to_string(), String::from(ABSOLUTE_ZERO_K.to_string() + "k"));

        Ok(())
    }

    #[test]
    fn test_unit() -> Result<(), Box<dyn Error>> {
        let abs_zero_k = Temperature::new(String::from(ABSOLUTE_ZERO_K.to_string() + "k"))?;
        let abs_zero_f = Temperature::new(String::from(ABSOLUTE_ZERO_F.to_string() + "f"))?;
        let abs_zero_c = Temperature::new(String::from(ABSOLUTE_ZERO_C.to_string() + "c"))?;

        assert_eq!(abs_zero_k.unit(), Unit::Kelvin);
        assert_eq!(abs_zero_f.unit(), Unit::Fahrenheit);
        assert_eq!(abs_zero_c.unit(), Unit::Celcius);        

        Ok(())

    }
}