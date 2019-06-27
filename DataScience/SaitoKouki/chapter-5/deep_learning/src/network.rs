use crate::calculus::*;
use crate::layer;
use crate::layer::*;
use crate::numpy;
use crate::numpy::*;
use rand::prelude::*;

pub struct Network {
    pub layer1: Box<AffineLayer>,
    pub layer2: Box<ReluLayer>,
    pub layer3: Box<AffineLayer>,
    pub last_layer: Box<SoftmaxWithLossLayer>,
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
        let w1 = NumpyArray::from_vec(&a1).muls(2.0).adds(-1.0).muls(weight_init_std);
        let w2 = NumpyArray::from_vec(&a2).muls(2.0).adds(-1.0).muls(weight_init_std);
        let b1 = NumpyVector::from_vec(&vec![0.0; hidden_size]);
        let b2 = NumpyVector::from_vec(&vec![0.0; output_size]);
        Network {
            layer1: Box::new(AffineLayer::new(w1, b1)),
            layer2: Box::new(ReluLayer::new()),
            layer3: Box::new(AffineLayer::new(w2, b2)),
            last_layer: Box::new(SoftmaxWithLossLayer::new()),
        }
    }

    pub fn predict(&mut self, input: &NumpyVector) -> NumpyVector {
        let mut tmp = self.layer1.forward(input);
        tmp = self.layer2.forward(&tmp);
        tmp = self.layer3.forward(&tmp);
        tmp
    }

    pub fn loss(&mut self, x: &NumpyVector, t: &NumpyVector) -> f32 {
        let y = self.predict(&x);
        self.last_layer.forward(&y, &t)
    }

    pub fn gradient(&mut self, x: &NumpyVector, t: &NumpyVector) -> ((NumpyArray, NumpyVector), (NumpyArray, NumpyVector)) {
        let _ = self.loss(x, t);
        let mut dout = NumpyVector::new(0);
        dout = self.last_layer.backward(&dout);
        dout = self.layer3.backward(&dout);
        dout = self.layer2.backward(&dout);
        self.layer1.backward(&dout);
        ((self.layer1.dw.clone(), self.layer1.db.clone()), (self.layer3.dw.clone(), self.layer3.db.clone()))
    }
}

#[test]
fn predict_test() {
    let mut network = Network::new(2, 3, 2, 0.01);
    let result = network.predict(&NumpyVector::from_vec(&vec![0.0, 0.0]));
    assert_eq!(result.data.len(), 2);
}

#[test]
fn gradient_test() {
    let network = Network::new(2, 3, 2, 0.01);
    let x = NumpyVector::from_vec(&vec![1.0, 0.2]);
    let t = NumpyVector::from_vec(&vec![0.0, 1.0]);
//    let result = network.gradient(&x, &t);
}
