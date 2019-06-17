use crate::numpy;
use crate::numpy::*;
use crate::layer::*;

pub struct SimpleNet {
    pub w: NumpyArray
}

impl SimpleNet {
    pub fn new() -> Self {
        SimpleNet {
            w: NumpyArray::from_vec(&vec![vec![0.5, 0.1, 0.0]; 2])
        }
    }

    pub fn predict(&self, x: &NumpyVector) -> NumpyVector {
        numpy::dot(&x.to_row(), &self.w).to_vector()
    }

    pub fn loss(&self, x: &NumpyVector, t: &NumpyVector) -> f32 {
        let mut z = self.predict(&x);
        let mut y = softmax(&mut z);
        cross_entropy_error(&mut y, &t)
    }
}

#[test]
fn predict_test() {
    let net = SimpleNet::new();
    let result = net.predict(&NumpyVector::from_vec(&vec![0.6, 0.9]));
    assert_eq!(vec![0.75, 0.15, 0.0], result.data);
}

#[test]
fn loss_test() {
    let net = SimpleNet::new();
    let x = NumpyVector::from_vec(&vec![0.6, 0.9]);
    let t1 = NumpyVector::from_vec(&vec![1.0, 0.0, 0.0]);
    let r1 = net.loss(&x, &t1);
    let t2 = NumpyVector::from_vec(&vec![0.0, 1.0, 0.0]);
    let r2 = net.loss(&x, &t2);
    let t3 = NumpyVector::from_vec(&vec![0.0, 0.0, 1.0]);
    let r3 = net.loss(&x, &t3);
    assert!(r1 > 0.70 && r1 < 0.71);
    assert!(r2 > 1.30 && r2 < 1.31);
    assert!(r3 > 1.45 && r3 < 1.46);
}
