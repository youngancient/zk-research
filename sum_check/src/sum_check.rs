use ark_ff::{BigInteger, PrimeField};
use fiat_shamir::fiat_shamir::Transcript;
use multilinear::evaluation_form::{
    convert_to_fq_elements, interpolate_and_evaluate, MultilinearEvalForm, ProdPoly,
};
use sha3::{Digest, Keccak256};
use univariate::dense_polynomial::UnivariatePolynomialDense;

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

// pub fn evaluate_at_n_vars<F:PrimeField>(poly_vec: &Vec<F>, var: usize){
//     println!("Var -> {var}");
//     // todo!()
// }

// @note proves that the claim_sum was derived from the polynomial
// returns a proof
pub fn prove<F: PrimeField>(polynomial: &mut MultilinearEvalForm<F>, claim_sum: F) -> Proof<F> {
    let mut transcript: Transcript<F, Keccak256> = Transcript::init(Keccak256::new());
    transcript.append(&MultilinearEvalForm::to_bytes(&polynomial.eval_form));

    partial_prove(polynomial, claim_sum, &mut transcript)
}

// @note performs partial prove, does not add initial poly to transcript
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
    for i in 1..=polynomial.number_of_variables {
        let univariate_poly = evaluate_at_two_vars(&polynomial.eval_form, 1 as usize);
        // evaluate_at_n_vars(&polynomial.eval_form, 1 as usize);
        transcript.append(&MultilinearEvalForm::to_bytes(&univariate_poly));

        let challenge = transcript.hash();

        proof.polynomials.push(univariate_poly);
        polynomial.partial_evaluate(i, challenge);
    }
    proof
}

//  @note verifies that the claim_sum was gotten from the polynomial based on the proof provided
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

// @note partially verifies i.e doesnt commit the initial poly
// does not perform oracle check
// returns (bool, F, Vec<F>)
// bool -> if the round checks were successful
// F -> the last claimed_sum
// Vec<F> -> A list of all the random challenges (r)
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

// @note sumcheck over a product of 2 multilinear polys

// prove sum over the Boolean HC of a prod poly
pub fn prove_prod_poly<F: PrimeField>(claim_sum: F, prod_poly: &mut ProdPoly<F>) -> Proof<F> {
    let mut transcript: Transcript<F, Keccak256> = Transcript::init(Keccak256::new());
    // add polynomial to transcript
    transcript.append(&prod_poly.to_bytes());

    partial_prove_prod_poly(prod_poly, claim_sum, &mut transcript)
}

//  @note partial prove for prod_poly
pub fn partial_prove_prod_poly<F: PrimeField>(
    prod_poly: &mut ProdPoly<F>,
    claim_sum: F,
    transcript: &mut Transcript<F, Keccak256>,
) -> Proof<F> {
    transcript.append(claim_sum.into_bigint().to_bytes_be().as_slice());

    let mut proof: Proof<F> = Proof {
        sum: F::zero(),
        polynomials: Vec::new(),
    };

    proof.sum = claim_sum;

    for i in 0..prod_poly.no_of_vars {
        let mut univariate_poly = vec![F::zero(); 3];
        // replace this fixed value with variable - degree
        // how do i get the degree? (I currently have number_of_variables)
        for j in 0..=2 {
            let partial_evaluation: Vec<F> =
                prod_poly.clone().partial_evaluate(i + 1, F::from(j as u32));
            univariate_poly[j] = partial_evaluation.iter().sum();
        }
        transcript.append(&MultilinearEvalForm::to_bytes(&univariate_poly));
        let challenge = transcript.hash();
        prod_poly.partial_evaluate(i + 1, challenge);

        proof.polynomials.push(univariate_poly);
    }
    proof
}

//  @note verify the proofs of the prove_prod_poly
pub fn verify_prod_poly<F: PrimeField>(proof: Proof<F>, prod_poly: &mut ProdPoly<F>) -> bool {
    let mut transcript: Transcript<F, Keccak256> = Transcript::init(Keccak256::new());
    transcript.append(&prod_poly.to_bytes());

    let (is_partially_verified, claimed_sum, random_challenges) =
        partial_verify_prod_poly(&mut transcript, proof);
    if !is_partially_verified {
        return false;
    }
    let derived_sum = prod_poly.evaluate(&random_challenges);

    // oracle check
    if claimed_sum != derived_sum {
        return false;
    }
    true
}

// @note partially verifies i.e doesnt commit the initial poly
// does not perform oracle check
// returns (bool, F, Vec<F>)
// bool -> if the round checks were successful
// F -> the last claimed_sum
// Vec<F> -> A list of all the random challenges (r)
pub fn partial_verify_prod_poly<F: PrimeField>(
    transcript: &mut Transcript<F, Keccak256>,
    proof: Proof<F>,
) -> (bool, F, Vec<F>) {
    transcript.append(proof.sum.into_bigint().to_bytes_be().as_slice());
    let mut claimed_sum = proof.sum;
    let mut random_challenges: Vec<F> = Vec::new();
    println!("=====================================");
    println!("first claimed sum -> {claimed_sum}");
    for poly in &proof.polynomials {
        println!("the poly -> {:?}",poly);
        let verified_sum: F = poly[0] + poly[1]; // get sum over boolean HC
        println!("verified_sum -> {verified_sum}");
        // checks if the sums are equal
        if claimed_sum != verified_sum {
            return (false, claimed_sum, random_challenges);
        }
        transcript.append(&MultilinearEvalForm::to_bytes(&poly));
        let interpolated_poly = interpolate_to_univariate(&poly);
        // evaluate the poly at random_challenge to get the next claim_sum
        let challenge = transcript.hash();
        // the claim_sum for the next univraite_poly
        claimed_sum = interpolated_poly.evaluate(challenge);
        println!("next claim sum -> {claimed_sum}");
        random_challenges.push(challenge);
    }
    (true, claimed_sum, random_challenges)
}

// @note function takes in the univariate poly
// splits into [x0,x1,x2] and [y0,y1,y2]
// interpolates to get a univariate poly
pub fn interpolate_to_univariate<F: PrimeField>(poly: &Vec<F>) -> UnivariatePolynomialDense<F> {
    let x_values = (0..poly.len()).map(|i| F::from(i as u64)).collect();
    UnivariatePolynomialDense::interpolate(x_values, poly.to_vec())
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

    fn get_prod_poly() -> ProdPoly<Fq> {
        let polynomials = vec![
            MultilinearEvalForm::new(vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]),
            MultilinearEvalForm::new(vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]),
        ];
        let prod_poly = ProdPoly::new(polynomials);
        prod_poly
    }

    fn get_prod_poly2() -> ProdPoly<Fq> {
        let polynomials = vec![
            MultilinearEvalForm::new(convert_to_fq_elements(vec![0, 0, 0, 3, 0, 0, 2, 5])),
            MultilinearEvalForm::new(convert_to_fq_elements(vec![0, 0, 0, 3, 0, 0, 2, 5])),
        ];
        let prod_poly = ProdPoly::new(polynomials);
        prod_poly
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

    #[test]
    fn test_prove_prod_poly() {
        let mut prod_poly = get_prod_poly();
        let claim_sum = prod_poly.reduce().iter().sum();
        prove_prod_poly(claim_sum, &mut prod_poly);
    }

    #[test]
    fn test_prove_prod_poly2() {
        let mut prod_poly = get_prod_poly2();
        let claim_sum = prod_poly.reduce().iter().sum();
        prove_prod_poly(claim_sum, &mut prod_poly);
    }

    #[test]
    fn test_prove_and_verify_prod_poly() {
        let mut prod_poly = get_prod_poly();
        let claim_sum = prod_poly.reduce().iter().sum();
        let proof = prove_prod_poly(claim_sum, &mut prod_poly);
        let is_valid = verify_prod_poly(proof, &mut prod_poly);
        assert_eq!(is_valid,true);
    }

    #[test]
    fn test_prove_and_verify_prod_poly2() {
        let mut prod_poly = get_prod_poly2();
        let claim_sum = prod_poly.reduce().iter().sum();
        let proof = prove_prod_poly(claim_sum, &mut prod_poly);
        let is_valid = verify_prod_poly(proof, &mut prod_poly);
        assert_eq!(is_valid,true);
    }
}
