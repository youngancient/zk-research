use rand::Rng;
use std::u128;
use univariate::polynomial::UnivariatePolynomialDense;

// this fn takes in a secret, threshold, and shares_no, it returns a vector of tuples representing the shares
// instead of making the secret a point at 0, we can make it a point at any x value, where x is the password
// todo: implement the function
// make it use password instead
const MIN_THRESHOLD_VALUE: u128 = u128::MAX / 2;

pub fn share_secret(secret: u128, threshold: u128, shares_no: u128) -> Vec<(u128, u128)> {
    let mut shares: Vec<(u128, u128)> = Vec::new();
    let mut x_values: Vec<f64> = vec![0.0];
    let mut y_values: Vec<f64> = vec![secret as f64];

    let mut rng = rand::thread_rng();
    for _i in 0..threshold {
        let x = rng.gen_range(MIN_THRESHOLD_VALUE..=u128::MAX);
        let y = rng.gen_range(MIN_THRESHOLD_VALUE..=u128::MAX);
        x_values.push(x as f64);
        y_values.push(y as f64);
        shares.push((x, y));
    }
    let poly = UnivariatePolynomialDense::interpolate(x_values, y_values);
    for _i in threshold..shares_no {
        let x = rng.gen_range(MIN_THRESHOLD_VALUE..=u128::MAX);
        let y = poly.evaluate(x as f64);
        shares.push((x, y as u128));
    }
    shares
}

// recovers the secret from the shares
pub fn recover_secret(secret_shares: Vec<(u128, u128)>) -> u128 {
    todo!();
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_share_secret() {
        let secret = 340282366920938463463374607431768211455;
        let threshold = 4;
        let shares_no = 10;
        let shares = share_secret(secret, threshold, shares_no);
        assert_eq!(shares.len(), shares_no as usize);
    }

    // fn test_recover_secret() {}
}
