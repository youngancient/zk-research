use crate::circuits::gates::{Gate, GateOperation};
use crate::circuits::layers::Layer;
use ark_ff::PrimeField;
use multilinear::evaluation_form::EvaluationForm;

pub struct Circuit {
    pub layers: Vec<Layer>,
}

impl Circuit {
    pub fn new(layers: Vec<Layer>) -> Self {
        Circuit { layers }
    }

    pub fn evaluate<F: PrimeField>(&self, inputs: Vec<F>) -> Vec<Vec<F>> {
        let mut layer_evaluations: Vec<Vec<F>> = Vec::with_capacity(self.layers.len() + 1);
        // pushes the input as the first array element
        layer_evaluations.push(inputs);

        // we iterate through each layer
        for layer in &self.layers {
            // we take the last added element in the array and pass in as input into the layer evaluation fn
            let next_output = layer.evaluate(layer_evaluations.last().unwrap());
            layer_evaluations.push(next_output);
        }
        layer_evaluations
    }

    pub fn add_i<F: PrimeField>(&self, layer_index: u32) -> EvaluationForm<F> {
        // the top layer is indexed at i = 0
        // preceding layers are i + 1
        if layer_index + 1 >= self.layers.len() as u32 {
            panic!("Layer {} has no add_i",layer_index);
        }
        let layer = &self.layers[layer_index as usize];
        let prev_layer = &self.layers[(layer_index + 1) as usize];
        println!("layer -> {:?}", layer);
        println!("=========Gate===========");
        for gate in &layer.gates{
            println!("gate => {:?}",gate);
        }
        println!("====================");
        println!("prev layer -> {:?}", prev_layer);
        todo!()
    }

    pub fn mul_i<F: PrimeField>(layer_index: u32) -> EvaluationForm<F> {
        todo!()
    }
}

#[cfg(test)]

mod test {
    use std::vec;

    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_circuit_creation() {
        let l1_gate1 = Gate::new(0, 1, 0, GateOperation::Mul);
        let l1_gate2 = Gate::new(2, 3, 1, GateOperation::Add);
        let l1_gate3 = Gate::new(4, 5, 2, GateOperation::Mul);
        let l1_gate4 = Gate::new(6, 7, 3, GateOperation::Add);
        let layer_one = Layer::new(vec![l1_gate1, l1_gate2, l1_gate3, l1_gate4]);

        let l2_gate1 = Gate::new(0, 1, 0, GateOperation::Add);
        let l2_gate2 = Gate::new(2, 3, 1, GateOperation::Mul);
        let layer_two = Layer::new(vec![l2_gate1, l2_gate2]);

        let l3_gate1 = Gate::new(0, 1, 0, GateOperation::Mul);
        let layer_three = Layer::new(vec![l3_gate1]);

        let circuit_example = Circuit::new(vec![layer_one, layer_two, layer_three]);
        assert_eq!(circuit_example.layers.len(), 3);
    }

    #[test]
    fn test_circuit_evaluation() {
        let l1_gate1 = Gate::new(0, 1, 0, GateOperation::Mul);
        let l1_gate2 = Gate::new(2, 3, 1, GateOperation::Add);
        let l1_gate3 = Gate::new(4, 5, 2, GateOperation::Mul);
        let l1_gate4 = Gate::new(6, 7, 3, GateOperation::Add);
        let layer_one = Layer::new(vec![l1_gate1, l1_gate2, l1_gate3, l1_gate4]);

        let l2_gate1 = Gate::new(0, 1, 0, GateOperation::Add);
        let l2_gate2 = Gate::new(2, 3, 1, GateOperation::Mul);
        let layer_two = Layer::new(vec![l2_gate1, l2_gate2]);

        let l3_gate1 = Gate::new(0, 1, 0, GateOperation::Mul);
        let layer_three = Layer::new(vec![l3_gate1]);

        let circuit_example = Circuit::new(vec![layer_one, layer_two, layer_three]);

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

        let circuit_evaluation = circuit_example.evaluate(inputs);
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
    fn test_add_i() {
        let l1_gate1 = Gate::new(0, 1, 0, GateOperation::Mul);
        let l1_gate2 = Gate::new(2, 3, 1, GateOperation::Add);
        let l1_gate3 = Gate::new(4, 5, 2, GateOperation::Mul);
        let l1_gate4 = Gate::new(6, 7, 3, GateOperation::Add);
        let layer_one = Layer::new(vec![l1_gate1, l1_gate2, l1_gate3, l1_gate4]);

        let l2_gate1 = Gate::new(0, 1, 0, GateOperation::Add);
        let l2_gate2 = Gate::new(2, 3, 1, GateOperation::Mul);
        let layer_two = Layer::new(vec![l2_gate1, l2_gate2]);

        let l3_gate1 = Gate::new(0, 1, 0, GateOperation::Mul);
        let layer_three = Layer::new(vec![l3_gate1]);

        let circuit_example = Circuit::new(vec![layer_one, layer_two, layer_three]);

        let layer_index = 0;
        // let inputs:Vec<u32> = vec![0, 0, 1];
        circuit_example.add_i::<Fq>(layer_index);
    }
}
