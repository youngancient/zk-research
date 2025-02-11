use ark_ff::{BigInteger, PrimeField};
use std::collections::HashSet;

// update this to use binary instead of decimal
#[derive(Clone)]

pub struct EvaluationForm<F: PrimeField> {
    pub number_of_variables: u32,
    pub eval_form: Vec<F>,
    pub boolean_hypercube: Vec<u32>,
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
            boolean_hypercube: (0..n).collect(),
        }
    }
    // variable position: 1st , 2nd , 3rd etc
    // in a f(a,b,c) -> a is 1, b -> 2 , c -> 3
    pub fn partial_evaluate(&mut self, variable_position: u32, value: F) {
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
        let target = 2u32.pow(self.number_of_variables - variable_position);
        let pairings = find_pairs_with_xor(&self.boolean_hypercube, target);
        let mut new_vec: Vec<F> = Vec::new();

        for pair in pairings {
            let index_one = pair.0;
            let index_two = pair.1;
            let v = interpolate_and_evaluate(
                (
                    self.eval_form[index_one as usize],
                    self.eval_form[index_two as usize],
                ),
                value,
            );
            new_vec.push(v);
        }
        self.eval_form = new_vec;
        self.boolean_hypercube = (0..self.eval_form.len() as u32).collect();
    }

    // the order of the variables is important -> [a, b, c, d,...] for f(a,b,c,d,...)
    pub fn evaluate(&mut self, variables: &Vec<F>) -> F {
        if variables.len() != self.number_of_variables as usize {
            panic!("Invalid number of points")
        }
        // dbg!(&variables, &self.eval_form);
        for (i, var) in variables.iter().enumerate() {
            self.partial_evaluate((i + 1) as u32, *var);
        }
        if self.eval_form.is_empty() {
            panic!("polynomial is empty!");
        }
        self.eval_form[0]
    }

    // pub fn evaluate_two_vars()

    // converts polynimial from F -> list of bytes
    // use case:: fiat-shamir implementation
    pub fn to_bytes(polynomial: &Vec<F>) -> Vec<u8> {
        polynomial
            .iter()
            .flat_map(|coeff| coeff.into_bigint().to_bytes_be())
            .collect()
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

use rand::thread_rng;
pub fn gen_random_vars<F: PrimeField>(n: u32) -> Vec<F> {
    let mut rng = thread_rng();
    let mut vars_list: Vec<F> = Vec::new();
    for _ in 0..n {
        let y: F = F::rand(&mut rng);
        vars_list.push(y);
    }
    vars_list
}
pub fn gen_based_on_two<F: PrimeField>(n: u32) -> Vec<F> {
    let to_pow_two = 2u32.pow(n);
    gen_random_vars(to_pow_two)
}

// product poly
// instead of getting the prod of 2 polynomials : 3ab x 2ab
// we represent the 2 polynomials in the form: 3ab x 2ab
pub struct ProdPoly<F: PrimeField> {
    pub polynomials: Vec<EvaluationForm<F>>,
    pub no_of_vars:u32,
}

impl<F: PrimeField> ProdPoly<F> {
    pub fn new(polynomials: Vec<EvaluationForm<F>>) -> Self {
        if polynomials.is_empty() {
            panic!("poly cannot be empty!");
        }

        let no_of_vars = polynomials[0].number_of_variables;
        for poly in &polynomials {
            if poly.number_of_variables != no_of_vars {
                panic!("Polynomials must be of the same number of variables");
            }
        }
        ProdPoly { polynomials, no_of_vars }
    }

    pub fn partial_evaluate(&mut self, variable_position: u32, value: F) {
        for poly in &mut self.polynomials {
            poly.partial_evaluate(variable_position, value);
        }
    }

    pub fn evaluate(&mut self, variables: &Vec<F>) -> F {
        let mut product = F::one();
        if (variables.len() as u32) != self.no_of_vars{
            panic!("Invalid variable length!");
        }
        for poly in &mut self.polynomials {
            product *= poly.evaluate(&variables);
        }
        product
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.polynomials
            .iter()
            .flat_map(|polynomial| EvaluationForm::to_bytes(&polynomial.eval_form))
            .collect()
    }
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
        assert_eq!(eval_form.boolean_hypercube, vec![0, 1, 2, 3]);
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
    fn test_finding_pairs() {
        let num = 4;
        let boolean_hypercube_of_2_vars = (0..num).collect::<Vec<u32>>();
        // Since we have just 2 bits representing two vars, -> a , b
        // 1 -> our target is b
        // 2 -> our target is a
        let a_target = 2;
        let b_target = 1;
        let pairs_for_a = find_pairs_with_xor(&boolean_hypercube_of_2_vars, a_target);
        assert_eq!(pairs_for_a, vec![(0, 2), (1, 3)]);

        let pairs_for_b = find_pairs_with_xor(&boolean_hypercube_of_2_vars, b_target);
        assert_eq!(pairs_for_b, vec![(0, 1), (2, 3)]);
    }

    #[test]
    fn test_finding_pairs_for_3vars() {
        let num = 8;
        let boolean_hypercube_of_2_vars = (0..num).collect::<Vec<u32>>();
        // Since we have just 2 bits representing two vars, -> a , b
        // 01 -> our target is b
        // 10 -> our target is a
        let a_target = 4;
        let b_target = 2;
        let c_target = 1;

        let pairs_for_a = find_pairs_with_xor(&boolean_hypercube_of_2_vars, a_target);
        assert_eq!(pairs_for_a, vec![(0, 4), (1, 5), (2, 6), (3, 7)]);

        let pairs_for_b = find_pairs_with_xor(&boolean_hypercube_of_2_vars, b_target);
        assert_eq!(pairs_for_b, vec![(0, 2), (1, 3), (4, 6), (5, 7)]);

        let pairs_for_c = find_pairs_with_xor(&boolean_hypercube_of_2_vars, c_target);
        assert_eq!(pairs_for_c, vec![(0, 1), (2, 3), (4, 5), (6, 7)]);
    }

    #[test]
    fn test_interpolate_and_evaluate() {
        let y_values = (Fq::from(1), Fq::from(2));
        let r = Fq::from(3);
        assert_eq!(interpolate_and_evaluate(y_values, r), Fq::from(4));
    }

    #[test]
    fn test_partial_evaluate_1vars() {
        let mut poly = EvaluationForm::new(vec![Fq::from(4), Fq::from(7)]);
        poly.partial_evaluate(1, Fq::from(3));
        assert_eq!(poly.eval_form, vec![Fq::from(13)]);
        assert_eq!(poly.number_of_variables, 1);
    }

    #[test]
    fn test_partial_evaluate_2vars() {
        let mut poly =
            EvaluationForm::new(vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)]);
        poly.partial_evaluate(1, Fq::from(2));
        assert_eq!(poly.eval_form, vec![Fq::from(4), Fq::from(7)]);
        assert_eq!(poly.number_of_variables, 2);
    }

    #[test]
    fn test_partial_evaluate_3vars() {
        let mut poly = EvaluationForm::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(3),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(5),
        ]);
        poly.partial_evaluate(3, Fq::from(3));
        assert_eq!(
            poly.eval_form,
            vec![Fq::from(0), Fq::from(9), Fq::from(0), Fq::from(11)]
        );
        assert_eq!(poly.number_of_variables, 3);
    }

    #[test]
    fn test_evaluate_for_2vars() {
        let mut eval_form =
            EvaluationForm::new(vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)]);
        assert_eq!(
            eval_form.evaluate(&vec![Fq::from(2), Fq::from(3)]),
            Fq::from(13)
        );
    }

    #[test]
    fn test_to_bytes_for_2vars() {
        let polynomial =
            EvaluationForm::new(vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)]);
        // Compute the expected byte representation manually
        let expected_bytes: Vec<u8> = polynomial
            .eval_form
            .iter()
            .flat_map(|coeff| coeff.into_bigint().to_bytes_be())
            .collect();

        // Get actual bytes using the to_bytes function
        let actual_bytes = EvaluationForm::to_bytes(&polynomial.eval_form);

        // Assert that both byte representations match
        assert_eq!(actual_bytes, expected_bytes);
    }
    #[test]
    fn test_evaluate_for_3vars() {
        let mut eval_form = EvaluationForm::new(vec![
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
            eval_form.evaluate(&vec![Fq::from(4), Fq::from(2), Fq::from(3)]),
            Fq::from(34)
        );
    }

    fn get_prod_poly() -> ProdPoly<Fq>{
        let polynomials = vec![
            EvaluationForm::new(vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]),
            EvaluationForm::new(vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]),
        ];
        let prod_poly = ProdPoly::new(polynomials);
        prod_poly
    }
    #[test]
    fn test_prod_poly_creation() {
        let prod_poly = get_prod_poly();
        assert_eq!(prod_poly.no_of_vars,2);
    }

    #[test]
    fn test_prod_poly_eval(){
        let mut prod_poly = get_prod_poly();
        let eval = prod_poly.evaluate(&vec![Fq::from(1), Fq::from(2)]);
        assert_eq!(eval,Fq::from(24));
    }
}
