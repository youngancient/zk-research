use ark_ff::{BigInteger, PrimeField};
use fiat_shamir::fiat_shamir::Transcript;
use multilinear::evaluation_form::{interpolate_and_evaluate, EvaluationForm};
use sha3::{Digest, Keccak256};

#[derive(Debug)]
pub struct Proof<F: PrimeField> {
    sum: F,
    polynomials: Vec<Vec<F>>,
}

pub fn get_sum_at_0_and_1<F: PrimeField>(polynomial: &Vec<F>) -> F {
    let sum: F = polynomial.iter().sum();
    sum
}

fn split_by_var<F: PrimeField>(arr: &[F], var: usize) -> (Vec<F>, Vec<F>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let max_bits = arr.len().ilog2() as usize;

    if var > max_bits {
        panic!(
            "var ({}) is larger than the number of bits required ({})",
            var, max_bits
        );
    }

    if arr.len() == 1 {
        return (arr.to_vec(), Vec::new()); // Edge case: single-element array
    }

    for (i, &val) in arr.iter().enumerate() {
        if (i >> (max_bits - var)) & 1 == 0 {
            left.push(val);
        } else {
            right.push(val);
        }
    }

    (left, right)
}

// takes a vector array
// add the left half, the right half
// return the sume in a vec
pub fn evaluate_at_two_vars<F: PrimeField>(poly_vec: &Vec<F>, var: usize) -> Vec<F> {
    let (left, right) = split_by_var(&poly_vec, var);
    vec![left.iter().sum::<F>(), right.iter().sum::<F>()]
}

pub fn prove<F: PrimeField>(polynomial: &mut EvaluationForm<F>, claim_sum: F) -> Proof<F> {
    let mut transcript: Transcript<F, Keccak256> = Transcript::init(Keccak256::new());
    transcript.append(&EvaluationForm::to_bytes(&polynomial.eval_form));

    transcript.append(
        claim_sum
            .into_bigint()
            .to_bytes_be()
            .as_slice(),
    );

    let mut proof: Proof<F> = Proof {
        sum: F::zero(),
        polynomials: Vec::new(),
    };

    proof.sum = claim_sum;
    for i in 0..polynomial.number_of_variables - 1 {
        // println!("i -> {}",i);
        let univariate_poly_eval_form =
            evaluate_at_two_vars(&polynomial.eval_form, (i + 1) as usize);
        println!("prover poly -> {:?}",univariate_poly_eval_form);
        transcript.append(&EvaluationForm::to_bytes(&univariate_poly_eval_form));
        // println!("poly -> {:?}", univariate_poly_eval_form);
        proof.polynomials.push(univariate_poly_eval_form);

        let random_challenge = transcript.hash();
        println!("prover random chall-> {:?}", random_challenge);
        polynomial.partial_evaluate(i + 1, random_challenge);
    }
    // println!("proof-> {:?}",proof);
    proof
}

pub fn verify<F: PrimeField>(proof: Proof<F>, polynomial: &mut EvaluationForm<F>) -> bool {
    let mut claimed_sum = proof.sum;
    let mut transcript: Transcript<F, Keccak256> = Transcript::init(Keccak256::new());
    transcript.append(&EvaluationForm::to_bytes(&polynomial.eval_form));
    transcript.append(claimed_sum.into_bigint().to_bytes_be().as_slice());
    let mut random_challenges:Vec<F> = Vec::new();
    for poly in proof.polynomials {
        let derived_sum = get_sum_at_0_and_1(&poly);
        println!("derived sum -> {}",derived_sum);
        if claimed_sum != derived_sum {
            return false;
        }
        transcript.append(&EvaluationForm::to_bytes(&poly));
        let random_challenge = transcript.hash();
        random_challenges.push(random_challenge);
        println!("verifier random chall-> {:?}", random_challenge);
        claimed_sum = interpolate_and_evaluate((poly[0], poly[1]), random_challenge);
    }

    // Oracle check
    if polynomial.evaluate(random_challenges) != claimed_sum{
        return false;
    }
    return true;
}

#[cfg(test)]

mod tests {
    use super::*;
    use ark_bn254::Fq;

    fn get_test_poly() -> EvaluationForm<Fq> {
        EvaluationForm::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(3),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(5),
        ])
    }

    fn get_test_poly2() -> EvaluationForm<Fq> {
        EvaluationForm::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(4),
        ])
    }

    #[test]
    fn test_get_sum_at_0_and_1() {
        let sum1 = get_sum_at_0_and_1(&get_test_poly().eval_form);
        assert_eq!(sum1, Fq::from(10));
        let sum2 = get_sum_at_0_and_1(&get_test_poly2().eval_form);
        assert_eq!(sum2, Fq::from(12));
    }

    #[test]
    fn test_evaluate_at_two_vars() {
        let poly1 = get_test_poly();

        assert_eq!(
            evaluate_at_two_vars(&poly1.eval_form, 1 as usize),
            vec![Fq::from(3), Fq::from(7)]
        );
        let poly2 = get_test_poly2();

        assert_eq!(
            evaluate_at_two_vars(&poly2.eval_form, 1 as usize),
            vec![Fq::from(2), Fq::from(10)]
        );

        assert_eq!(
            evaluate_at_two_vars(&poly1.eval_form, 2 as usize),
            vec![Fq::from(0), Fq::from(10)]
        );
    }

    #[test]
    fn test_prove() {
        let mut poly1 = get_test_poly();
        let sum = get_sum_at_0_and_1(&poly1.eval_form);
        let proof = prove(&mut poly1, sum);
        assert_eq!(proof.sum, Fq::from(10));
    }
    #[test]
    fn test_prove2() {
        let mut poly2 = get_test_poly2();
        let sum  = get_sum_at_0_and_1(&poly2.eval_form);
        let proof = prove(&mut poly2, sum);
        assert_eq!(proof.sum, Fq::from(12));
    }

    #[test]
    fn test_prove_and_verify_valid_proof() {
        let mut poly1 = get_test_poly();
        let sum = get_sum_at_0_and_1(&poly1.eval_form);
        let proof = prove(&mut poly1, sum);
        let is_valid = verify(proof, &mut poly1);
        assert_eq!(is_valid, true);
    }

    // #[test]
    // fn test_prove_and_verify_valid_proof2() {
    //     let mut poly2 = get_test_poly2();
    //     let sum = get_sum_at_0_and_1(&poly2.eval_form);
    //     let proof = prove(&mut poly2, sum);
    //     let is_valid = verify(proof, &poly2);
    //     assert_eq!(is_valid, true);
    // }
}
