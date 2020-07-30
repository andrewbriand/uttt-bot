pub struct Layer {
    size: usize,
    input_size: usize,
    weights: Vec<f32>,
    biases: Vec<f32>,
}

impl Layer {
    fn new(_size: usize, _input_size: usize, _weights: Vec<f32>, _biases: Vec<f32>) -> Layer {
        debug_assert!(_weights.len() == _input_size * _size);
        debug_assert!(_biases.len() == _size);
        Layer {
            size: _size,
            input_size: _input_size,
            weights: _weights,
            biases: _biases,
        }
    }

    fn vec_dot(v1: &[f32], v2: &[f32]) -> f32 {
        debug_assert!(v1.len() == v2.len());
        v1.iter().zip(v2).map(|(i1, i2)| i1 * i2).sum()
    }

    fn eval(&self, input: &Vec<f32>) -> Vec<f32> {
        debug_assert!(input.len() == self.input_size);
        let mut result = Vec::with_capacity(self.size);
        // For every neuron:
        // multiply the input by the weights of that neuron (dot product)
        for i in 0..self.size {
            result.push(Layer::vec_dot(&input[..], &self.weights[(i*self.size)..(i*(self.size + 1))]));
        }

        // add the biases
        result.iter().zip(self.biases.as_slice()).map(|(i1, i2)| i1 + i2).collect()
    }
}

pub struct NeuralNet {
    layers: Vec<Layer>,
}

impl NeuralNet {
    fn new(_layers: Vec<Layer>) -> NeuralNet {
        NeuralNet {
            layers: _layers,
        }
    }

    fn infer(&self, input: &Vec<f32>) -> Vec<f32> {
        let mut curr_vec = self.layers[0].eval(input);
        for i in 1..self.layers.len() {
            curr_vec = self.layers[i].eval(&curr_vec);
        }
        return curr_vec;
    }
}