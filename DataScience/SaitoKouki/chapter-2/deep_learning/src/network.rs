use crate::layer;
use crate::layer::*;
use crate::numpy;
use crate::numpy::*;

pub struct Network {
    w1: NumpyArray,
    w2: NumpyArray,
    w3: NumpyArray,
    b1: NumpyArray,
    b2: NumpyArray,
    b3: NumpyArray,
}

impl Network {
    pub fn new() -> Self {
        Network {
            w1: NumpyArray::from_vec(vec![vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0]]),
            w2: NumpyArray::from_vec(vec![vec![0.0, 0.0], vec![0.0, 0.0], vec![0.0, 0.0]]),
            w3: NumpyArray::from_vec(vec![vec![0.0, 0.0], vec![0.0, 0.0]]),
            b1: NumpyArray::row_vec(vec![0.0, 0.0, 0.0]),
            b2: NumpyArray::row_vec(vec![0.0, 0.0]),
            b3: NumpyArray::row_vec(vec![0.0, 0.0]),
        }
    }

    pub fn predict(&self, input: NumpyArray) -> NumpyArray {
        let a1 = numpy::add(&numpy::dot(&input, &self.w1), &self.b1);
        let z1 = layer::sigmoid(&a1);
        let a2 = numpy::add(&numpy::dot(&z1, &self.w2), &self.b2);
        let z2 = layer::sigmoid(&a2);
        let a3 = numpy::add(&numpy::dot(&z2, &self.w3), &self.b3);
        layer::softmax(&a3)
    }
}

#[test]
fn predict_test() {
    let network = Network::new();
    let result = network.predict(NumpyArray::row_vec(vec![0.0, 0.0]));
    let expected = NumpyArray::row_vec(vec![0.0, 0.0]);
    assert_ne!(expected.data, result.data);
}
