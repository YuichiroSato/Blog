use crate::numpy::*;

pub fn identity(x: &NumpyArray) -> &NumpyArray {
    x
}

pub fn step(x: &NumpyArray) -> NumpyArray {
    x.apply(&|x| if x > 0.0 { 1.0 } else { 0.0 })
}

pub fn sigmoid(x: &NumpyArray) -> NumpyArray {
    x.apply(&|x| 1.0 / (1.0 + f32::exp(x)))
}

pub fn relu(x: &NumpyArray) -> NumpyArray {
    x.apply(&|x| if x > 0.0 { x } else { 0.0 })
}

pub fn softmax(x: &NumpyArray) -> NumpyArray {
    let mut c = x.max();
    let s = x.apply(&|x| f32::exp(x - c)).sum();
    x.apply(&|x| f32::exp(x - c) / s)
}

#[test]
fn identity_test() {
    let x = NumpyArray::column_vec(vec![1.0, 2.0, 3.0]);
    assert_eq!(x.data, identity(&x).data);
}

#[test]
fn step_test() {
    let x = NumpyArray::row_vec(vec![1.0, -2.0, 3.0]);
    let result = step(&x);
    let expected = vec![vec![1.0, 0.0, 1.0]];
    assert_eq!(expected, result.data);
}

#[test]
fn sigmoid_test() {
    let x = NumpyArray::row_vec(vec![0.0, 0.0, 0.0]);
    let result = sigmoid(&x);
    let expected = vec![vec![0.5, 0.5, 0.5]];
    assert_eq!(expected, result.data);
}

#[test]
fn relu_test() {
    let x = NumpyArray::row_vec(vec![1.0, -2.0, 3.0]);
    let result = relu(&x);
    let expected = vec![vec![1.0, 0.0, 3.0]];
    assert_eq!(expected, result.data);
}

#[test]
fn softmax_test() {
    let x = NumpyArray::row_vec(vec![0.0, 1.0, 2.0]);
    let result = softmax(&x);

    println!("{:?}", result.data);
    assert!(result.data[0][0] > 0.0 && result.data[0][0] < 0.1);
    assert!(result.data[0][1] > 0.2 && result.data[0][1] < 0.3);
    assert!(result.data[0][2] > 0.66 && result.data[0][2] < 0.67);
}
