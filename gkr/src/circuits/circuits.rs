use crate::circuits::gates::{Gate, GateOperation};
use crate::circuits::layers::Layer;
use ark_bn254::Fq;
use ark_ff::PrimeField;
use multilinear::evaluation_form::EvaluationForm;

pub struct Circuit<F: PrimeField> {
    pub layers: Vec<Layer>,
    pub layer_evals: Vec<Vec<F>>,
}

impl<F: PrimeField> Circuit<F> {
    pub fn new(layers: Vec<Layer>) -> Self {
        Circuit {
            layers,
            layer_evals: Vec::new(),
        }
    }

    pub fn evaluate(&mut self, inputs: &Vec<F>) -> Vec<Vec<F>> {
        let mut layer_evaluations: Vec<Vec<F>> = Vec::with_capacity(self.layers.len() + 1);
        // pushes the input as the first array element
        layer_evaluations.push(inputs.to_vec());

        // we iterate through each layer
        for layer in &self.layers {
            // we take the last added element in the array and pass in as input into the layer evaluation fn
            let next_output = layer.evaluate(layer_evaluations.last().unwrap());
            layer_evaluations.push(next_output);
        }
        self.layer_evals = layer_evaluations.iter().rev().cloned().collect();
        layer_evaluations
    }

    // layer evaluation polynomial for layer_i
    // the top most layer is layer 0
    pub fn w_mle(&self, layer_index: u32) -> EvaluationForm<F> {
        let length = self.layer_evals.len() as u32;
        if length <= layer_index || length == 0 {
            panic!("Compute circuit first!");
        }
        EvaluationForm::new(self.layer_evals[layer_index as usize].clone())
    }

    pub fn add_and_mul_i(&self, layer_index: u32) -> (EvaluationForm<F>, EvaluationForm<F>) {
        // the top layer is indexed at i = 0
        // preceding layers are i + 1
        let diff = (self.layers.len() as u32) - layer_index - 1;

        let layer = &self.layers[diff as usize];
        let no_of_gates = layer.gates.len() as u32;

        let mut add_eval_form: Vec<F> = vec![F::zero(); 2u32.pow(no_of_gates * 2) as usize];
        let mut mul_eval_form: Vec<F> = vec![F::zero(); 2u32.pow(no_of_gates * 2) as usize];

        for gate in &layer.gates {
            if gate.op == GateOperation::Add {
                let index = combine_convert(
                    vec![
                        gate.output as u32,
                        gate.left_index as u32,
                        gate.right_index as u32,
                    ],
                    to_log2(no_of_gates * 2) as usize,
                ) as usize;
                add_eval_form[index] = F::one();
            } else if gate.op == GateOperation::Mul {
                let index = combine_convert(
                    vec![
                        gate.output as u32,
                        gate.left_index as u32,
                        gate.right_index as u32,
                    ],
                    to_log2(no_of_gates * 2) as usize,
                ) as usize;
                mul_eval_form[index] = F::one();
            }
        }
        (
            EvaluationForm::new(add_eval_form),
            EvaluationForm::new(mul_eval_form),
        )
    }

    pub fn mul_i(&self, layer_index: u32) -> EvaluationForm<F> {
        // the top layer is indexed at i = 0
        // preceding layers are i + 1
        let diff = (self.layers.len() as u32) - layer_index - 1;

        let layer = &self.layers[diff as usize];
        let no_of_gates = layer.gates.len() as u32;
        let mut eval_form: Vec<F> = vec![F::zero(); 2u32.pow(no_of_gates * 2) as usize];

        for gate in &layer.gates {
            if gate.op == GateOperation::Mul {
                let index = combine_convert(
                    vec![
                        gate.output as u32,
                        gate.left_index as u32,
                        gate.right_index as u32,
                    ],
                    to_log2(no_of_gates * 2) as usize,
                ) as usize;
                eval_form[index] = F::one();
            }
        }
        EvaluationForm::new(eval_form)
    }
}

pub fn combine_convert(values: Vec<u32>, digit: usize) -> u32 {
    let binary_string: String = values
        .iter()
        .map(|&x| format!("{:0digit$b}", x, digit = digit))
        .collect();
    u32::from_str_radix(&binary_string, 2).unwrap()
}

pub fn to_log2(n: u32) -> u32 {
    if n == 0 {
        panic!("log2 is undefined for zero");
    }
    31 - n.leading_zeros()
}

pub fn convert_to_fq_elements(values: Vec<u32>) -> Vec<Fq> {
    values.into_iter().map(|x| Fq::from(x)).collect()
}

#[cfg(test)]

mod test {
    use std::vec;

    use super::*;

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
    fn test_circuit_creation() {
        let circuit_example: Circuit<Fq> = get_circuit();

        assert_eq!(circuit_example.layers.len(), 3);
    }

    #[test]
    fn test_circuit_evaluation() {
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

        let circuit_evaluation = circuit_example.evaluate(&inputs);
        assert_eq!(
            circuit_evaluation,
            vec![
                vec![
                    Fq::from(2),
                    Fq::from(3),
                    Fq::from(7),
                    Fq::from(10),
                    Fq::from(5),
                    Fq::from(4),
                    Fq::from(3),
                    Fq::from(8),
                ],
                vec![Fq::from(6), Fq::from(17), Fq::from(20), Fq::from(11)],
                vec![Fq::from(23), Fq::from(220)],
                vec![Fq::from(5060)]
            ]
        )
    }

    #[test]
    fn test_w_mle() {
        let mut circuit_example: Circuit<Fq> = get_circuit2();
        let inputs = vec![
            Fq::from(1),
            Fq::from(2),
            Fq::from(3),
            Fq::from(4),
            Fq::from(5),
            Fq::from(6),
            Fq::from(7),
            Fq::from(8),
        ];
        circuit_example.evaluate(&inputs);
        assert_eq!(circuit_example.w_mle(0).eval_form, vec![Fq::from(88)]);
        assert_eq!(
            circuit_example.w_mle(1).eval_form,
            vec![Fq::from(21), Fq::from(67)]
        );
        assert_eq!(
            circuit_example.w_mle(2).eval_form,
            vec![Fq::from(3), Fq::from(7), Fq::from(11), Fq::from(56)]
        );
        assert_eq!(
            circuit_example.w_mle(3).eval_form,
            vec![
                Fq::from(1),
                Fq::from(2),
                Fq::from(3),
                Fq::from(4),
                Fq::from(5),
                Fq::from(6),
                Fq::from(7),
                Fq::from(8),
            ]
        );

        assert_eq!(
            circuit_example.w_mle(1).evaluate(&vec![Fq::from(0)]),
            Fq::from(21)
        );
    }

    #[test]
    fn test_combine_convert() {
        assert_eq!(combine_convert(vec![1, 2, 3], 3), 83);
    }

    #[test]
    fn test_add_and_mul_i() {
        let circuit_example: Circuit<Fq> = get_circuit3();
        let layer_index = 2;
        // let inputs:Vec<u32> = vec![0, 0, 1];
        let (add_i_poly, mul_i_poly) = circuit_example.add_and_mul_i(layer_index);
        assert_eq!(
            add_i_poly
                .clone()
                .evaluate(&convert_to_fq_elements(vec![0, 0, 0, 0, 0, 0, 0, 1])),
            Fq::from(1)
        );
        assert_eq!(
            add_i_poly
                .clone()
                .evaluate(&convert_to_fq_elements(vec![0, 1, 0, 1, 0, 0, 1, 1])),
            Fq::from(1)
        );
        assert_eq!(
            add_i_poly
                .clone()
                .evaluate(&convert_to_fq_elements(vec![1, 0, 1, 0, 0, 1, 0, 1])),
            Fq::from(1)
        );
        assert_eq!(
            add_i_poly
                .clone()
                .evaluate(&convert_to_fq_elements(vec![1, 1, 0, 1, 1, 0, 0, 1])),
            Fq::from(0)
        );
        assert_eq!(
            mul_i_poly
                .clone()
                .evaluate(&convert_to_fq_elements(vec![1, 1, 1, 1, 0, 1, 1, 1])),
            Fq::from(1)
        );
        assert_eq!(
            mul_i_poly
                .clone()
                .evaluate(&convert_to_fq_elements(vec![0, 1, 1, 1, 0, 1, 1, 1])),
            Fq::from(0)
        );
        assert_eq!(
            mul_i_poly
                .clone()
                .evaluate(&convert_to_fq_elements(vec![1, 1, 1, 1, 1, 1, 1, 1])),
            Fq::from(0)
        );
    }
}
