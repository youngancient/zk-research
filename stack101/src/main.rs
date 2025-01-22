fn main() {
    println!("Hello, world!");
}

#[cfg(test)]

mod tests {
    use super::*;
    use ark_bn254::Fq;
    use univariate::dense_polynomial::UnivariatePolynomialDense;

    #[test]
    fn test_fib() {
        let x_values = vec![
            Fq::from(0),
            Fq::from(1),
            Fq::from(2),
            Fq::from(3),
            Fq::from(4),
            Fq::from(5),
            Fq::from(6),
            Fq::from(7),
        ];
        let y_values = vec![
            Fq::from(1),
            Fq::from(1),
            Fq::from(2),
            Fq::from(3),
            Fq::from(5),
            Fq::from(8),
            Fq::from(13),
            Fq::from(21),
        ];

        let poly = UnivariatePolynomialDense::interpolate(x_values, y_values);

        // test for degree
        assert_eq!(poly.degree, 7);

        // test boundary condition
        assert_eq!(poly.evaluate(Fq::from(0)), Fq::from(1));
        assert_eq!(poly.evaluate(Fq::from(1)), Fq::from(1));

        // f(x) = f(x- 1) + f(x - 2)
        let x = Fq::from(5);
        assert_eq!(
            poly.evaluate(x),
            poly.evaluate(x - Fq::from(1)) + poly.evaluate(x - Fq::from(2))
        );

        // f(7) = 21

        assert_eq!(poly.evaluate(Fq::from(7)), Fq::from(21));
    }
}
