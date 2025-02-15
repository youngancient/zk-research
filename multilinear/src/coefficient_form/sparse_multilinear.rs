use ark_ff::PrimeField;
// pub struct Monomial <F:PrimeField>{
//     coefficient : F,
//     variables : Vec<F>
// }
pub struct SparseMultilinear<F: PrimeField> {
    pub poly_rep: Vec<(Vec<F>, F)>,
}

impl<F: PrimeField> SparseMultilinear<F> {
    pub fn new(polynomial: Vec<(Vec<F>, F)>) -> Self {
        Self {
            poly_rep: polynomial,
        }
    }
}
