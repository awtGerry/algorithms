use rand::Rng;

struct NeuralNetwork {
    input_nodes: usize,
    hidden_nodes: usize,
    output_nodes: usize,
    learning_rate: f64,
    weights_ih: Vec<Vec<f64>>,
    weights_ho: Vec<f64>,
    bias_h: Vec<f64>,
    bias_o: f64,
}

impl NeuralNetwork {
    fn new(input_nodes: usize, hidden_nodes: usize, output_nodes: usize, learning_rate: f64) -> Self {
        let mut rng = rand::thread_rng();

        let weights_ih: Vec<Vec<f64>> = (0..hidden_nodes)
            .map(|_| (0..input_nodes)
                .map(|_| rng.gen_range(-1.0..1.0))
                .collect())
            .collect();

        let weights_ho: Vec<f64> = (0..output_nodes)
            .map(|_| rng.gen_range(-1.0..1.0))
            .collect();

        let bias_h: Vec<f64> = (0..hidden_nodes)
            .map(|_| rng.gen_range(-1.0..1.0))
            .collect();

        let bias_o = rng.gen_range(-1.0..1.0);

        NeuralNetwork {
            input_nodes,
            hidden_nodes,
            output_nodes,
            learning_rate,
            weights_ih,
            weights_ho,
            bias_h,
            bias_o,
        }
    }

    fn sigmoid(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    fn sigmoid_derivative(x: f64) -> f64 {
        x * (1.0 - x)
    }

    fn feed_forward(&self, input: &[f64; 2]) -> f64 {
        // Capa de entrada a capa oculta
        let mut hidden = vec![0.0; self.hidden_nodes];
        for i in 0..self.hidden_nodes {
            hidden[i] = self.bias_h[i];
            for j in 0..self.input_nodes {
                hidden[i] += input[j] * self.weights_ih[i][j];
            }
            hidden[i] = NeuralNetwork::sigmoid(hidden[i]);
        }

        // Capa oculta a capa de salida
        let mut output = 0.0;
        for i in 0..self.output_nodes {
            output += hidden[i] * self.weights_ho[i];
        }

        NeuralNetwork::sigmoid(output)
    }

    fn train(&mut self, input: &[f64; 2], target: f64) {
        // Paso hacia adelante
        // Capa de entrada a capa oculta
        let mut hidden = vec![0.0; self.hidden_nodes];
        for i in 0..self.hidden_nodes {
            hidden[i] = self.bias_h[i];
            for j in 0..self.input_nodes {
                hidden[i] += input[j] * self.weights_ih[i][j];
            }
            hidden[i] = NeuralNetwork::sigmoid(hidden[i]);
        }

        // Capa oculta a capa de salida
        let mut output = 0.0;
        for i in 0..self.output_nodes {
            output += hidden[i] * self.weights_ho[i];
        }

        let predicted = NeuralNetwork::sigmoid(output);

        // Calcular el error
        let error = target - predicted;

        // Paso hacia atrás (retropropagación)
        // Capa de salida a capa oculta
        let output_gradient = error * NeuralNetwork::sigmoid_derivative(predicted);
        for i in 0..self.output_nodes {
            self.bias_o += self.learning_rate * output_gradient;
            for j in 0..self.hidden_nodes {
                self.weights_ho[i] += self.learning_rate * output_gradient * hidden[j];
            }
        }

        // Capa oculta a capa de entrada
        let mut hidden_gradient = vec![0.0; self.hidden_nodes];
        for i in 0..self.hidden_nodes {
            hidden_gradient[i] = 0.0;
            for j in 0..self.output_nodes {
                hidden_gradient[i] += output_gradient * self.weights_ho[j];
            }
            hidden_gradient[i] *= NeuralNetwork::sigmoid_derivative(hidden[i]);
            for j in 0..self.input_nodes {
                self.weights_ih[i][j] += self.learning_rate * hidden_gradient[i] * input[j];
            }
            self.bias_h[i] += self.learning_rate * hidden_gradient[i];
        }
    }
}

fn main() {
    let mut nn = NeuralNetwork::new(2, 2, 1, 0.1);
    let training_data = [
        ([0.0, 0.0], 0.0),
        ([0.0, 1.0], 1.0),
        ([1.0, 0.0], 1.0),
        ([1.0, 1.0], 0.0),
    ];

    for _ in 0..10000 {
        for &(inputs, target) in &training_data {
            nn.train(&inputs, target);
        }
    }

    for &(inputs, _) in &training_data {
        let result = nn.feed_forward(&inputs);
        println!("{:?} -> {:.4}", inputs, result);
    }
}
