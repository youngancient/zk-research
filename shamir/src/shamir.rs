use ark_ff::PrimeField;
use rand;
use univariate::polynomial::UnivariatePolynomialDense;
// this fn takes in a secret, threshold, and shares_no, it returns a vector of tuples representing the shares
// instead of making the secret a point at 0, we can make it a point at any x value, where x is the password
// todo: implement the function
// make it use password instead

pub fn share_secret<F: PrimeField>(secret: F, threshold: u64, shares_no: u64) -> Vec<(F, F)> {
    let mut y_values: Vec<F> = vec![secret];

    let mut rng = rand::thread_rng();

    for _i in 0..threshold - 1 {
        let y: F = F::rand(&mut rng);
        y_values.push(y as F);
    }

    let poly = UnivariatePolynomialDense::new(y_values);

    println!("Actual polynomial: {:?}", poly.coefficients);

    let mut shares: Vec<(F, F)> = Vec::new();

    for _i in 0..shares_no {
        let x: F = F::rand(&mut rng);
        let y = poly.evaluate(x as F);
        shares.push((x as F, y as F));
    }
    shares
}

pub fn share_secret_with_password<F: PrimeField>(
    secret: F,
    threshold: u64,
    shares_no: u64,
    password: F,
) -> Vec<(F, F)> {
    let mut rng = rand::thread_rng();
    let mut x_values: Vec<F> = vec![password];
    let mut y_values: Vec<F> = vec![secret];

    for _i in 0..threshold - 1 {
        let y: F = F::rand(&mut rng);
        let x: F = F::rand(&mut rng);
        y_values.push(y as F);
        x_values.push(x as F);
    }

    let poly = UnivariatePolynomialDense::interpolate(x_values, y_values);
    if poly.degree != threshold - 1 {
        panic!("Polynomial interpolation failed");
    }

    let mut shares: Vec<(F, F)> = Vec::new();

    for _i in 0..shares_no {
        let x: F = F::rand(&mut rng);
        let y = poly.evaluate(x as F);
        shares.push((x as F, y as F));
    }
    shares
}

// recovers the secret from the shares
pub fn recover_secret<F: PrimeField>(secret_shares: Vec<(F, F)>, password: F) -> F {
    let x_values: Vec<F> = secret_shares.iter().map(|(x, _y)| *x as F).collect();
    let y_values: Vec<F> = secret_shares.iter().map(|(_x, y)| *y as F).collect();
    let resulting_poly = UnivariatePolynomialDense::interpolate(x_values, y_values);
    println!("Resulting poly: {:?}", resulting_poly.coefficients);
    resulting_poly.evaluate(password)
}

#[cfg(test)]

mod tests {
    use std::i32;

    use super::*;
    use ark_bn254::Fq;

    fn return_values() -> (Fq, u64, u64) {
        let secret = Fq::from(i32::MAX);
        let threshold = 4;
        let shares_no = 10;
        (secret, threshold, shares_no)
    }
    #[test]
    fn test_share_secret() {
        let (secret, threshold, shares_no) = return_values();
        let shares = share_secret(secret, threshold, shares_no);
        assert_eq!(shares.len(), shares_no as usize);
    }

    #[test]
    fn test_recover_secret_success() {
        let (secret, threshold, shares_no) = return_values();
        let shares = share_secret(secret, threshold, shares_no);
        let first_4_shares: Vec<(Fq, Fq)> = shares.iter().take(4).cloned().collect();
        let password = Fq::from(0);

        let recovered_secret = recover_secret(first_4_shares, password);
        assert_eq!(secret, recovered_secret);
    }

    #[test]
    fn test_interpolate_share_secret() {
        let (secret, threshold, shares_no) = return_values();
        let password = Fq::from(0);
        let shares = share_secret_with_password(secret, threshold, shares_no, password);

        let first_4_shares: Vec<(Fq, Fq)> = shares.iter().take(4).cloned().collect();
        let recovered_secret = recover_secret(first_4_shares, password);

        assert_eq!(secret, recovered_secret);
    }
}
