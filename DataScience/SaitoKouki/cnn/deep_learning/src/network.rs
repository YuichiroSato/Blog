use crate::calculus::*;
use crate::layer;
use crate::layer::*;
use crate::numpy;
use crate::numpy::*;
use rand::distributions::{Normal, Distribution};
use rand::prelude::*;

pub struct Network {
    pub layer1: Box<ConvolutionLayer>,
    pub layer2: Box<CnnReluLayer>,
    pub layer3: Box<MaxPoolingLayer>,
    pub layer4: Box<ConvolutionLayer>,
    pub layer5: Box<CnnReluLayer>,
    pub layer6: Box<MaxPoolingLayer>,
    pub layer7: Box<ConvolutionLayer>,
    pub layer8: Box<CnnReluLayer>,
    pub layer9: Box<FullConnector>,
    pub layer10: Box<AffineLayer>,
    pub layer11: Box<ReluLayer>,
    pub last_layer: Box<SoftmaxWithLossLayer>,
}

impl Network {
    pub fn new() -> Self {
        let mut filters1 = Vec::new();
        for _ in 0..32 {
            let w = NumpyArray::from_vec(&Network::gen_random_vec_vec(rand::thread_rng(), 3, 3, Network::weight(3, 3, 32)));
            filters1.push(Filter::new(&mut vec![w], 0, 1, 28, 28));
        }

        let mut filters2 = Vec::new();
        for _ in 0..64 {
            let mut fs = Vec::new();
            for _ in 0..32 {
                let w = NumpyArray::from_vec(&Network::gen_random_vec_vec(rand::thread_rng(), 4, 4, Network::weight(4, 4, 64)));
                fs.push(w);
            }
            filters2.push(Filter::new(&mut fs, 0, 1, 13, 13));
        }

        let mut filters3 = Vec::new();
        for _ in 0..64 {
            let mut fs = Vec::new();
            for _ in 0..64 {
                let w = NumpyArray::from_vec(&Network::gen_random_vec_vec(rand::thread_rng(), 3, 3, Network::weight(3, 3, 64)));
                fs.push(w);
            }
            filters3.push(Filter::new(&mut fs, 0, 1, 5, 5));
        }

        let w = NumpyArray::from_vec(&Network::gen_random_vec_vec(rand::thread_rng(), 576, 10, 1.0 / ((576.0 as f64).sqrt() as f64)));
        let b1 = vec![0.0; 32];
        let b2 = vec![0.0; 64];
        let b3 = vec![0.0; 64];
        let b4 = NumpyVector::from_vec(&vec![0.0; 10]);
        Network {
            layer1: Box::new(ConvolutionLayer::new(1, filters1, b1, 0, 1, 28, 28)),
            layer2: Box::new(CnnReluLayer::new()),
            layer3: Box::new(MaxPoolingLayer::new(2, 2, 26, 26)),
            layer4: Box::new(ConvolutionLayer::new(32, filters2, b2, 0, 1, 13, 13)),
            layer5: Box::new(CnnReluLayer::new()),
            layer6: Box::new(MaxPoolingLayer::new(2, 2, 10, 10)),
            layer7: Box::new(ConvolutionLayer::new(64, filters3, b3, 0, 1, 5, 5)),
            layer8: Box::new(CnnReluLayer::new()),
            layer9: Box::new(FullConnector::new(3, 3)),
            layer10: Box::new(AffineLayer::new(w, b4)),
            layer11: Box::new(ReluLayer::new()),
            last_layer: Box::new(SoftmaxWithLossLayer::new()),
        }
    }

    fn weight(width: usize, heigth: usize, depth: usize) -> f64 {
        1.0 / ((width * heigth * depth) as f64).sqrt()
    }

    fn gen_random_vec(mut r: ThreadRng, n: usize, std: f64) -> Vec<f32> {
        let normal = Normal::new(0.0, std);
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(normal.sample(&mut r) as f32);
        }
        v
    }

    fn gen_random_vec_vec(mut r: ThreadRng, n: usize, m: usize, std: f64) -> Vec<Vec<f32>> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(Network::gen_random_vec(rand::thread_rng(), m, std));
        }
        v
    }

    pub fn predict(&mut self, input: &NumpyArray) -> NumpyVector {
        let mut tmp = self.layer1.forward(&vec![input.clone()]);
        tmp = self.layer2.forward(&tmp);
        tmp = self.layer3.forward(&tmp);
        tmp = self.layer4.forward(&tmp);
        tmp = self.layer5.forward(&tmp);
        tmp = self.layer6.forward(&tmp);
        tmp = self.layer7.forward(&tmp);
        tmp = self.layer8.forward(&tmp);
        let mut tmpv = self.layer9.forward(&tmp);
        tmpv = self.layer10.forward(&tmpv);
        tmpv = self.layer11.forward(&tmpv);
        tmpv
    }

    pub fn loss(&mut self, x: &NumpyArray, t: &NumpyVector) -> f32 {
        let y = self.predict(&x);
        self.last_layer.forward(&y, &t)
    }

    pub fn gradient(&mut self, x: &NumpyArray, t: &NumpyVector) {
        let _ = self.loss(x, t);
        let mut dout = NumpyVector::new(0);
        dout = self.last_layer.backward(&dout);
        dout = self.layer11.backward(&dout);
        dout = self.layer10.backward(&dout);
        let mut douta = self.layer9.backward(&dout);
        douta = self.layer8.backward(&douta);
        douta = self.layer7.backward(&douta);
        douta = self.layer6.backward(&douta);
        douta = self.layer5.backward(&douta);
        douta = self.layer4.backward(&douta);
        douta = self.layer3.backward(&douta);
        douta = self.layer2.backward(&douta);
        self.layer1.backward(&douta);
    }

    pub fn update(&mut self) {
        self.layer1.update();
        self.layer2.update();
        self.layer3.update();
        self.layer5.update();
        self.layer6.update();
        self.layer7.update();
        self.layer8.update();
        self.layer10.update();
        self.layer11.update();
    }
}

#[test]
fn predict_test() {
    let mut network = Network::new(2.0);
    //let result = network.predict(&NumpyVector::from_vec(&vec![0.0, 0.0]));
    //assert_eq!(result.data.len(), 2);
}

#[test]
fn gradient_test() {
    let network = Network::new(2.0);
    let x = NumpyVector::from_vec(&vec![1.0, 0.2]);
    let t = NumpyVector::from_vec(&vec![0.0, 1.0]);
//    let result = network.gradient(&x, &t);
}
