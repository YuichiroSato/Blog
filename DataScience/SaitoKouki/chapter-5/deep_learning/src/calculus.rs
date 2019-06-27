use crate::numpy;
use crate::numpy::*;

pub fn numerical_diff(f: &Fn(f32) -> f32, x: f32) -> f32 {
    (f(x + 0.0001) - f(x - 0.0001)) / 0.0002
}

pub fn numerical_grad(f: &Fn(NumpyVector) -> f32, x: &NumpyVector) -> NumpyVector {
    let mut grad = NumpyVector::new(x.data.len());
    let mut tmp_data = x.data.clone();

    for i in 0..x.data.len() {
        let tmp_val = tmp_data[i];

        tmp_data[i] = tmp_val + 0.0001;
        let fx1 = f(NumpyVector::from_vec(&tmp_data));
        tmp_data[i] = tmp_val - 0.0001;
        let fx2 = f(NumpyVector::from_vec(&tmp_data));

        grad.data[i] = (fx1 - fx2) / 0.0002;

        tmp_data[i] = tmp_val;
    }
    grad
}

pub fn numerical_grad_array(f: &Fn(NumpyArray) -> f32, x: &NumpyArray) -> NumpyArray {
    let mut grad = NumpyArray::new(x.row, x.column);
    let mut tmp_data = x.data.clone();

    for r in 0..x.row {
        for c in 0..x.column {
            let tmp_val = tmp_data[r][c];

            tmp_data[r][c] = tmp_val + 0.0001;
            let fx1 = f(NumpyArray::from_vec(&tmp_data));
            tmp_data[r][c] = tmp_val - 0.0001;
            let fx2 = f(NumpyArray::from_vec(&tmp_data));

            grad.data[r][c] = (fx1 - fx2) / 0.0002;

            tmp_data[r][c] = tmp_val;
        }
    }
    grad
}

pub fn gradient_descent(f: &Fn(NumpyVector) -> f32, init_x: &NumpyVector, lr: f32, step: usize) -> NumpyVector {
    let mut x = NumpyVector::from_vec(&init_x.data.clone());
    for _ in 0..step {
        let grad = numerical_grad(f, &x);
        x = x.subv(&grad.muls(lr));
    }
    x
}

#[test]
fn numerical_diff_test() {
    let result = numerical_diff(&|x| x * x, 2.0);
    assert!(result > 3.99 && result < 4.01);
}

#[test]
fn numerical_grad_test() {
    let result = numerical_grad(&|x: NumpyVector| x.data[0] * x.data[0] + x.data[1] * x.data[1], &NumpyArray::row_vec(&vec![3.0, 4.0]).to_vector());
    assert!(result.data[0] > 5.98 && result.data[0] < 6.01);
    assert!(result.data[1] > 7.99 && result.data[1] < 8.01);
}

#[test]
fn gradient_descent_test() {
    let f = |x: NumpyVector| x.data[0] * x.data[0] + x.data[1] * x.data[1];
    let init = NumpyArray::row_vec(&vec![-3.0, 4.0]).to_vector();
    let result = gradient_descent(&f, &init, 0.1, 100);
    assert!(result.data[0] > -0.001 && result.data[0] < 0.001);
    assert!(result.data[1] > -0.001 && result.data[1] < 0.001);
}
