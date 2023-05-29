use rand::{thread_rng, Rng};

pub struct NeuralNetwork {
    input_layer: Vec<f64>,
    hidden_layers: Vec<Layer>,
    output_layer: Layer,
}

pub struct Layer {
    neurons: Vec<f64>,
    biases: Vec<f64>,
    weights: Vec<Vec<f64>>,
}

impl NeuralNetwork {
    pub fn new(num_layers: Vec<usize>) -> Option<Self> {
        if num_layers.len() < 3 {
            return None;
        }

        let input_layer = vec![0.0; *num_layers.get(0).unwrap()];
        let mut hidden_layers = Vec::new();
        for i in 0..num_layers.len() - 2 {
            hidden_layers.push(Layer::new(
                *num_layers.get(i + 1).unwrap(),
                *num_layers.get(i).unwrap(),
            ));
        }

        let output_layer = Layer::new(
            *num_layers.get(num_layers.len() - 1).unwrap(),
            *num_layers.get(num_layers.len() - 2).unwrap(),
        );

        Some(Self {
            input_layer,
            hidden_layers,
            output_layer,
        })
    }
}

impl Layer {
    pub fn new(neuron_count: usize, prev_nueron_count: usize) -> Self {
        let neurons = vec![0.0; neuron_count];
        let biases = vec![0.0; neuron_count];

        let mut weights = Vec::new();
        for _ in 0..neuron_count {
            let mut neuron_weights = Vec::new();
            for _ in 0..prev_nueron_count {
                neuron_weights.push(thread_rng().gen_range(-1.0..1.0));
            }
            weights.push(neuron_weights);
        }

        Self {
            neurons,
            biases,
            weights,
        }
    }
}
