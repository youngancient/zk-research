use ark_ff::PrimeField;
/// A struct representing a univariate polynomial in dense form
pub struct UnivariatePolynomialDense<F: PrimeField> {
    pub degree: u64,
    pub coefficients: Vec<F>,
}

impl<F: PrimeField> UnivariatePolynomialDense<F> {
    /// Creates a new `UnivariatePolynomialDense` with the given coefficients.
    ///
    /// # Arguments
    ///
    /// * `coefficients` - A vector of coefficients, where the i-th element is the coefficient for x^i.
    pub fn new(coefficients: Vec<F>) -> Self {
        let degree: u64 = if coefficients.is_empty() {
            0
        } else {
            (coefficients.len() - 1) as u64
        };
        Self {
            degree,
            coefficients,
        }
    }

    // Evaluates the polynomial at the given point `x`.
    ///
    /// # Arguments
    ///
    /// * `x` - The point at which to evaluate the polynomial.
    ///
    /// # Returns
    ///
    /// The value of the polynomial at `x`.
    pub fn evaluate(&self, x: F) -> F {
        // logic
        let mut val: F = F::zero();
        for (i, value) in self.coefficients.iter().enumerate() {
            if x == F::zero() {
                return self.coefficients[0];
            }
            if *value != F::zero() {
                val += *value * x.pow([i as u64]);
            }
        }
        val
    }

    // polynomial addition
    pub fn polynomial_addition(
        &self,
        other: &UnivariatePolynomialDense<F>,
    ) -> UnivariatePolynomialDense<F> {
        let mut origin_coefficients: Vec<F> = Vec::new();
        let mut other_cofficients: Vec<F> = Vec::new();

        let mut result_coefficients: Vec<F> = Vec::new();

        let degree: u64;

        if self.degree > other.degree {
            origin_coefficients = self.coefficients.clone();
            other_cofficients = other.coefficients.clone();
            degree = self.degree; // degree of the result is the highest degree polynomial in the addition
        } else {
            origin_coefficients = other.coefficients.clone();
            other_cofficients = self.coefficients.clone();
            degree = other.degree; // degree of the result is the highest degree polynomial in the addition
        }
        for (i, value) in origin_coefficients.iter().enumerate() {
            if i < other_cofficients.len() {
                result_coefficients.push(*value + other_cofficients[i]);
            } else {
                result_coefficients.push(*value);
            }
        }
        UnivariatePolynomialDense {
            degree,
            coefficients: result_coefficients,
        }
    }

    // polynomial multiplication

    pub fn polynomial_multiplication(
        &self,
        other: &UnivariatePolynomialDense<F>,
    ) -> UnivariatePolynomialDense<F> {
        let m = self.coefficients.len();
        let n = other.coefficients.len();
        let no_of_coefficients = m + n - 1;
        let mut prod_array: Vec<F> = vec![F::zero(); (no_of_coefficients) as usize];
        for i in 0..m as usize {
            for j in 0..n as usize {
                prod_array[i + j] += self.coefficients[i] * other.coefficients[j];
            }
        }

        UnivariatePolynomialDense::new(prod_array)
    }

    // polynomial scalar multiplication

    pub fn scalar_multiplication(&self, scalar: F) -> UnivariatePolynomialDense<F> {
        let mut product: Vec<F> = Vec::new();
        for val in self.coefficients.iter() {
            product.push(*val * (scalar as F))
        }
        UnivariatePolynomialDense {
            degree: self.degree,
            coefficients: product,
        }
    }

    pub fn lagrange_basis(
        interpolating_set: Vec<F>,
        focus_point: F,
        y_value: F,
    ) -> UnivariatePolynomialDense<F> {
        // set basis polynomial to constant 1
        let mut basis_poly_numerator: UnivariatePolynomialDense<F> =
            UnivariatePolynomialDense::new(vec![F::one()]);

        for x_value in interpolating_set.iter() {
            // for the numerator
            // (x - 1)(x - 2)...(x - n)
            if *x_value != focus_point {
                let univariate_poly = UnivariatePolynomialDense::new(vec![-(*x_value), F::one()]);
                basis_poly_numerator =
                    basis_poly_numerator.polynomial_multiplication(&univariate_poly);
            }
        }
        // for the denominator
        let denominator = basis_poly_numerator.evaluate(focus_point);
        // get the inverse of the denominator
        // multiply by the y_value

        let basis_poly = basis_poly_numerator.scalar_multiplication(y_value / denominator);

        basis_poly
    }

    // y_values -> [x0,x1,x2 ..., xn] [y0,y1,y2, ...yn]
    pub fn interpolate(x_values: Vec<F>, y_values: Vec<F>) -> UnivariatePolynomialDense<F> {
        // let x_values: Vec<F> = (0..y_values.len()).map(|x| x as F).collect();
        if x_values.len() != y_values.len() {
            panic!("The number of x values must be equal to the number of y values");
        }
        // polyniomial sum :: but we set it to the zero polynomial at first or additive identity
        let mut polynomial_sum: UnivariatePolynomialDense<F> =
            UnivariatePolynomialDense::new(vec![F::zero()]);

        for (i, x) in x_values.iter().enumerate() {
            let single_basis_poly =
                UnivariatePolynomialDense::lagrange_basis(x_values.clone(), *x, y_values[i]);
            polynomial_sum = polynomial_sum.polynomial_addition(&single_basis_poly);
        }

        polynomial_sum
    }
}

// write tests
#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;

    fn poly1() -> UnivariatePolynomialDense<Fq> {
        UnivariatePolynomialDense::new(vec![Fq::from(1), Fq::from(2), Fq::from(3)])
    }
    fn poly2() -> UnivariatePolynomialDense<Fq> {
        UnivariatePolynomialDense::new(vec![Fq::from(5), Fq::from(2)])
    }

    #[test]
    fn test_create_polynomial() {
        let poly = poly1();
        assert_eq!(poly.degree, 2);
        assert_eq!(
            poly.coefficients,
            vec![Fq::from(1), Fq::from(2), Fq::from(3)]
        );
    }

    #[test]
    fn test_evaluate() {
        let poly = poly1();
        assert_eq!(poly.evaluate(Fq::from(2)), Fq::from(17));
    }

    #[test]
    fn test_evaluate_at_zero() {
        let poly = poly1();
        assert_eq!(poly.evaluate(Fq::from(0)), Fq::from(1));
    }

    #[test]
    fn test_poly_add() {
        let poly_1 = poly1();
        let poly_2 = poly2();
        assert_eq!(
            poly_1.polynomial_addition(&poly_2).coefficients,
            vec![Fq::from(6), Fq::from(4), Fq::from(3)]
        );
    }

    #[test]
    fn test_poly_mul() {
        let poly_1 = poly1();
        let poly_2 = poly2();
        assert_eq!(
            poly_1.polynomial_multiplication(&poly_2).coefficients,
            vec![Fq::from(5), Fq::from(12), Fq::from(19), Fq::from(6)]
        );
    }

    #[test]
    fn test_scalar_mul() {
        let poly_1 = poly1();
        let scalar = Fq::from(2);
        assert_eq!(
            poly_1.scalar_multiplication(scalar).coefficients,
            vec![Fq::from(2), Fq::from(4), Fq::from(6)]
        );
    }

    #[test]
    fn test_interpolate() {
        let result = UnivariatePolynomialDense::interpolate(
            vec![Fq::from(0), Fq::from(1), Fq::from(2)],
            vec![Fq::from(2), Fq::from(4), Fq::from(10)],
        );
        assert_eq!(
            result.coefficients,
            vec![Fq::from(2), Fq::from(0), Fq::from(2)]
        );
    }
}
