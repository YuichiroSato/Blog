use crate::numpy;
use crate::numpy::*;

pub fn numerical_diff(f: &Fn(f32) -> f32, x: f32) -> f32 {
    (f(x + 0.0001) - f(x - 0.0001)) / 0.0002
}

pub fn numerical_grad(f: &Fn(&NumpyVector) -> f32, x: &mut NumpyVector) -> NumpyVector {
    let mut grad = NumpyVector::new(x.data.len());

    for i in 0..x.data.len() {
        let tmp_val = x.data[i];

        x.data[i] = tmp_val + 0.0001;
        let fx1 = f(&x);
        x.data[i] = tmp_val - 0.0001;
        let fx2 = f(&x);

        grad.data[i] = (fx1 - fx2) / 0.0002;

        x.data[i] = tmp_val;
    }
    grad
}

pub fn numerical_grad_array(f: &Fn(&NumpyArray) -> f32, x: &mut NumpyArray) -> NumpyArray {
    let mut grad = NumpyArray::new(x.row, x.column);

    for r in 0..x.row {
        for c in 0..x.column {
            let tmp_val = x.data[r][c];

            x.data[r][c] = tmp_val + 0.0001;
            let fx1 = f(&x);
            x.data[r][c] = tmp_val - 0.0001;
            let fx2 = f(&x);

            grad.data[r][c] = (fx1 - fx2) / 0.0002;

            x.data[r][c] = tmp_val;
        }
    }
    grad
}

pub fn numerical_grad_box(f: &Fn(Box<NumpyArray>) -> f32, x: &mut Box<NumpyArray>) -> NumpyArray {
    let mut grad = NumpyArray::new(x.row, x.column);

    for r in 0..x.row {
        for c in 0..x.column {
            let tmp_val = (*x).data[r][c];

            (*x).data[r][c] = tmp_val + 0.0001;
            let fx1 = f(x.clone());
            (*x).data[r][c] = tmp_val - 0.0001;
            let fx2 = f(x.clone());

            grad.data[r][c] = (fx1 - fx2) / 0.0002;

            x.data[r][c] = tmp_val;
        }
    }
    grad
}

pub fn gradient_descent<'a>(f: &Fn(&NumpyVector) -> f32, x: &'a mut NumpyVector, lr: f32, step: usize) -> &'a NumpyVector {
    for _ in 0..step {
        //let a = numerical_grad(f, x).muls(lr);
        //x = x.subv(a);
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
    let result = numerical_grad(&|x: &NumpyVector| x.data[0] * x.data[0] + x.data[1] * x.data[1], &mut NumpyArray::row_vec(&vec![3.0, 4.0]).to_vector());
    assert!(result.data[0] > 5.98 && result.data[0] < 6.01);
    assert!(result.data[1] > 7.99 && result.data[1] < 8.01);
}

#[test]
fn gradient_descent_test() {
    let f = |x: &NumpyVector| x.data[0] * x.data[0] + x.data[1] * x.data[1];
    let mut init = NumpyArray::row_vec(&vec![-3.0, 4.0]).to_vector();
    let result = gradient_descent(&f, &mut init, 0.1, 100);
    assert!(result.data[0] > -0.001 && result.data[0] < 0.001);
    assert!(result.data[1] > -0.001 && result.data[1] < 0.001);
}
