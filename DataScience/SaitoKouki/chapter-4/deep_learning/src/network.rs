use crate::calculus::*;
use crate::layer;
use crate::layer::*;
use crate::numpy;
use crate::numpy::*;
use rand::prelude::*;

pub struct Network {
    pub w1: NumpyArray,
    pub w2: NumpyArray,
    pub b1: NumpyArray,
    pub b2: NumpyArray,
}

impl Network {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize, weight_init_std: f32) -> Self {
        let mut r = rand::thread_rng();

        let a1: Vec<Vec<f32>> = (0..input_size).map(|_| {
            (0..hidden_size).map(|_| {
                r.gen()
            }).collect()
        }).collect();
        let a2: Vec<Vec<f32>> = (0..hidden_size).map(|_| {
            (0..output_size).map(|_| {
                r.gen()
            }).collect()
        }).collect();
        Network {
            w1: NumpyArray::from_vec(&a1).muls(2.0).adds(-1.0).muls(weight_init_std),
            w2: NumpyArray::from_vec(&a2).muls(2.0).adds(-1.0).muls(weight_init_std),
            b1: NumpyArray::row_vec(&vec![0.0; hidden_size]),
            b2: NumpyArray::row_vec(&vec![0.0; output_size]),
        }
    }

    pub fn predict(&self, input: &NumpyVector) -> NumpyVector {
        let a1 = numpy::add(&numpy::dot(&input.to_row(), &self.w1), &self.b1);
        let z1 = layer::sigmoid(&a1.to_vector());
        let a2 = numpy::add(&numpy::dot(&z1.to_row(), &self.w2), &self.b2);
        layer::softmax(&a2.to_vector())
    }

    pub fn loss(&self, x: &NumpyVector, t: &NumpyVector) -> f32 {
        let y = self.predict(&x);
        cross_entropy_error(&y, &t)
    }

    pub fn gradient(&self, x: &NumpyVector, t: &NumpyVector) -> Network {
        let gw1 = numerical_grad_array(&|w| {
            let tmp_network = Network {
                w1: w,
                w2: self.w2.clone(),
                b1: self.b1.clone(),
                b2: self.b2.clone(),
            };
            tmp_network.loss(&x, &t) 
        }, &self.w1);
        let gw2 = numerical_grad_array(&|w| {
            let tmp_network = Network {
                w1: self.w1.clone(),
                w2: w,
                b1: self.b1.clone(),
                b2: self.b2.clone(),
            };
            tmp_network.loss(&x, &t)
        }, &self.w2);
        let gb1 = numerical_grad_array(&|b| {
            let tmp_network = Network {
                w1: self.w1.clone(),
                w2: self.w2.clone(),
                b1: b,
                b2: self.b2.clone(),
            };
            tmp_network.loss(&x, &t)
        }, &self.b1);
        let gb2 = numerical_grad_array(&|b| {
            let tmp_network = Network {
                w1: self.w1.clone(),
                w2: self.w2.clone(),
                b1: self.b1.clone(),
                b2: b,
            };
            tmp_network.loss(&x, &t)
        }, &self.b2);
        Network {
            w1: gw1,
            w2: gw2,
            b1: gb1,
            b2: gb2,
        }
    }
}

#[test]
fn predict_test() {
    let network = Network::new(2, 3, 2, 0.01);
    let result = network.predict(&NumpyVector::from_vec(&vec![0.0, 0.0]));
    assert_eq!(result.data.len(), 2);
}

#[test]
fn gradient_test() {
    let network = Network::new(2, 3, 2, 0.01);
    let x = NumpyVector::from_vec(&vec![1.0, 0.2]);
    let t = NumpyVector::from_vec(&vec![0.0, 1.0]);
    let result = network.gradient(&x, &t);
    assert_ne!(result.w1.data, vec![vec![0.0]]);
}
