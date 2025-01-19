use std::f64;

/// A struct representing a univariate polynomial in dense form
pub struct UnivariatePolynomialDense {
    degree: u32,
    coefficients: Vec<f64>,
}

impl UnivariatePolynomialDense {
    /// Creates a new `UnivariatePolynomialDense` with the given coefficients.
    ///
    /// # Arguments
    ///
    /// * `coefficients` - A vector of coefficients, where the i-th element is the coefficient for x^i.
    pub fn new(coefficients: Vec<f64>) -> Self {
        let degree: u32 = if coefficients.is_empty() {
            0
        } else {
            (coefficients.len() - 1) as u32
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
    pub fn evaluate(&self, x: f64) -> f64 {
        // logic
        let mut val: f64 = 0.0;
        for (i, value) in self.coefficients.iter().enumerate() {
            if *value != 0.0 {
                val += value * x.powf(i as f64);
            }
        }
        val
    }

    // polynomial addition
    pub fn polynomial_addition(
        &self,
        other: &UnivariatePolynomialDense,
    ) -> UnivariatePolynomialDense {
        let mut origin_coefficients: Vec<f64> = Vec::new();
        let mut other_cofficients: Vec<f64> = Vec::new();

        let mut result_coefficients: Vec<f64> = Vec::new();

        let degree: u32;

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
                result_coefficients.push(value + other_cofficients[i]);
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
        other: &UnivariatePolynomialDense,
    ) -> UnivariatePolynomialDense {
        let m = self.coefficients.len();
        let n = other.coefficients.len();
        let no_of_coefficients = m + n - 1;
        let mut prod_array: Vec<f64> = vec![0.0; (no_of_coefficients) as usize];
        for i in 0..m as usize {
            for j in 0..n as usize {
                prod_array[i + j] += self.coefficients[i] * other.coefficients[j];
            }
        }

        UnivariatePolynomialDense::new(prod_array)
    }

    // polynomial scalar multiplication

    pub fn scalar_multiplication(&self, scalar: f64) -> UnivariatePolynomialDense {
        let mut product: Vec<f64> = Vec::new();
        for val in self.coefficients.iter() {
            product.push(val * (scalar as f64))
        }
        UnivariatePolynomialDense {
            degree: self.degree,
            coefficients: product,
        }
    }

    pub fn lagrange_basis(
        interpolating_set: Vec<f64>,
        focus_point: f64,
        y_value: f64,
    ) -> UnivariatePolynomialDense {
        // set basis polynomial to constant 1
        let mut basis_poly_numerator: UnivariatePolynomialDense =
            UnivariatePolynomialDense::new(vec![1.0]);

        for x_value in interpolating_set.iter() {
            // for the numerator
            // (x - 1)(x - 2)...(x - n)
            if *x_value != focus_point {
                let univariate_poly = UnivariatePolynomialDense::new(vec![-(*x_value), 1.0]);
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
    pub fn interpolate(x_values: Vec<f64>, y_values: Vec<f64>) -> UnivariatePolynomialDense {
        // let x_values: Vec<f64> = (0..y_values.len()).map(|x| x as f64).collect();
        if x_values.len() != y_values.len() {
            panic!("The number of x values must be equal to the number of y values");
        }
        // polyniomial sum :: but we set it to the zero polynomial at first or additive identity
        let mut polynomial_sum: UnivariatePolynomialDense =
            UnivariatePolynomialDense::new(vec![0.0]);

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

    fn poly1() -> UnivariatePolynomialDense {
        UnivariatePolynomialDense::new(vec![1.0, 2.0, 3.0])
    }
    fn poly2() -> UnivariatePolynomialDense {
        UnivariatePolynomialDense::new(vec![5.0, 2.0])
    }

    #[test]
    fn test_create_polynomial() {
        let poly = poly1();
        assert_eq!(poly.degree, 2);
        assert_eq!(poly.coefficients, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_evaluate() {
        let poly = poly1();
        assert_eq!(poly.evaluate(2.0), 17.0);
    }

    #[test]
    fn test_poly_add() {
        let poly_1 = poly1();
        let poly_2 = poly2();
        assert_eq!(
            poly_1.polynomial_addition(&poly_2).coefficients,
            vec![6.0, 4.0, 3.0]
        );
    }

    #[test]
    fn test_poly_mul() {
        let poly_1 = poly1();
        let poly_2 = poly2();
        assert_eq!(
            poly_1.polynomial_multiplication(&poly_2).coefficients,
            vec![5.0, 12.0, 19.0, 6.0]
        );
    }

    #[test]
    fn test_scalar_mul() {
        let poly_1 = poly1();
        let scalar = 2.0;
        assert_eq!(
            poly_1.scalar_multiplication(scalar).coefficients,
            vec![2.0, 4.0, 6.0]
        );
    }

    #[test]
    fn test_interpolate() {
        let result =
            UnivariatePolynomialDense::interpolate(vec![0.0, 1.0, 2.0], vec![2.0, 4.0, 10.0]);
        assert_eq!(result.coefficients, vec![2.0, 0.0, 2.0]);
    }
}
