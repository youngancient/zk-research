use crate::circuits::gates::{Gate, GateOperation};
use ark_ff::PrimeField;

pub struct Layer {
    pub gates: Vec<Gate>,
}

impl Layer {
    pub fn new(gates: Vec<Gate>) -> Self {
        Layer { gates }
    }

    pub fn evaluate<F: PrimeField>(&self, inputs: &Vec<F>) -> Vec<F> {
        let mut outputs = vec![F::zero(); self.gates.len()];
        for gate in &self.gates {
            gate.evaluate(inputs, &mut outputs);
        }
        outputs
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_layer_creation() {
        let gate = Gate::new(0, 1, 0, GateOperation::Add);
        let gate2 = Gate::new(2, 3, 1, GateOperation::Mul);
        let layer_one = Layer::new(vec![gate, gate2]);
        assert_eq!(layer_one.gates.len(), 2);
    }

    #[test]
    fn test_layer_evaluation() {
        let gate = Gate::new(0, 1, 0, GateOperation::Add);
        let gate2 = Gate::new(2, 3, 1, GateOperation::Mul);
        let layer_one = Layer::new(vec![gate, gate2]);
        let inputs = vec![Fq::from(0), Fq::from(2), Fq::from(4), Fq::from(6)];
        let output = layer_one.evaluate::<Fq>(&inputs);
        assert_eq!(output.len(), 2);
        assert_eq!(output, vec![Fq::from(2), Fq::from(24)]);
    }

    #[test]
    fn test_layer_evaluation2() {
        let gate = Gate::new(0, 1, 0, GateOperation::Mul);
        let gate2 = Gate::new(2, 3, 1, GateOperation::Add);
        let gate3 = Gate::new(4, 5, 2, GateOperation::Mul);
        let gate4 = Gate::new(6, 7, 3, GateOperation::Add);
        let layer_one = Layer::new(vec![gate, gate2, gate3, gate4]);
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
        let output = layer_one.evaluate::<Fq>(&inputs);
        assert_eq!(output.len(), 4);
        assert_eq!(
            output,
            vec![Fq::from(6), Fq::from(17), Fq::from(20), Fq::from(11)]
        );
    }
}
