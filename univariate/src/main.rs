use std::f32;

struct UnivariatePolynomialDense {
    degree: u32,
    coefficients: Vec<f32>,
}

impl UnivariatePolynomialDense {
    fn new(coefficients: Vec<f32>) -> Self {
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

    fn evaluate(&self, x: f32) -> f32 {
        // logic
        let mut val: f32 = 0.0;
        for (i, value) in self.coefficients.iter().enumerate() {
            if *value != 0.0 {
                val += value * x.powf(i as f32);
            }
        }
        val
    }

    fn degree(&self) -> u32 {
        self.degree
    }

    // polynomial addition
    fn polynomial_addition(&self, other: &UnivariatePolynomialDense) -> UnivariatePolynomialDense {
        let mut origin_coefficients: Vec<f32> = Vec::new();
        let mut other_cofficients: Vec<f32> = Vec::new();

        let mut result_coefficients: Vec<f32> = Vec::new();

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

    fn polynomial_multiplication(
        &self,
        other: &UnivariatePolynomialDense,
    ) -> UnivariatePolynomialDense {
        let m = self.coefficients.len();
        let n = other.coefficients.len();
        let no_of_coefficients = m + n - 1;
        let mut prod_array: Vec<f32> = vec![0.0; (no_of_coefficients) as usize];
        for i in 0..m as usize {
            for j in 0..n as usize {
                prod_array[i + j] += self.coefficients[i] * other.coefficients[j];
            }
        }

        UnivariatePolynomialDense::new(prod_array)
    }

    // polynomial scalar multiplication

    fn scalar_multiplication(&self, scalar: f32) -> UnivariatePolynomialDense {
        let mut product: Vec<f32> = Vec::new();
        for val in self.coefficients.iter() {
            product.push(val * (scalar as f32))
        }
        UnivariatePolynomialDense {
            degree: self.degree,
            coefficients: product,
        }
    }

    // input -> [y0,y1,y2], where the x values represent the powers of the variables in ascending order
    fn interpolate(input: Vec<f32>) -> UnivariatePolynomialDense {
        let x_values: Vec<f32> = (0..input.len()).map(|x| x as f32).collect();
        println!("------------");
        println!("{:?}", x_values);

        // polyniomial sum :: but we set it to the zero polynomial at first or additive identity
        let mut polynomial_sum: UnivariatePolynomialDense =
            UnivariatePolynomialDense::new(vec![0.0]);

        for (i, x) in x_values.iter().enumerate() {
            println!("----l{x}--------");
            let single_basis_poly = lagrange_basis(x_values.clone(), *x, input[i]);
            polynomial_sum = polynomial_sum.polynomial_addition(&single_basis_poly);
        }

        println!("--------------------------");
        println!("-----The polynomial-------");
        println!("{:?}", polynomial_sum.coefficients);

        polynomial_sum
    }
}

fn lagrange_basis(
    interpolating_set: Vec<f32>,
    focus_point: f32,
    y_value: f32,
) -> UnivariatePolynomialDense {
    println!("{:?} -> {}", interpolating_set, focus_point);
    // set basis polynomial to constant 1
    let mut basis_poly_numerator: UnivariatePolynomialDense =
        UnivariatePolynomialDense::new(vec![1.0]);
    let mut denominator = 1.0;
    for x_value in interpolating_set.iter() {
        // for the numerator
        // (x - 1)(x - 2)...(x - n)
        if *x_value != focus_point {
            let univariate_poly = UnivariatePolynomialDense::new(vec![-(*x_value), 1.0]);
            basis_poly_numerator = basis_poly_numerator.polynomial_multiplication(&univariate_poly);
            // for the denominator
            denominator *= focus_point - *x_value;
        }
    }
    // get the inverse of the denominator
    // multiply by the y_value
    let denominator_val: f32 = y_value * 1.0 / denominator;

    let basis_poly = basis_poly_numerator.scalar_multiplication(denominator_val);
    println!("------------");
    println!("basis_poly: {:?}", basis_poly.coefficients);

    basis_poly
}

fn main() {
        // call the interpolate function
    UnivariatePolynomialDense::interpolate(vec![2.0, 4.0, 10.0]);
}


// write tests