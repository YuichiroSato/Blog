use crate::calculus::*;
use crate::layer;
use crate::layer::*;
use crate::numpy;
use crate::numpy::*;
use rand::prelude::*;

pub struct Network {
    pub layer1: Box<ConvolutionLayer>,
    pub layer2: Box<CnnReluLayer>,
    pub layer3: Box<MaxPoolingLayer>,
    pub layer4: Box<FullConnector>,
    pub layer5: Box<AffineLayer>,
    pub layer6: Box<ReluLayer>,
    pub layer7: Box<AffineLayer>,
    pub last_layer: Box<SoftmaxWithLossLayer>,
}

impl Network {
    pub fn new(weight_init_std: f32) -> Self {
        let mut r = rand::thread_rng();

        let mut filters = Vec::new();
        for _ in 0..30 {
            let w = NumpyArray::from_vec(&Network::gen_random_vec_vec(&mut r, 5, 5)).muls(2.0).adds(-1.0).muls(weight_init_std);
            filters.push(Filter::new(&mut vec![w], 0, 1, 28, 28));
        }
        let w2 = NumpyArray::from_vec(&Network::gen_random_vec_vec(&mut r, 4320, 100)).muls(2.0).adds(-1.0).muls(weight_init_std);
        let w3 = NumpyArray::from_vec(&Network::gen_random_vec_vec(&mut r, 100, 10)).muls(2.0).adds(-1.0).muls(weight_init_std);
        let b1 = vec![0.0; 30];
        let b2 = NumpyVector::from_vec(&vec![0.0; 100]);
        let b3 = NumpyVector::from_vec(&vec![0.0; 10]);
        Network {
            layer1: Box::new(ConvolutionLayer::new(1, filters, b1, 0, 1, 28, 28)),
            layer2: Box::new(CnnReluLayer::new()),
            layer3: Box::new(MaxPoolingLayer::new(2, 2, 24, 24)),
            layer4: Box::new(FullConnector::new(12, 12)),
            layer5: Box::new(AffineLayer::new(w2, b2)),
            layer6: Box::new(ReluLayer::new()),
            layer7: Box::new(AffineLayer::new(w3, b3)),
            last_layer: Box::new(SoftmaxWithLossLayer::new()),
        }
    }

    fn gen_random_vec(r: &mut ThreadRng, n: usize) -> Vec<f32> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(r.gen());
        }
        v
    }

    fn gen_random_vec_vec(r: &mut ThreadRng, n: usize, m: usize) -> Vec<Vec<f32>> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(Network::gen_random_vec(r, m));
        }
        v
    }

    pub fn predict(&mut self, input: &NumpyArray) -> NumpyVector {
        let mut tmp = self.layer1.forward(&vec![input.clone()]);
        tmp = self.layer2.forward(&tmp);
        tmp = self.layer3.forward(&tmp);
        let mut tmpv = self.layer4.forward(&tmp);
        tmpv = self.layer5.forward(&tmpv);
        tmpv = self.layer6.forward(&tmpv);
        tmpv = self.layer7.forward(&tmpv);
        tmpv
    }

    pub fn loss(&mut self, x: &NumpyArray, t: &NumpyVector) -> f32 {
        let y = self.predict(&x);
        self.last_layer.forward(&y, &t)
    }

    pub fn gradient(&mut self, x: &NumpyArray, t: &NumpyVector) -> ((Vec<NumpyVector>, Vec<f32>), (NumpyArray, NumpyVector), (NumpyArray, NumpyVector)) {
        let _ = self.loss(x, t);
        let mut dout = NumpyVector::new(0);
        dout = self.last_layer.backward(&dout);
        dout = self.layer7.backward(&dout);
        dout = self.layer6.backward(&dout);
        dout = self.layer5.backward(&dout);
        let mut douta = self.layer4.backward(&dout);
        douta = self.layer3.backward(&douta);
        douta = self.layer2.backward(&douta);
        self.layer1.backward(&douta);
        ((self.layer1.get_dw(), self.layer1.db.clone()),
         (self.layer5.dw.clone(), self.layer5.db.clone()),
        (self.layer7.dw.clone(), self.layer7.db.clone()))
    }
}

#[test]
fn predict_test() {
    let mut network = Network::new(2, 3, 2, 0.01);
    //let result = network.predict(&NumpyVector::from_vec(&vec![0.0, 0.0]));
    //assert_eq!(result.data.len(), 2);
}

#[test]
fn gradient_test() {
    let network = Network::new(2, 3, 2, 0.01);
    let x = NumpyVector::from_vec(&vec![1.0, 0.2]);
    let t = NumpyVector::from_vec(&vec![0.0, 1.0]);
//    let result = network.gradient(&x, &t);
}
