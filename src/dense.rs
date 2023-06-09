use ndarray::{Array1, Array2};
use crate::block::Block;
use rand_distr::{Distribution, Normal};
use log::info;

// Defines struct for storing dense parameters
pub struct DenseParams {
    weights: Vec<Array2::<f32>>,
    biases: Vec<Array1::<f32>>,
}

// Defines dense layer struct
pub struct Dense {
    input: Array1::<f32>,
    pub input_size: usize,
    linear: bool,
    layer: Vec<Array1::<f32>>,
    _error: Vec<Array1::<f32>>,
    params: DenseParams,
}

impl Dense {
    /// Create a new self-attention block with the given parameters
    pub fn new(layer_sizes: Array1<usize>, linear: bool) -> Dense {
        let input = Array1::<f32>::zeros(layer_sizes[0]);
        let mut layer = vec![];
        let mut error = vec![];
        let mut weights = vec![];
        let mut biases = vec![Array1::<f32>::zeros(0)];

        for i in 0..layer_sizes.len()-1 {
            let normal = Normal::new(0.0, (2.0 / layer_sizes[i] as f32).sqrt()).unwrap();
            let mut layer_weights = Array2::<f32>::zeros((layer_sizes[i],layer_sizes[i+1]));
            let mut layer_biases = Array1::<f32>::zeros(layer_sizes[i+1]);

            // Use He initialisation by using a mean of 0.0 and a standard deviation of sqrt(2/n)
            layer_weights.mapv_inplace(|_| normal.sample(&mut rand::thread_rng()));
            layer_biases.mapv_inplace(|_| normal.sample(&mut rand::thread_rng()));

            weights.push(layer_weights);
            biases.push(layer_biases);
            layer.push(Array1::<f32>::zeros(layer_sizes[i]));
            error.push(Array1::<f32>::zeros(layer_sizes[i]));
        }

        layer.push(Array1::<f32>::zeros(layer_sizes[layer_sizes.len()-1]));
        error.push(Array1::<f32>::zeros(layer_sizes[layer_sizes.len()-1]));

        let params = DenseParams { weights, biases };

        let block: Dense = Dense {
            input,
            input_size: layer_sizes[0],
            linear,
            layer,
            _error: error,
            params
        };

        block
    }
}

impl Block for Dense {
    type Input = Array1<f32>;
    type Output = Array1<f32>;

    fn set_block(&mut self, value: Self::Input) {
        self.input = value;
    }

    fn forward_propagate(&mut self) -> Self::Output {
        info!("Dense block input: \n {:?}", self.input);

        self.layer[0].assign(&self.input);
        for i in 1..self.layer.len() {
            let weighted_sum = &self.layer[i - 1].dot(&self.params.weights[i - 1]);
            self.layer[i] = weighted_sum + &self.params.biases[i];
            if !self.linear {
                self.layer[i].mapv_inplace(|x| if x > 0.0 { x } else { 0.0 });
            }
        }

        info!("Dense block output: \n {:?}", self.layer[self.layer.len()-1]);

        self.layer[self.layer.len()-1].clone()
    }
}