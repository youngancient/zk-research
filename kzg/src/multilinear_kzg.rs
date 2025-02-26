use ark_ff::PrimeField;
use fiat_shamir::fiat_shamir::Transcript;

// returns the powers of Tau
pub fn trusted_setup<F: PrimeField>(n: u32) -> Vec<F> {
    let mut powers_of_tau = vec![F::zero(); n as usize];
    let mut rng = rand::thread_rng();
    for i in 0..n {
        let y: F = F::rand(&mut rng);
        powers_of_tau[i as usize] = y;
    }
    powers_of_tau
}

// @note prover
// Generates the commitment to the polynomial by evaluating at the poly[f(a,b,c)] at the taus
// add the commitment to the transcript
// get n-number of random challenges -> a, b , c
// evaluate the polynomial at the random challenges to get V
// find Q(...)
// numerator = f(a,b,c) - v
// for each variable , we have an iteration
// for which we:
// i) find the remainder by partially evaluating the poly for the round at the current variable
// ii) find the quotient by partially evaluating the poly(in the form it was before the first partial evaluation) when the current var = 1, 
//  and when the current var is 0, and subtracting the both.


pub fn prove() {}

//
pub fn verify() {}

#[cfg(test)]

mod test {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_trusted_setup() {
        let n = 3;
        assert_eq!(trusted_setup::<Fq>(3).len(), n);
    }
}
