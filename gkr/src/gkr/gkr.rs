use crate::circuits::{
    circuits::Circuit,
    gates::{Gate, GateOperation},
    layers::Layer,
};
use ark_ff::PrimeField;
// ==============================================================//
//    @note GKR Prover
// =============================================================//
pub struct Proof {}

// circuit and input
pub fn gkr_prover<F: PrimeField>(circuit: Circuit<F>, inputs: Vec<F>) -> Proof {
    // sends output polynomial, w0(a) of circuit evaluation to the verifier (i.e commit to transcript)
    // generate random challenge from transcript
    // evaluate w0(a) at r -> w0(r)
    
    todo!()
}

// ==============================================================//
//    @note GKR Verifier
// =============================================================//
pub fn gkr_verifier<F: PrimeField>(circuit: Circuit<F>, inputs: Vec<F>) -> bool {
    todo!()
}

#[cfg(test)]

mod tests {
    use super::*;
    use ark_bn254::Fq;

    fn get_circuit() -> Circuit<Fq> {
        let l2_gate1 = Gate::new(0, 1, 0, GateOperation::Mul);
        let l2_gate2 = Gate::new(2, 3, 1, GateOperation::Add);
        let l2_gate3 = Gate::new(4, 5, 2, GateOperation::Mul);
        let l2_gate4 = Gate::new(6, 7, 3, GateOperation::Add);
        let layer_two = Layer::new(vec![l2_gate1, l2_gate2, l2_gate3, l2_gate4]);

        let l1_gate1 = Gate::new(0, 1, 0, GateOperation::Add);
        let l1_gate2 = Gate::new(2, 3, 1, GateOperation::Mul);
        let layer_one = Layer::new(vec![l1_gate1, l1_gate2]);

        let l0_gate1 = Gate::new(0, 1, 0, GateOperation::Mul);
        let layer_zero = Layer::new(vec![l0_gate1]);
        Circuit::new(vec![layer_two, layer_one, layer_zero])
    }
    fn get_circuit2() -> Circuit<Fq> {
        let l2_gate1 = Gate::new(0, 1, 0, GateOperation::Add);
        let l2_gate2 = Gate::new(2, 3, 1, GateOperation::Add);
        let l2_gate3 = Gate::new(4, 5, 2, GateOperation::Add);
        let l2_gate4 = Gate::new(6, 7, 3, GateOperation::Mul);
        let layer_two = Layer::new(vec![l2_gate1, l2_gate2, l2_gate3, l2_gate4]);

        let l1_gate1 = Gate::new(0, 1, 0, GateOperation::Mul);
        let l1_gate2 = Gate::new(2, 3, 1, GateOperation::Add);
        let layer_one = Layer::new(vec![l1_gate1, l1_gate2]);

        let l0_gate1 = Gate::new(0, 1, 0, GateOperation::Add);
        let layer_zero = Layer::new(vec![l0_gate1]);
        Circuit::new(vec![layer_two, layer_one, layer_zero])
    }
    fn get_circuit3() -> Circuit<Fq> {
        let l2_gate1 = Gate::new(0, 1, 0, GateOperation::Add);
        let l2_gate2 = Gate::new(2, 3, 1, GateOperation::Add);
        let l2_gate3 = Gate::new(4, 5, 2, GateOperation::Add);
        let l2_gate4 = Gate::new(6, 7, 3, GateOperation::Mul);
        let layer_two = Layer::new(vec![l2_gate1, l2_gate2, l2_gate3, l2_gate4]);

        let l1_gate1 = Gate::new(0, 1, 0, GateOperation::Mul);
        let l1_gate2 = Gate::new(2, 3, 1, GateOperation::Add);
        let layer_one = Layer::new(vec![l1_gate1, l1_gate2]);

        let l0_gate1 = Gate::new(0, 1, 0, GateOperation::Add);
        let layer_zero = Layer::new(vec![l0_gate1]);
        Circuit::new(vec![layer_two, layer_one, layer_zero])
    }
    #[test]
    fn test_gkr_prover() {
        let mut circuit_example: Circuit<Fq> = get_circuit();
        let inputs = vec![
            Fq::from(2),
            Fq::from(3),
            Fq::from(7),
            Fq::from(10),
            Fq::from(5),
            Fq::from(4),
            Fq::from(3),
            Fq::from(8),
        ];

        let circuit_evaluations = circuit_example.evaluate(&inputs);
        println!("evaluations -> {:?}", circuit_evaluations);
        for i in 0..circuit_evaluations.len() {
            let w_i = Circuit::w_mle(i as u32, &circuit_evaluations);
            println!("layer {} -> {:?}", i, w_i.eval_form);
        }
        assert_eq!("hello gkr", "hello gkr");
    }

    #[test]
    fn test_gkr_verifier() {
        assert_eq!("hello gkr", "hello gkr");
    }
}
