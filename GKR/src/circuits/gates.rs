use ark_ff::PrimeField;
pub enum GateOperation {
    Add,
    Mul,
}

pub struct Gate {
    pub left_index: usize,
    pub right_index: usize,
    pub output: usize,
    pub op: GateOperation,
}

impl Gate {
    pub fn new(left: usize, right: usize, out: usize, op: GateOperation) -> Self {
        Gate {
            left_index: left,
            right_index: right,
            output: out,
            op,
        }
    }

    pub fn evaluate<F: PrimeField>(&self, inputs: &Vec<F>, outputs: &mut Vec<F>) {
        let left_val = inputs[self.left_index];
        let right_val = inputs[self.right_index];

        let output = match self.op {
            GateOperation::Add => left_val + right_val,
            GateOperation::Mul => left_val * right_val,
        };
        outputs[self.output] = output;
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_gate_creation() {
        let gate = Gate::new(1, 2, 3, GateOperation::Add);
        assert_eq!(gate.left_index, 1);
        assert_eq!(gate.right_index, 2);
        assert_eq!(gate.output, 3);
    }

    #[test]
    fn test_gate_evaluation() {
        let inputs = vec![Fq::from(0), Fq::from(2), Fq::from(4), Fq::from(6)];
        let mut outputs: Vec<Fq> = vec![Fq::from(0); 2];

        let gate = Gate::new(0, 1, 0, GateOperation::Add);
        gate.evaluate(&inputs, &mut outputs);
        assert_eq!(outputs[0], Fq::from(2));

        let gate2 = Gate::new(2, 3, 1, GateOperation::Mul);
        gate2.evaluate(&inputs, &mut outputs);
        assert_eq!(outputs[1], Fq::from(24));
    }
}
