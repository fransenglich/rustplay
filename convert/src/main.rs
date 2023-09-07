use std::env;

struct Celcius {
    pub degrees: f64,
}

impl Celcius {
    fn new(d: f64) -> Celcius {
        Celcius { degrees: d }
    }
}

// 1C == 33.8F
impl From<Fahrenheit> for Celcius {
    fn from(f: Fahrenheit) -> Celcius {
        Celcius::new(f.degrees * 33.8)
    }
}

impl From<usize> for Celcius {
    fn from(f: usize) -> Celcius {
        Celcius::new(f as f64)
    }
}

struct Fahrenheit {
    pub degrees: f64,
}

impl Fahrenheit {
    fn new(d: f64) -> Fahrenheit {
        Fahrenheit { degrees: d }
    }
}

impl From<Celcius> for Fahrenheit {
    fn from(c: Celcius) -> Fahrenheit {
        Fahrenheit::new(c.degrees * 1.8)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let from = &args[1];
    let to = &args[2];

    let degrees_F: f64 = from.parse::<f64>().unwrap();
    let f: Fahrenheit = Fahrenheit::new(degrees_F);

    let from_u: Celcius = Celcius::From(3);
    let c: Celcius = Celcius::From(f);

    println!("F: {}", c.degrees);
    println!("Hello, world!");
}

/*
#[cfg(test)]
mod tests {

    #[test]
    fn first_test() {
        assert_eq!(2, 2);
    }
}

*/
