use rand::Rng;

struct Perceptron {
    weights: [f64; 2],
    bias: f64,
    learning_rate: f64,
}

impl Perceptron {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let weights = [rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)];
        let bias = rng.gen_range(-1.0..1.0);
        let learning_rate = 0.01;

        Perceptron {
            weights,
            bias,
            learning_rate,
        }
    }

    fn activate(&self, sum: f64) -> i32 {
        if sum > 0.0 {
            1
        } else {
            0
        }
    }

    fn feed_forward(&self, inputs: &[i32; 2]) -> i32 {
        let weighted_sum = (inputs[0] as f64 * self.weights[0])
            + (inputs[1] as f64 * self.weights[1])
            + self.bias;
        self.activate(weighted_sum)
    }

    fn train(&mut self, inputs: &[i32; 2], target: i32) {
        let guess = self.feed_forward(inputs);
        let error = target - guess;
        for i in 0..2 {
            self.weights[i] += self.learning_rate * (error as f64) * (inputs[i] as f64);
        }
        self.bias += self.learning_rate * (error as f64);
    }
}

fn main() {
    // Perceptrón para la compuerta lógica AND
    let mut and_perceptron = Perceptron::new();
    let and_training_data = [
        ([0, 0], 0),
        ([0, 1], 0),
        ([1, 0], 0),
        ([1, 1], 1),
    ];

    // Entrenamiento para la compuerta lógica AND
    for _ in 0..10000 {
        for &(inputs, target) in &and_training_data {
            and_perceptron.train(&inputs, target);
        }
    }

    // Pruebas para AND
    for &(inputs, _) in &and_training_data {
        let result = and_perceptron.feed_forward(&inputs);
        println!("AND {:?} -> {}", inputs, result);
    }
}
