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
    let mut c = x.max();
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
