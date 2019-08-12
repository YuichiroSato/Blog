use crate::numpy;
use crate::numpy::*;

pub trait ArrayOptimizer {
    fn optimize(&mut self, param: &NumpyArray, grad: &NumpyArray) -> NumpyArray;
}

pub struct ArrayRMSprop {
    lr: f32,
    decay: f32,
    buf: NumpyArray,
}

impl ArrayOptimizer for ArrayRMSprop {
    fn optimize(&mut self, param: &NumpyArray, grad: &NumpyArray) -> NumpyArray {
        self.buf.muls(self.decay);
        self.buf = numpy::add(&self.buf, &grad.map(&|x| { x * x }).muls(1.0 - self.decay));
        let sita = self.buf.map(&|x| { x.sqrt() }).adds(0.00001);
        let ue = grad.muls(self.lr);
        let dp = numpy::div(&ue, &sita);
        numpy::sub(&param, &dp)
    }
}

impl ArrayRMSprop {
    pub fn new(lr: f32, decay: f32, row_size: usize, column_size: usize) -> Self {
        ArrayRMSprop {
            lr: lr,
            decay: decay,
            buf: NumpyArray::new(row_size, column_size),
        }
    }
}

#[test]
fn array_outimizer_test() {
    let mut optimizer = ArrayRMSprop::new(0.01, 0.99, 2, 3);
    let param = NumpyArray::from_vec(&vec![vec![1.0, 1.0, 1.0],vec![1.0, 1.0, 1.0]]);
    let grad = NumpyArray::from_vec(&vec![vec![1.0, 1.0, 1.0],vec![1.0, 1.0, 1.0]]);

    let result = optimizer.optimize(&param, &grad);
    println!("{:?}", result.data);
    let result2 = optimizer.optimize(&result, &grad);
    println!("{:?}", result2.data);
    //assert!(false);
}

pub trait VectorOptimizer {
    fn optimize(&mut self, param: &NumpyVector, grad: &NumpyVector) -> NumpyVector;
}

pub struct VectorRMSprop {
    lr: f32,
    decay: f32,
    buf: NumpyVector,
}

impl VectorOptimizer for VectorRMSprop {
    fn optimize(&mut self, param: &NumpyVector, grad: &NumpyVector) -> NumpyVector {
        self.buf.muls(self.decay);
        self.buf = self.buf.addv(&grad.map(&|x| { x * x }).muls(1.0 - self.decay));
        let sita = self.buf.map(&|x| { x.sqrt() }).adds(0.00001);
        let ue = grad.muls(self.lr);
        let dp = ue.divv(&sita);
        param.subv(&dp)
    }
}

impl VectorRMSprop {
    pub fn new(lr: f32, decay: f32, data_size: usize) -> Self {
        VectorRMSprop {
            lr: lr,
            decay: decay,
            buf: NumpyVector::new(data_size),
        }
    }
}

#[test]
fn vector_outimizer_test() {
    let mut optimizer = VectorRMSprop::new(0.01, 0.99, 3);
    let param = NumpyVector::from_vec(&vec![1.0, 1.0, 1.0]);
    let grad = NumpyVector::from_vec(&vec![1.0, 1.0, 1.0]);

    let result = optimizer.optimize(&param, &grad);
    println!("{:?}", result.data);
    let result2 = optimizer.optimize(&result, &grad);
    println!("{:?}", result2.data);
    //assert!(false);
}



pub trait VecOptimizer {
    fn optimize(&mut self, param: &Vec<f32>, grad: &Vec<f32>) -> Vec<f32>;
}

pub struct VecRMSprop {
    lr: f32,
    decay: f32,
    buf: Vec<f32>,
}

impl VecOptimizer for VecRMSprop {
    fn optimize(&mut self, param: &Vec<f32>, grad: &Vec<f32>) -> Vec<f32> {
        let mut a = vec![0.0; param.len()];
        for i in 0..param.len() {
            self.buf[i] *= self.decay;
            self.buf[i] += (1.0 - self.decay) * grad[i] * grad[i];
            let dx = self.lr * grad[i] / (self.buf[i].sqrt() + 0.00001);
            a[i] = param[i] - dx
        }
        a
    }
}

impl VecRMSprop {
    pub fn new(lr: f32, decay: f32, data_size: usize) -> Self {
        VecRMSprop {
            lr: lr,
            decay: decay,
            buf: vec![0.0; data_size],
        }
    }
}

#[test]
fn vec_outimizer_test() {
    let mut optimizer = VecRMSprop::new(0.01, 0.99, 3);
    let param = vec![1.0, 1.0, 1.0];
    let grad = vec![1.0, 1.0, 1.0];

    let result = optimizer.optimize(&param, &grad);
    println!("{:?}", result);
    let result2 = optimizer.optimize(&result, &grad);
    println!("{:?}", result2);
    //assert!(false);
}

