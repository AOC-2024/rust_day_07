use std::{collections::HashMap, fs::read_to_string};


pub fn get_calibration_results(input_path: &str) -> u128 {
    let equations = extract_equations(input_path);

    return equations.iter()
    .filter(|equation| equation.is_possible())
    .map(|equation| equation.result)
    .fold(0, |acc, value| acc + value);
}

fn extract_equations(input_path: &str) -> Vec<Equation> {
    read_to_string(input_path)
    .unwrap()
    .lines()
    .map(|line| {
        Equation::map_line(line)
    })
    .collect::<Vec<Equation>>()
}

#[derive(PartialEq, Debug)]
struct Equation {
    result: u128,
    numbers: Vec<u128>
}

impl Equation {
    fn map_line(line: &str) -> Equation {
        Equation {
            result: line.chars().take_while(|character| character != &':').collect::<String>().parse().unwrap(),
            numbers: line.chars().skip_while(|character| character != &':').skip(2).collect::<String>().split(" ").map(|number| number.parse().unwrap()).collect::<Vec<u128>>()
        }        
    }

    fn is_possible(&self) -> bool {
        self.count_solutions() > 0
    }

    fn count_solutions(&self) -> usize {

        if self.numbers.len() == 0 {
            return 0;
        }


        if self.numbers.len() == 1 {
            if self.result.eq(self.numbers.get(0).unwrap()) {
                return 1;
            } else {
                return 0;
            }
        }

        let mut possibilities: HashMap<usize, Vec<u128>> = HashMap::new();
            possibilities.insert(0, vec![*self.numbers.get(0).unwrap()]);


        for i in 1..self.numbers.len() {
            let previous_values = possibilities.get(&(i - 1)).unwrap();
            let mut next_values = Vec::new();
            for existing_value in previous_values {
                next_values.push(existing_value + self.numbers.get(i).unwrap());
                next_values.push(existing_value * self.numbers.get(i).unwrap());
            }
            possibilities.insert(i, next_values);
        }
        
        let binding = possibilities.clone();
        let last_results = binding.get(&(self.numbers.len() - 1)).unwrap();

        return last_results.iter().filter(|el| **el == self.result).count();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_count_solution_2_when_can_add_or_multiply_multiple_numbers() {
        let equation = Equation {
            result: 3267,
            numbers: vec![81, 40, 27]
        };
        assert_eq!(equation.count_solutions(), 2)
    }

    #[test]
    fn should_count_solution_1_when_can_add_multiple_numbers() {
        let equation = Equation {
            result: 2,
            numbers: vec![0, 1, 0, 1]
        };
        assert_eq!(equation.count_solutions(), 1)
    }

    #[test]
    fn should_count_solution_2_when_can_multiply_or_add_two_numbers() {
        let equation = Equation {
            result: 0,
            numbers: vec![0, 0]
        };
        assert_eq!(equation.count_solutions(), 2)
    }

    #[test]
    fn should_count_solution_1_when_can_multiply_two_numbers() {
        let equation = Equation {
            result: 2,
            numbers: vec![2, 1]
        };
        assert_eq!(equation.count_solutions(), 1)
    }

    #[test]
    fn should_count_solution_1_when_can_add_two_numbers() {
        let equation = Equation {
            result: 2,
            numbers: vec![1, 1]
        };
        assert_eq!(equation.count_solutions(), 1)
    }

    #[test]
    fn should_count_solution_1_when_one_number_equal_to_result() {
        let equation = Equation {
            result: 2,
            numbers: vec![2]
        };
        assert_eq!(equation.count_solutions(), 1)
    }

    #[test]
    fn should_count_solution_0_when_empty_numbers() {
        let equation = Equation {
            result: 190,
            numbers: Vec::new()
        };
        assert_eq!(equation.count_solutions(), 0)
    }

    #[test]
    fn should_count_solution_0_when_empty_result() {
        let equation = Equation {
            result: 0,
            numbers: vec![10, 19]
        };
        assert_eq!(equation.count_solutions(), 0)
    }

    #[test]
    fn should_extract_equations() {
        assert_eq!(extract_equations("tests/resources/light_puzzle.txt"), vec![
            Equation {
                result: 190,
                numbers: vec![10, 19]
            },
            Equation {
                result: 3267,
                numbers: vec![81, 40, 27]
            }
        ])
    }
}