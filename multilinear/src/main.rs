use ark_bn254::Fq;
use multilinear::evaluation_form::{self, find_pairs_with_xor, EvaluationForm};

fn main() {
    println!("Hello world, lfg!");
    let form = evaluation_form::EvaluationForm::new(vec![
        Fq::from(0),
        Fq::from(0),
        Fq::from(0),
        Fq::from(0),
        Fq::from(0),
        Fq::from(3),
        Fq::from(0),
        Fq::from(3),
        Fq::from(0),
        Fq::from(0),
        Fq::from(0),
        Fq::from(0),
        Fq::from(0),
        Fq::from(3),
        Fq::from(2),
        Fq::from(5),
    ]);
    println!("num of vars: {}", form.number_of_variables);
    println!("Boolean hypercube: {:?}", form.boolean_hypercube);

    // let variable_position = 1; // referring to a
    // let value = Fq::from(4);
    // let pt = form.partial_evaluate(variable_position, value);
    // // println!("{:?}", pt); 011

    // the order of the variables is important -> [a, b, c, d,...] for f(a,b,c,d,...)
    let pairs = find_pairs_with_xor(&form.boolean_hypercube, 3);
    println!("pairs: {:?}", pairs);
}
