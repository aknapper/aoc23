fn main() {
    // get input data
    let input = include_str!("./input.txt");
    // pass input data to the executing function
    let output= part1(input);
    // get puzzle output
    dbg!(output);
}

// get list of integers from a string
fn str_to_unsigned_vec(input: &str) -> Vec<char>{
    let mut int_list: Vec<char> = vec![];
    for (_idx, character) in input.char_indices() {
        if character.is_numeric() {
            // println!("{}", character);
            int_list.push(character);
        }
    }
    return int_list
}

fn part1(input: &str) -> String {
    // iterate over lines filtering out integers, and joining the first and last elements
    let mut calibration_values: Vec<f64> = vec![];
    for line in input.split("\n") {
        // keep only integers
        let cleaned_line = str_to_unsigned_vec(line);

        // Join the first and last characters into a String
        let result: String = match (cleaned_line.first(), cleaned_line.last()) {
            (Some(first), Some(last)) => format!("{}{}", first, last),
            _ => String::from("Vector is empty or has only one element."),
        };

        // Attempt to convert the string to an f64
        match result.parse::<f64>() {
            Ok(calib_value) => {
                calibration_values.push(calib_value);
                println!("{}", calib_value);
            }
            Err(err) => {
                println!("Error parsing float: {}", err);
            }
        }
    }
    
    // sum up "calibration values"
    let sum: f64 = calibration_values.iter().sum();
    return sum.to_string();
}

// test to check algo against example data and answer
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_data_check() {
        let result = part1(include_str!("./ex-input1.txt"));
        assert_eq!(result, "142".to_string());
    }
}
