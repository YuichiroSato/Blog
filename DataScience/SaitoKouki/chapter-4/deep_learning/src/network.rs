use crate::calculus::*;
use crate::layer;
use crate::layer::*;
use crate::numpy;
use crate::numpy::*;
use rand::prelude::*;

pub struct Network {
    pub w1: Box<NumpyArray>,
    pub w2: Box<NumpyArray>,
    pub b1: Box<NumpyArray>,
    pub b2: Box<NumpyArray>,
}

impl Network {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize, weight_init_std: f32) -> Network {
        let mut r = rand::thread_rng();

        let a1: Vec<Vec<f32>> = (0..input_size).map(|_| {
            (0..hidden_size).map(|_| {
                let i: f32 = r.gen();
                weight_init_std * (i * 2.0 - 1.0)
            }).collect()
        }).collect();
        let a2: Vec<Vec<f32>> = (0..hidden_size).map(|_| {
            (0..output_size).map(|_| {
                let i: f32 = r.gen();
                weight_init_std * (i * 2.0 - 1.0)
            }).collect()
        }).collect();
        Network {
            w1: Box::new(NumpyArray::from_vec(&a1)),
            w2: Box::new(NumpyArray::from_vec(&a2)),
            b1: Box::new(NumpyArray::row_vec(&vec![0.0; hidden_size])),
            b2: Box::new(NumpyArray::row_vec(&vec![0.0; output_size])),
        }
    }

    pub fn predict(&self, input: &NumpyVector) -> NumpyVector {
        let mut row = input.to_row();
        let mut aa = numpy::dot(&mut row, &self.w1);
        let mut a1 = numpy::add(&mut aa, &self.b1).to_vector();
        let mut z1 = layer::sigmoid(&mut a1).to_row();
        let mut bb = numpy::dot(&mut z1, &self.w2);
        let mut a2 = numpy::add(&mut bb, &self.b2).to_vector();
        layer::softmax(&mut a2);
        a2
    }

    pub fn loss(&self, x: &NumpyVector, t: &NumpyVector) -> f32 {
        let mut y = self.predict(&x);
        cross_entropy_error(&mut y, &t)
    }

    pub fn gradient(&mut self, x: &NumpyVector, t: &NumpyVector) -> Network {
        let mut a1 = self.w1.clone();
        let mut a2 = self.w2.clone();
        let mut a3 = self.b1.clone();
        let mut a4 = self.b2.clone();
        let gw1 = numerical_grad_box(&|w: Box<NumpyArray>| {
            let tmp_network = Network {
                w1: w,
                w2: self.w2.clone(),
                b1: self.b1.clone(),
                b2: self.b2.clone(),
            };
            tmp_network.loss(&x, &t) 
        }, &mut a1);
        let gw2 = numerical_grad_box(&|w: Box<NumpyArray>| {
            let tmp_network = Network {
                w1: self.w1.clone(),
                w2: w,
                b1: self.b1.clone(),
                b2: self.b2.clone(),
            };
            tmp_network.loss(&x, &t)
        }, &mut a2);
        let gb1 = numerical_grad_box(&|b: Box<NumpyArray>| {
            let tmp_network = Network {
                w1: self.w1.clone(),
                w2: self.w2.clone(),
                b1: b,
                b2: self.b2.clone(),
            };
            tmp_network.loss(&x, &t)
        }, &mut a3);
        let gb2 = numerical_grad_box(&|b: Box<NumpyArray>| {
            let tmp_network = Network {
                w1: self.w1.clone(),
                w2: self.w2.clone(),
                b1: self.b1.clone(),
                b2: b,
            };
            tmp_network.loss(&x, &t)
        }, &mut a4);
        Network {
            w1: Box::new(gw1),
            w2: Box::new(gw2),
            b1: Box::new(gb1),
            b2: Box::new(gb2),
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
    let mut network = Network::new(2, 3, 2, 0.01);
    let x = NumpyVector::from_vec(&vec![1.0, 0.2]);
    let t = NumpyVector::from_vec(&vec![0.0, 1.0]);
    let result = network.gradient(&x, &t);
    assert_ne!(result.w1.data, vec![vec![0.0]]);
}
