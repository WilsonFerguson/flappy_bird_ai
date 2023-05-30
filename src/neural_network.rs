use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct NeuralNetwork {
    hidden_layers: Vec<Layer>,
    output_layer: Layer,
}

#[derive(Clone)]
pub struct Layer {
    pub biases: Vec<f64>,
    pub weights: Vec<Vec<f64>>,
}

impl NeuralNetwork {
    pub fn new(num_layers: &[usize]) -> Self {
        if num_layers.len() < 3 {
            panic!("Neural network must have at least 3 layers");
        }

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

        Self {
            hidden_layers,
            output_layer,
        }
    }

    pub fn feed_forward(&self, inputs: &Vec<f64>) -> Vec<f64> {
        let mut outputs = inputs.clone();
        for layer in &self.hidden_layers {
            outputs = layer.feed_forward(&outputs);
        }
        outputs = self.output_layer.feed_forward(&outputs);

        return outputs;
    }

    pub fn mutate(&mut self, mutation_rate: f64) {
        self.hidden_layers.iter_mut().for_each(|layer| {
            layer.mutate(mutation_rate);
        });
        self.output_layer.mutate(mutation_rate);
    }

    pub fn crossover(&self, other: &NeuralNetwork) -> NeuralNetwork {
        let hidden_layers: Vec<Layer> = self
            .hidden_layers
            .iter()
            .zip(other.hidden_layers.iter())
            .map(|(a, b)| a.crossover(b))
            .collect();
        let output_layer = self.output_layer.crossover(&other.output_layer);

        NeuralNetwork {
            hidden_layers,
            output_layer,
        }
    }
}

impl Layer {
    pub fn new(neuron_count: usize, prev_nueron_count: usize) -> Self {
        // let biases = vec![0.0; neuron_count]; // java version did this I think
        // Randomize the biases.
        let mut biases = Vec::new();
        for _ in 0..neuron_count {
            biases.push(thread_rng().gen_range(-1.0..1.0));
        }

        let mut weights = Vec::new();
        for _ in 0..neuron_count {
            let mut neuron_weights = Vec::new();
            for _ in 0..prev_nueron_count {
                neuron_weights.push(thread_rng().gen_range(-1.0..1.0));
            }
            weights.push(neuron_weights);
        }

        Self { biases, weights }
    }

    pub fn feed_forward(&self, inputs: &Vec<f64>) -> Vec<f64> {
        (0..self.biases.len())
            .map(|i| {
                let mut sum = 0.0;
                for j in 0..inputs.len() {
                    sum += inputs[j] * self.weights[i][j];
                }
                sum += self.biases[i];
                self.sigmoid(sum)
            })
            .collect()
    }

    fn sigmoid(&self, x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    pub fn mutate(&mut self, mutation_rate: f64) {
        self.biases
            .iter_mut()
            .filter(|_| thread_rng().gen_range(0.0..1.0) < mutation_rate)
            .for_each(|b| *b = thread_rng().gen_range(-1.0..1.0));
    }

    pub fn crossover(&self, other: &Layer) -> Layer {
        let mut child: Layer = Layer::new(self.biases.len(), self.weights.len()); // Note: in java I
                                                                                  // did weights[0].length
        child.biases.iter_mut().enumerate().for_each(|(i, b)| {
            *b = if thread_rng().gen_range(0.0..1.0) < 0.5 {
                self.biases[i]
            } else {
                other.biases[i]
            }
        });

        child.weights.iter_mut().enumerate().for_each(|(i, w)| {
            w.iter_mut().enumerate().for_each(|(j, n)| {
                *n = if thread_rng().gen_range(0.0..1.0) < 0.5 {
                    self.weights[i][j]
                } else {
                    other.weights[i][j]
                }
            })
        });

        child
    }
}
