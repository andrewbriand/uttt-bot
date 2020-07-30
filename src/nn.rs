pub struct Layer {
    size: usize,
    input_size: usize,
    weights: Vec<f32>,
    biases: Vec<f32>,
}

impl Layer {
    fn new(_size: usize, _input_size: usize, _weights: Vec<f32>, _biases: Vec<f32>) -> Layer {
        debug_assert!(_weights.len() == input_size * size);
        debug_assert!(_biases.len() == size);
        Layer {
            size: _size,
            input_size: _input_size,
            weights: _weights,
            biases: _biases,
        }
    }

    fn vec_dot(v1: &[f32], v2: &[f32]) -> f32 {
        debug_assert!(v1.len() == v2.len());
        v1.iter().zip(v2).map(|(&i1, &i2)| i1 * i2).sum()
    }

    fn eval(&self, input: &Vec<f32>) -> Vec<f32> {
        let result = Vec::with_capacity(self.size);
        // For every neuron:
        // multiply the input by the weights of that neuron (dot product)
        for i in 0..size {
            result.push(Layer::vec_dot(&input[..], &self.weights[(i*size)..(i*(size + 1))]));
        }

        // add the bias
        result.iter().zip(biases).map(|(&i1, &i2)| i1 + i2).collect();
    }
}

pub struct NeuralNet {
    layers: Vec<Layer>,
}

impl NeuralNet {
    fn new(layers: Vec<Layer>) {

    }
}