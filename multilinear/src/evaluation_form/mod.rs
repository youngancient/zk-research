use ark_ff::PrimeField;
use std::collections::HashSet;

#[derive(Clone)]
pub struct EvaluationForm<F: PrimeField> {
    pub number_of_variables: u32,
    pub eval_form: Vec<F>,
    pub polynomial_hypercube: Vec<u32>,
}

impl<F: PrimeField> EvaluationForm<F> {
    pub fn new(eval_form: Vec<F>) -> Self {
        // if the length of the list is n, then
        // there must exist a m, such that 2^m = n
        let n = eval_form.len() as u32;
        let no_of_vars = get_power_of_two(n);
        Self {
            eval_form,
            number_of_variables: no_of_vars,
            polynomial_hypercube: generate_binary_range(n),
        }
    }
    // variable position: 1st , 2nd , 3rd etc
    // in a f(a,b,c) -> a is 1, b -> 2 , c -> 3
    pub fn partial_evaluate(&self, variable_position: u32, value: F) -> Vec<F> {
        // Since we have just 2 bits representing two vars, -> a , b
        // 01 -> our target is b
        // 10 -> our target is a
        if variable_position == 0 || variable_position > self.number_of_variables {
            panic!("Number of variables Exceeded!")
        }
        // for F(a,b,c)
        // 1 -> 100 -> a
        // 2 -> 010 -> b
        // 3 -> 001 -> c

        // 2 ^ (number_of_vars - variable_position)
        let target = to_binary(2u32.pow(self.number_of_variables - variable_position));

        let pairings = find_pairs_with_xor(&self.polynomial_hypercube, target);

        let mut new_vec: Vec<F> = Vec::new();

        for pair in pairings {
            let index_one = binary_to_decimal(pair.0);
            let index_two = binary_to_decimal(pair.1);
            let v = interpolate_and_evaluate(
                (
                    self.eval_form[index_one as usize],
                    self.eval_form[index_two as usize],
                ),
                value,
            );
            new_vec.push(v);
        }
        new_vec
    }

    // the order of the variables is important -> [a, b, c, d,...] for f(a,b,c,d,...)
    pub fn evaluate(&self, variables: Vec<F>) -> F {
        if variables.len() != self.number_of_variables as usize {
            panic!("Invalid number of points")
        }
        // let mut result: EvaluationForm<F> = (*self).clone();

        // for i in 0..self.number_of_variables {
        //     result.eval_form = result.partial_evaluate(i + 1, variables[i as usize]);
        // }
        // println!("Result: {:?}", result.eval_form);
        todo!();
    }
}

// helper functions

// this function receives a value and tells the highest exponent of 2 that returns the value
// i.e given Y, the function returns x, where 2 ^ x = Y
pub fn get_power_of_two(value: u32) -> u32 {
    if value == 0 || (value & (value - 1)) != 0 {
        panic!("{} is not a power of 2; Invalid length", value);
    }
    value.trailing_zeros()
}

// this function converts decimal to binary
pub fn to_binary(mut num: u32) -> u32 {
    let mut result: u32 = 0;
    let mut pow: u32 = 1;

    while num > 0 {
        result += (num % 2) * pow;
        num /= 2;
        pow *= 10;
    }

    result
}

// this function converts binary to decimal
pub fn binary_to_decimal(mut binary: u32) -> u32 {
    let mut decimal: u32 = 0;
    let mut pow: u32 = 1;

    while binary > 0 {
        decimal += (binary % 10) * pow;
        binary /= 10;
        pow *= 2;
    }

    decimal
}

// this function takes in a number and generates binary values up to the num - 1
pub fn generate_binary_range(n: u32) -> Vec<u32> {
    (0..n).map(|i| to_binary(i)).collect()
}

// this function uses XOR gate to pair the entities of the boolean hypercube
pub fn find_pairs_with_xor(nums: &[u32], target: u32) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    let mut seen = HashSet::new();

    for &num in nums {
        let complement = num ^ target;
        if seen.contains(&complement) {
            result.push((complement, num));
        }
        seen.insert(num);
    }

    result
}

// suppose we want to evaluate y1 -> y2   at r
pub fn interpolate_and_evaluate<F: PrimeField>(y_values: (F, F), r: F) -> F {
    y_values.0 + r * (y_values.1 - y_values.0)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_create_rep() {
        let eval_form =
            EvaluationForm::new(vec![Fq::from(1), Fq::from(2), Fq::from(3), Fq::from(4)]);
        assert_eq!(eval_form.number_of_variables, 2);
        assert_eq!(eval_form.polynomial_hypercube, vec![0, 1, 10, 11]);
    }

    #[test]
    #[should_panic]
    fn test_create_rep_fail() {
        EvaluationForm::new(vec![
            Fq::from(1),
            Fq::from(2),
            Fq::from(3),
            Fq::from(4),
            Fq::from(15),
        ]);
    }

    #[test]
    fn test_binary_conversion() {
        let num = 4;
        assert_eq!(to_binary(num), 100);
    }

    #[test]
    fn test_binary_range_conversion() {
        let num = 4;
        assert_eq!(generate_binary_range(num), vec![0, 1, 10, 11]);
    }

    #[test]
    fn test_binary_to_decimal() {
        let num = 4;
        let binary_val = to_binary(num);
        assert_eq!(num, binary_to_decimal(binary_val));
    }

    #[test]
    fn test_finding_pairs() {
        let num = 4;
        let boolean_hypercube_of_2_vars = generate_binary_range(num);
        // Since we have just 2 bits representing two vars, -> a , b
        // 01 -> our target is b
        // 10 -> our target is a
        let a_target = 10;
        let b_target = 01;
        let pairs_for_a = find_pairs_with_xor(&boolean_hypercube_of_2_vars, a_target);
        assert_eq!(pairs_for_a, vec![(0, 10), (1, 11)]);

        let pairs_for_b = find_pairs_with_xor(&boolean_hypercube_of_2_vars, b_target);
        assert_eq!(pairs_for_b, vec![(0, 1), (10, 11)]);
    }

    #[test]
    fn test_finding_pairs_for_3vars() {
        let num = 8;
        let boolean_hypercube_of_2_vars = generate_binary_range(num);
        // Since we have just 2 bits representing two vars, -> a , b
        // 01 -> our target is b
        // 10 -> our target is a
        let a_target = 100;
        let b_target = 010;
        let c_target = 001;

        let pairs_for_a = find_pairs_with_xor(&boolean_hypercube_of_2_vars, a_target);
        assert_eq!(pairs_for_a, vec![(0, 100), (1, 101), (10, 110), (11, 111)]);

        let pairs_for_b = find_pairs_with_xor(&boolean_hypercube_of_2_vars, b_target);
        assert_eq!(pairs_for_b, vec![(0, 10), (1, 11), (100, 110), (101, 111)]);

        let pairs_for_c = find_pairs_with_xor(&boolean_hypercube_of_2_vars, c_target);
        assert_eq!(pairs_for_c, vec![(0, 1), (10, 11), (100, 101), (110, 111)]);
    }

    #[test]
    fn test_interpolate_and_evaluate() {
        let y_values = (Fq::from(1), Fq::from(2));
        let r = Fq::from(3);
        assert_eq!(interpolate_and_evaluate(y_values, r), Fq::from(4));
    }

    #[test]
    fn test_partial_evaluate_1vars() {
        let eval_form = EvaluationForm::new(vec![Fq::from(4), Fq::from(7)]);
        let result = eval_form.partial_evaluate(1, Fq::from(3));
        assert_eq!(result, vec![Fq::from(13)]);
    }

    #[test]
    fn test_partial_evaluate_2vars() {
        let eval_form =
            EvaluationForm::new(vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)]);
        let result = eval_form.partial_evaluate(1, Fq::from(2));
        assert_eq!(result, vec![Fq::from(4), Fq::from(7)]);
    }

    #[test]
    fn test_partial_evaluate_3vars() {
        let eval_form = EvaluationForm::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(3),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(5),
        ]);
        let result = eval_form.partial_evaluate(3, Fq::from(3));
        assert_eq!(
            result,
            vec![Fq::from(0), Fq::from(9), Fq::from(0), Fq::from(11)]
        );
    }

    #[test]
    fn test_evaluate() {
        let eval_form = EvaluationForm::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(3),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(5),
        ]);
        assert_eq!(
            eval_form.evaluate(vec![Fq::from(4), Fq::from(2), Fq::from(3)]),
            Fq::from(34)
        );
    }
}
