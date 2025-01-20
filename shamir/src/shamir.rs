use rand::Rng;
use std::f64;
use univariate::polynomial::UnivariatePolynomialDense;

// this fn takes in a secret, threshold, and shares_no, it returns a vector of tuples representing the shares
// instead of making the secret a point at 0, we can make it a point at any x value, where x is the password
// todo: implement the function
// make it use password instead

pub fn share_secret(secret: f64, threshold: u64, shares_no: u64) -> Vec<(f64, f64)> {
    let mut y_values: Vec<f64> = vec![secret];

    let mut rng = rand::thread_rng();
    for _i in 0..threshold-1 {
        let y: f64 = rng.gen_range(1000..1000000000) as f64;
        y_values.push(y as f64);
    }

    let poly = UnivariatePolynomialDense::new(y_values);

    println!("Actual polynomial: {:?}",poly.coefficients);
    
    let mut shares: Vec<(f64, f64)> = Vec::new();
    
    for _i in 0..shares_no {
        let x: f64 = rng.gen();
        let y = poly.evaluate(x as f64);
        shares.push((x as f64, y as f64));
    }
    shares
}

// recovers the secret from the shares
pub fn recover_secret(secret_shares: Vec<(f64, f64)>) -> f64 {
    let x_values: Vec<f64> = secret_shares.iter().map(|(x, _y)| *x as f64).collect();
    let y_values: Vec<f64> = secret_shares.iter().map(|(_x, y)| *y as f64).collect();
    let resulting_poly = UnivariatePolynomialDense::interpolate(x_values, y_values);
    println!("Resulting poly: {:?}", resulting_poly.coefficients);
    resulting_poly.evaluate(0.0).round() as f64
}

#[cfg(test)]

mod tests {
    use super::*;

    fn return_values() -> (f64, u64, u64) {
        let secret = 34028236692.0;
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
    fn test_recover_secret() {
        let (secret, threshold, shares_no) = return_values();
        let shares = share_secret(secret, threshold, shares_no);
        let first_4_shares: Vec<(f64, f64)> = shares.iter().take(3).cloned().collect();
        let recovered_secret = recover_secret(first_4_shares);
        assert_eq!(secret, recovered_secret);
    }

    // fn test_recover_secret() {}
}
