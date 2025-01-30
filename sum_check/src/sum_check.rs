use ark_ff::PrimeField;
use univariate::dense_polynomial::UnivariatePolynomialDense;

pub struct Prover{

}

impl Prover{

}

pub struct Proof <F:PrimeField>{
    pub claimed_sum : F,
    pub polynomials : Vec<UnivariatePolynomialDense<F>>
}

pub struct Verifier{

}   

impl Verifier{

}