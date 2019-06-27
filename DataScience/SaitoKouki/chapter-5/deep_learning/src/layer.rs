use crate::calculus::*;
use crate::numpy;
use crate::numpy::*;

pub fn identity(x: &NumpyVector) -> &NumpyVector {
    x
}

pub fn step(x: &NumpyVector) -> NumpyVector {
    x.map(&|x| if x > 0.0 { 1.0 } else { 0.0 })
}

pub fn sigmoid(x: &NumpyVector) -> NumpyVector {
    x.map(&|x| 1.0 / (1.0 + f32::exp(x)))
}

pub fn relu(x: &NumpyVector) -> NumpyVector {
    x.map(&|x| if x > 0.0 { x } else { 0.0 })
}

pub fn softmax(x: &NumpyVector) -> NumpyVector {
    let c = x.max();
    let s = x.map(&|x| f32::exp(x - c)).sum();
    x.map(&|x| f32::exp(x - c) / s)
}

pub fn mean_squared_error(input: &NumpyVector, train: &NumpyVector) -> f32 {
    0.5 * input.subv(&train).map(&|x| x * x).sum()
}

pub fn cross_entropy_error(input: &NumpyVector, train: &NumpyVector) -> f32 {
    - numpy::norm(&train, &(input.map(&|x| (x + 0.0000001).ln())))
}

pub fn cross_entropy_error_batch(inputs: &Vec<NumpyVector>, trains: &Vec<NumpyVector>) -> f32 {
    let mut e = 0.0;
    for i in 0..inputs.len() {
        e += cross_entropy_error(&inputs[i], &trains[i]);
    }
    e / (inputs.len() as f32)
}

pub trait Layer {
    fn forward(&mut self, x: &NumpyVector) -> NumpyVector;
    fn backward(&mut self, x: &NumpyVector) -> NumpyVector;
}

pub struct ReluLayer {
    mask: NumpyVector,
}

impl Layer for ReluLayer {
    fn forward(&mut self, x: &NumpyVector) -> NumpyVector {
        let out = relu(&x);
        self.mask = out.clone();
        out
    }

    fn backward(&mut self, x: &NumpyVector) -> NumpyVector {
        let mut dx = NumpyVector::new(x.data.len());
        for i in 0..dx.data.len() {
            dx.data[i] = if self.mask.data[i] > 0.0 {
                x.data[i]
            } else {
                0.0
            }
        }
        dx
    }
}

impl ReluLayer {
    pub fn new() -> Self {
        ReluLayer {
            mask: NumpyVector::new(0),
        }
    }
}

pub struct SigmoidLayer {
    out: NumpyVector,
}

impl Layer for SigmoidLayer {
    fn forward(&mut self, x: &NumpyVector) -> NumpyVector {
        self.out = sigmoid(&x);
        self.out.clone()
    }

    fn backward(&mut self, x: &NumpyVector) -> NumpyVector {
        let one = NumpyVector::from_vec(&vec![1.0; x.data.len()]);
        self.out.muls(numpy::norm(&self.out, &one.subv(&x)))
    }
}

impl SigmoidLayer {
    pub fn new() -> Self {
        SigmoidLayer {
            out: NumpyVector::new(0),
        }
    }
}

pub struct AffineLayer {
    pub w: NumpyArray,
    pub b: NumpyVector,
    pub dw: NumpyArray,
    pub db: NumpyVector,
    x: NumpyVector,
    out: NumpyVector,
}

impl Layer for AffineLayer {
    fn forward(&mut self, x: &NumpyVector) -> NumpyVector {
        self.x = x.clone();
        self.out = numpy::dot(&self.x.to_row(), &self.w).to_vector().addv(&self.b);
        self.out.clone()
    }

    fn backward(&mut self, x: &NumpyVector) -> NumpyVector {
        let dx = numpy::dot(&x.to_row(), &self.w.transpose()).to_vector();
        self.dw = numpy::dot(&self.x.to_column(), &x.to_row());
        self.db = x.clone();
        dx
    }
}

impl AffineLayer {
    pub fn new(w: NumpyArray, b: NumpyVector) -> Self {
        AffineLayer {
            w: w,
            b: b,
            dw: NumpyArray::new(0, 0),
            db: NumpyVector::new(0),
            x: NumpyVector::new(0),
            out: NumpyVector::new(0),
        }
    }
}

pub trait LastLayer {
    fn forward(&mut self, x: &NumpyVector, t: &NumpyVector) -> f32;
    fn backward(&mut self, x: &NumpyVector) -> NumpyVector;
}

pub struct SoftmaxWithLossLayer {
    y: NumpyVector,
    t: NumpyVector,
}

impl LastLayer for SoftmaxWithLossLayer {
    fn forward(&mut self, x: &NumpyVector, t: &NumpyVector) -> f32 {
        self.t = t.clone();
        self.y = softmax(x);
        cross_entropy_error(&self.y, &self.t)
    }

    fn backward(&mut self, _: &NumpyVector) -> NumpyVector {
        self.y.subv(&self.t)
    }
}

impl SoftmaxWithLossLayer {
    pub fn new() -> Self {
        SoftmaxWithLossLayer {
            y: NumpyVector::new(0),
            t: NumpyVector::new(0),
        }
    }
}

#[test]
fn identity_test() {
    let x = NumpyVector::from_vec(&vec![1.0, 2.0, 3.0]);
    assert_eq!(x.data, identity(&x).data);
}

#[test]
fn step_test() {
    let x = NumpyVector::from_vec(&vec![1.0, -2.0, 3.0]);
    let result = step(&x);
    let expected = vec![1.0, 0.0, 1.0];
    assert_eq!(expected, result.data);
}

#[test]
fn sigmoid_test() {
    let x = NumpyVector::from_vec(&vec![0.0, 0.0, 0.0]);
    let result = sigmoid(&x);
    let expected = vec![0.5, 0.5, 0.5];
    assert_eq!(expected, result.data);
}

#[test]
fn relu_test() {
    let x = NumpyVector::from_vec(&vec![1.0, -2.0, 3.0]);
    let result = relu(&x);
    let expected = vec![1.0, 0.0, 3.0];
    assert_eq!(expected, result.data);
}

#[test]
fn softmax_test() {
    let x = NumpyVector::from_vec(&vec![0.0, 1.0, 2.0]);
    let result = softmax(&x);

    println!("{:?}", result.data);
    assert!(result.data[0] > 0.0 && result.data[0] < 0.1);
    assert!(result.data[1] > 0.2 && result.data[1] < 0.3);
    assert!(result.data[2] > 0.66 && result.data[2] < 0.67);
}

#[test]
fn mean_squared_error_test() {
    let input = NumpyVector::from_vec(&vec![1.0, 2.0, 2.0, 3.0]);
    let train = NumpyVector::from_vec(&vec![3.0, 2.0, 2.0, 1.0]);
    let result = mean_squared_error(&input, &train);
    assert_eq!(4.0, result);
}

#[test]
fn cross_entropy_error_test() {
    let input = NumpyVector::from_vec(&vec![0.0, 0.6, 0.1, 0.3]);
    let train = NumpyVector::from_vec(&vec![0.0, 1.0, 0.0, 0.0]);
    let result = cross_entropy_error(&input, &train);
    assert!(result > 0.510 && result < 0.511);
}

#[test]
fn cross_entropy_error_batch_test() {
    let input1 = NumpyVector::from_vec(&vec![0.0, 0.6, 0.1, 0.3]);
    let train1 = NumpyVector::from_vec(&vec![0.0, 1.0, 0.0, 0.0]);
    let input2 = NumpyVector::from_vec(&vec![0.0, 0.6, 0.1, 0.3]);
    let train2 = NumpyVector::from_vec(&vec![0.0, 1.0, 0.0, 0.0]);
    let result = cross_entropy_error_batch(&vec![input1, input2], &vec![train1, train2]);
    assert!(result > 0.510 && result < 0.511);
}

#[test]
fn relu_layer_test() {
    let x = NumpyVector::from_vec(&vec![0.1, -0.2, 0.3]);
    let d = NumpyVector::from_vec(&vec![0.3, 0.2, 0.2]);
    let mut layer = ReluLayer::new();

    let out = layer.forward(&x);
    let dx = layer.backward(&d);

    let expected_out = vec![0.1, 0.0, 0.3];
    let expected_dx = vec![0.3, 0.0, 0.2];

    assert_eq!(expected_out, out.data);
    assert_eq!(expected_dx, dx.data);
}

#[test]
fn sigmoid_layer_test() {
    let x = NumpyVector::from_vec(&vec![0.1, 0.2, 0.3]);
    let d = NumpyVector::from_vec(&vec![0.1, 0.2, 0.3]);
    let mut layer = SigmoidLayer::new();

    let out = layer.forward(&x);
    let dx = layer.backward(&d);

    assert!(true);
}

#[test]
fn affine_layer_test() {
    let w = NumpyArray::from_vec(&vec![vec![0.1, 0.0, 0.1], vec![0.0, 0.0, 0.1]]);
    let b = NumpyVector::from_vec(&vec![-0.2, -0.2, -0.2]);
    let x = NumpyVector::from_vec(&vec![1.0, 2.0]);
    let d = NumpyVector::from_vec(&vec![0.1, 0.2, 0.3]);
    let mut layer = AffineLayer::new(w, b);

    let out = layer.forward(&x);
    let dx = layer.backward(&d);

    let expected_dw = vec![vec![0.1, 0.2, 0.3], vec![0.2, 0.4, 0.6]];
    let expected_db = vec![0.1, 0.2, 0.3];

    assert!(out.data[0] > -0.101 && out.data[0] < -0.099);
    assert!(out.data[1] > -0.201 && out.data[1] < -0.199);
    assert!(out.data[2] < 0.101 && out.data[2] > 0.099);

    assert_eq!(expected_dw, layer.dw.data);
    assert_eq!(expected_db, layer.db.data);

    assert!(dx.data[0] > 0.0399 && dx.data[0] < 0.0401);
    assert!(dx.data[1] > 0.0299 && dx.data[1] < 0.0301);
}

#[test]
fn softmax_with_loss_layer_test() {
    let x = NumpyVector::from_vec(&vec![0.1, 0.2, 0.3]);
    let t = NumpyVector::from_vec(&vec![0.0, 1.0, 0.0]);
    let d = NumpyVector::from_vec(&vec![0.1, 0.2, 0.3]);
    let mut layer = SoftmaxWithLossLayer::new();

    let out = layer.forward(&x, &t);
    let dx = layer.backward(&d);

    assert!(true);
}
