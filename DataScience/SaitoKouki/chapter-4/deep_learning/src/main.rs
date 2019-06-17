extern crate rand;

mod calculus;
mod layer;
mod mnist;
mod network;
mod numpy;
mod simple_net;

use crate::numpy::*;
use crate::network::*;

fn main() {
    let ((train_images, train_labels), (test_images, test_labels)) = mnist::load_mnist().unwrap();

    let mut network = Network::new(784, 50, 10, 0.01);

    println!("start");
    for i in 0..train_images.len() {
        let x = train_images[i].flatten();
        let t = endode_train_vec(train_labels[i]);
        let mut grad = network.gradient(&x, &t);
        let a1 = numpy::add(&mut network.w1, &(*grad.w1).muls(-0.01));
        let a2 = numpy::add(&mut network.w2, &(*grad.w2).muls(-0.01));
        let a3 = numpy::add(&mut network.b1, &(*grad.b1).muls(-0.01));
        let a4 = numpy::add(&mut network.b2, &(*grad.b2).muls(-0.01));
      //network.w1 = Box::new(*a1);
      //network.w2 = Box::new(*a2);
      //network.b1 = Box::new(*a3);
      //network.b2 = Box::new(*a4);

        println!("{}", network.loss(&x, &t));
    }
}

fn endode_train_vec(t: u8) -> NumpyVector {
    let mut v = vec![0.0; 10];
    v[t as usize] = 1.0;
    NumpyVector {
        data: v
    }
}
