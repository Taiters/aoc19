use std::env;
use std::fs::File;
use std::cmp::Ordering;
use std::io::BufReader;
use std::io::prelude::*;

fn calculate_fuel(mass: i32) -> i32 {
    let required: i32 = mass / 3 - 2;

    match required.cmp(&0) {
        Ordering::Greater => required + calculate_fuel(required),
        _ => 0,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("Input file required");

    let file = File::open(input_file).expect("Unable to read input file");
    let file = BufReader::new(file);

    let fuel_required: i32 = file.lines()
        .map(|x| x.unwrap().parse().unwrap())
        .map(|x| calculate_fuel(x))
        .sum();

    println!("{}", fuel_required);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuel_zero() {
        assert_eq!(calculate_fuel(0), 0);
    }

    #[test]
    fn test_calculate_fuel_negative() {
        assert_eq!(calculate_fuel(-123), 0);
    }

    #[test]
    fn test_calculate_fuel_example_from_docs() {
        assert_eq!(calculate_fuel(100756), 50346);
    }
}
