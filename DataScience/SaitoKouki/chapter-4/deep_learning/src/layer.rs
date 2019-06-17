use crate::numpy;
use crate::numpy::*;

pub fn identity(x: &NumpyVector) -> &NumpyVector {
    x
}

pub fn step(x: &mut NumpyVector) -> &mut NumpyVector {
    x.map(&|x| if x > 0.0 { 1.0 } else { 0.0 })
}

pub fn sigmoid(x: &mut NumpyVector) -> &mut NumpyVector {
    x.map(&|x| 1.0 / (1.0 + f32::exp(x)))
}

pub fn relu(x: &mut NumpyVector) -> &mut NumpyVector {
    x.map(&|x| if x > 0.0 { x } else { 0.0 })
}

pub fn softmax(x: &mut NumpyVector) -> &mut NumpyVector {
    let c = x.max();
    let mut s = 0.0;
    for i in 0..x.data.len() {
        s += f32::exp(x.data[i] - c);
    }
    x.map(&|x| f32::exp(x - c) / s)
}

pub fn mean_squared_error(input: &mut NumpyVector, train: &NumpyVector) -> f32 {
    0.5 * input.subv(&train).map(&|x| x * x).sum()
}

pub fn cross_entropy_error(input: &mut NumpyVector, train: &NumpyVector) -> f32 {
    - numpy::norm(&train, &(input.map(&|x| (x + 0.0000001).ln())))
}

pub fn cross_entropy_error_batch(inputs: &mut Vec<NumpyVector>, trains: &Vec<NumpyVector>) -> f32 {
    let mut e = 0.0;
    for i in 0..inputs.len() {
        e += cross_entropy_error(&mut inputs[i], &trains[i]);
    }
    e / (inputs.len() as f32)
}

#[test]
fn identity_test() {
    let x = NumpyVector::from_vec(&vec![1.0, 2.0, 3.0]);
    assert_eq!(x.data, identity(&x).data);
}

#[test]
fn step_test() {
    let mut x = NumpyVector::from_vec(&vec![1.0, -2.0, 3.0]);
    let result = step(&mut x);
    let expected = vec![1.0, 0.0, 1.0];
    assert_eq!(expected, result.data);
}

#[test]
fn sigmoid_test() {
    let mut x = NumpyVector::from_vec(&vec![0.0, 0.0, 0.0]);
    let result = sigmoid(&mut x);
    let expected = vec![0.5, 0.5, 0.5];
    assert_eq!(expected, result.data);
}

#[test]
fn relu_test() {
    let mut x = NumpyVector::from_vec(&vec![1.0, -2.0, 3.0]);
    let result = relu(&mut x);
    let expected = vec![1.0, 0.0, 3.0];
    assert_eq!(expected, result.data);
}

#[test]
fn softmax_test() {
    let mut x = NumpyVector::from_vec(&vec![0.0, 1.0, 2.0]);
    let result = softmax(&mut x);

    println!("{:?}", result.data);
    assert!(result.data[0] > 0.0 && result.data[0] < 0.1);
    assert!(result.data[1] > 0.2 && result.data[1] < 0.3);
    assert!(result.data[2] > 0.66 && result.data[2] < 0.67);
}

#[test]
fn mean_squared_error_test() {
    let mut input = NumpyVector::from_vec(&vec![1.0, 2.0, 2.0, 3.0]);
    let train = NumpyVector::from_vec(&vec![3.0, 2.0, 2.0, 1.0]);
    let result = mean_squared_error(&mut input, &train);
    assert_eq!(4.0, result);
}

#[test]
fn cross_entropy_error_test() {
    let mut input = NumpyVector::from_vec(&vec![0.0, 0.6, 0.1, 0.3]);
    let train = NumpyVector::from_vec(&vec![0.0, 1.0, 0.0, 0.0]);
    let result = cross_entropy_error(&mut input, &train);
    assert!(result > 0.510 && result < 0.511);
}

#[test]
fn cross_entropy_error_batch_test() {
    let input1 = NumpyVector::from_vec(&vec![0.0, 0.6, 0.1, 0.3]);
    let train1 = NumpyVector::from_vec(&vec![0.0, 1.0, 0.0, 0.0]);
    let input2 = NumpyVector::from_vec(&vec![0.0, 0.6, 0.1, 0.3]);
    let train2 = NumpyVector::from_vec(&vec![0.0, 1.0, 0.0, 0.0]);
    let result = cross_entropy_error_batch(&mut vec![input1, input2], &vec![train1, train2]);
    assert!(result > 0.510 && result < 0.511);
}
