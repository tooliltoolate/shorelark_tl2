use std::process::Output;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        for layer in &self.layers {
            inputs = layer.propagate(inputs)
        }
        inputs
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        let outputs = Vec::new();
        for neuron in &self.neurons {
            let output = neuron.propagate(&inputs);
            outputs.push(output);
        }
        outputs
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    output_weights: Vec<f32>,
}

impl Neuron {
    pub fn propagate(&self, inputs: Vec<f32>) -> f32 {
        todo!()
    }
}
