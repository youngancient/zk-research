use ark_ff::{BigInteger, PrimeField};
use fiat_shamir::fiat_shamir::Transcript;
use multilinear::evaluation_form::{interpolate_and_evaluate, MultilinearEvalForm};
use sha3::{Digest, Keccak256};

#[derive(Debug, Clone)]

pub struct Proof<F: PrimeField> {
    sum: F,
    polynomials: Vec<Vec<F>>,
}

// sums the univariate poly [a1,a2] at a= 0 and a = 1
pub fn get_sum_over_hypercube<F: PrimeField>(polynomial: &Vec<F>) -> F {
    let sum: F = polynomial.iter().sum();
    sum
}

// splits an array in a tuple of 2 vectors
// where the starting indexes of the elements in the first is 0
// and that of the second is 1
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

pub fn evaluate_at_n_vars<F:PrimeField>(poly_vec: &Vec<F>, var: usize){
    println!("Var -> {var}");
    // todo!()
}

// proves that the claim_sum was derived from the polynomial
// returns a proof
pub fn prove<F: PrimeField>(polynomial: &mut MultilinearEvalForm<F>, claim_sum: F) -> Proof<F> {
    let mut transcript: Transcript<F, Keccak256> = Transcript::init(Keccak256::new());
    transcript.append(&MultilinearEvalForm::to_bytes(&polynomial.eval_form));

    partial_prove(polynomial, claim_sum, &mut transcript)
}

pub fn partial_prove<F: PrimeField>(
    polynomial: &mut MultilinearEvalForm<F>,
    claim_sum: F,
    transcript: &mut Transcript<F, Keccak256>,
) -> Proof<F> {
    transcript.append(claim_sum.into_bigint().to_bytes_be().as_slice());

    let mut proof: Proof<F> = Proof {
        sum: F::zero(),
        polynomials: Vec::new(),
    };
    proof.sum = claim_sum;
    println!("======================");
    println!("poly vars -> {}", polynomial.number_of_variables);
    for i in 1..=polynomial.number_of_variables {
        // transcript.append();
        // println!("poly -> {:?}", polynomial.eval_form);
        let univariate_poly = evaluate_at_two_vars(&polynomial.eval_form, 1 as usize);
        println!("univariate ->-> {:?}", univariate_poly);
        evaluate_at_n_vars(&polynomial.eval_form, 1 as usize);
        transcript.append(&MultilinearEvalForm::to_bytes(&univariate_poly));

        let challenge = transcript.hash();

        proof.polynomials.push(univariate_poly);
        // println!("sum -> {:?}", get_sum_over_hypercube(&polynomial.eval_form));
        polynomial.partial_evaluate(i, challenge);
        println!("==========================");
        println!("poly after partial eval -> {:?}", polynomial.eval_form);
        println!("==========================");
    }
    println!("proof -> {:?}", proof);
    proof
}

// verifies that the claim_sum was gotten from the polynomial based on the proof provided
pub fn verify<F: PrimeField>(proof: Proof<F>, polynomial: &mut MultilinearEvalForm<F>) -> bool {
    let mut transcript: Transcript<F, Keccak256> = Transcript::init(Keccak256::new());
    transcript.append(&MultilinearEvalForm::to_bytes(&polynomial.eval_form));

    let (is_partially_verified, claimed_sum, random_challenges) =
        partial_verify(&mut transcript, proof);
    if !is_partially_verified {
        return false;
    }
    let derived_sum = polynomial.evaluate(&random_challenges);
    // oracle check
    if claimed_sum != derived_sum {
        return false;
    }
    true
}

pub fn partial_verify<F: PrimeField>(
    transcript: &mut Transcript<F, Keccak256>,
    proof: Proof<F>,
) -> (bool, F, Vec<F>) {
    transcript.append(proof.sum.into_bigint().to_bytes_be().as_slice());

    let mut claimed_sum = proof.sum;
    let mut random_challenges: Vec<F> = Vec::new();
    for univariate_poly in proof.polynomials {
        let verified_sum = get_sum_over_hypercube(&univariate_poly);

        // checks if the sums are equal
        if claimed_sum != verified_sum {
            return (false, claimed_sum, random_challenges);
        }

        transcript.append(&MultilinearEvalForm::to_bytes(&univariate_poly));

        let challenge = transcript.hash();
        // the sum for the next univariate_poly
        claimed_sum = interpolate_and_evaluate((univariate_poly[0], univariate_poly[1]), challenge);

        random_challenges.push(challenge);
    }

    (true, claimed_sum, random_challenges)
}

#[cfg(test)]

mod tests {
    use super::*;
    use ark_bn254::Fq;

    fn get_test_poly() -> MultilinearEvalForm<Fq> {
        MultilinearEvalForm::new(vec![
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

    fn get_test_poly2() -> MultilinearEvalForm<Fq> {
        MultilinearEvalForm::new(vec![
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

    fn get_test_poly3() -> MultilinearEvalForm<Fq> {
        MultilinearEvalForm::new(vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)])
    }

    fn get_test_poly4() -> MultilinearEvalForm<Fq> {
        MultilinearEvalForm::new(vec![Fq::from(0), Fq::from(5)])
    }

    // #[test]
    // fn test_get_sum_over_hypercube() {
    //     let sum1 = get_sum_over_hypercube(&get_test_poly().eval_form);
    //     assert_eq!(sum1, Fq::from(10));
    //     let sum2 = get_sum_over_hypercube(&get_test_poly2().eval_form);
    //     assert_eq!(sum2, Fq::from(12));
    // }

    // #[test]
    // fn test_evaluate_at_two_vars() {
    //     let mut poly1 = get_test_poly();

    //     assert_eq!(
    //         evaluate_at_two_vars(&mut poly1, 1 as usize),
    //         vec![Fq::from(3), Fq::from(7)]
    //     );
    //     let mut poly2 = get_test_poly2();

    //     assert_eq!(
    //         evaluate_at_two_vars(&mut poly2, 1 as usize),
    //         vec![Fq::from(2), Fq::from(10)]
    //     );

    //     let mut poly3 = get_test_poly3();

    //     assert_eq!(
    //         evaluate_at_two_vars(&mut poly3, 1 as usize),
    //         vec![Fq::from(3), Fq::from(7)]
    //     );
    //     let mut poly4 = get_test_poly4();
    //     assert_eq!(
    //         evaluate_at_two_vars(&mut poly4, 1 as usize),
    //         vec![Fq::from(0), Fq::from(5)]
    //     );
    // }

    #[test]
    fn test_prove() {
        let mut poly1 = get_test_poly();
        let sum = get_sum_over_hypercube(&poly1.eval_form);
        let proof = prove(&mut poly1, sum);
        assert_eq!(proof.sum, Fq::from(10));
    }
    // #[test]
    // fn test_partial_eval() {
    //     let mut polynomial = MultilinearEvalForm::new(vec![
    //         Fq::from(0),
    //         Fq::from(0),
    //         Fq::from(0),
    //         Fq::from(3),
    //         Fq::from(0),
    //         Fq::from(0),
    //         Fq::from(2),
    //         Fq::from(5),
    //     ]);
    //     let challenges = [Fq::from(1),Fq::from(2),Fq::from(3)];
    //     for i in 1..=polynomial.number_of_variables {
    //         println!("polynomial -> {:?}",polynomial.eval_form);
    //         let fl = polynomial.clone().partial_evaluate(i, Fq::from(0));
    //         let sl = polynomial.clone().partial_evaluate(i, Fq::from(1));
            
    //        let eval = polynomial.partial_evaluate(i, challenges[(i - 1) as usize]);
    //        println!("eval form -> {:?}",eval);
    //     }
    // }
    // #[test]
    // fn test_prove2() {
    //     let mut poly2 = get_test_poly2u();
    //     let sum = get_sum_over_hypercube(&poly2.eval_form);
    //     let proof = prove(&mut poly2, sum);
    //     assert_eq!(proof.sum, Fq::from(12));
    // }

    // #[test]
    // fn test_prove_and_verify_valid_proof() {
    //     let mut poly1 = get_test_poly();
    //     let sum = get_sum_over_hypercube(&poly1.eval_form);
    //     let proof = prove(&mut poly1.clone(), sum);
    //     let is_valid = verify(proof, &mut poly1);
    //     assert_eq!(is_valid, true);
    // }

    // #[test]
    // fn test_prove_and_verify_valid_proof2() {
    //     let mut poly2 = get_test_poly2();
    //     let sum = get_sum_over_hypercube(&poly2.eval_form);
    //     let proof = prove(&mut poly2.clone(), sum);
    //     let is_valid = verify(proof, &mut poly2);
    //     assert_eq!(is_valid, true);
    // }

    // #[test]
    // fn test_prove_and_verify_valid_proof_of_2vars() {
    //     let mut poly3 = get_test_poly3();
    //     let sum = get_sum_over_hypercube(&poly3.eval_form);
    //     let proof = prove(&mut poly3.clone(), sum);
    //     let is_valid = verify(proof, &mut poly3);
    //     assert_eq!(is_valid, true);
    // }

    // #[test]
    // fn test_prove_and_verify_invalid_proof() {
    //     let mut poly1 = get_test_poly();
    //     let sum = Fq::from(100000); // guessed sum
    //     let invalid_proof = prove(&mut poly1.clone(), sum); // invalid proof
    //     let is_valid = verify(invalid_proof, &mut poly1);
    //     assert_eq!(is_valid, false);
    // }
}
