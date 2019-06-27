extern crate rand;

mod calculus;
mod layer;
mod mnist;
mod network;
mod numpy;

use crate::numpy::*;
use crate::network::*;
use rand::prelude::*;

fn main() {
    let ((train_images, train_labels), (test_images, test_labels)) = mnist::load_mnist().unwrap();

    let train_size = train_images.len() as f32;
    let normalized_train_images: Vec<NumpyVector> = train_images.iter()
        .map(|arr| arr.map(&|x| x as f32 / 256.0).flatten())
        .collect();
    let encoded_train_labels: Vec<NumpyVector> = train_labels.iter()
        .map(|&v| endode_train_vec(v))
        .collect();
    let normalized_test_images: Vec<NumpyVector> = test_images.iter()
        .map(|arr| arr.map(&|x| x as f32 / 256.0).flatten())
        .collect();
    let encoded_test_labels: Vec<NumpyVector> = test_labels.iter()
        .map(|&v| endode_train_vec(v))
        .collect();
    let mut network = Network::new(784, 50, 10, 0.01);

    let mut r = rand::thread_rng();

    println!("learning start");
    for n in 0..10009 {
        let rs: Vec<f32> = (0..100).map(|_| r.gen() ).collect();
        let is: Vec<usize> = rs.iter().map(|&i| (i * train_size) as usize).collect();

        let mut dw1 = NumpyArray::new(784, 50);
        let mut db1 = NumpyVector::new(50);
        let mut dw2 = NumpyArray::new(50, 10);
        let mut db2 = NumpyVector::new(10);

        for j in 0..100 {
            let x = &normalized_train_images[is[j]];
            let t = &encoded_train_labels[is[j]];
            let ((_dw1, _db1), (_dw2, _db2)) = network.gradient(&x, &t);
            dw1 = numpy::add(&dw1, &_dw1);
            db1 = _db1.addv(&_db1);
            dw2 = numpy::add(&dw2, &_dw2);
            db2 = _db2.addv(&_db2);
        }

        network.layer1.w = numpy::add(&network.layer1.w, &dw1.muls(-0.001));
        network.layer3.w = numpy::add(&network.layer3.w, &dw2.muls(-0.001));
        network.layer1.b = network.layer1.b.addv(&db1.muls(-0.001));
        network.layer3.b = network.layer3.b.addv(&db2.muls(-0.001));

        let mut acc = 0;
        for i in is {
            let x = &normalized_train_images[i];
            let predict = network.predict(&x).argmax() as u8;
            if train_labels[i] == predict {
                acc += 1;
            }

        }
        println!("{} {}", n, acc);
    }
    println!("learning finished");


    let mut acc = 0;
    for i in 0..normalized_test_images.len() {
        let x = &normalized_test_images[i];
        let predict = network.predict(&x).argmax() as u8;
        if test_labels[i] == predict {
            acc += 1;
        }
    }
    println!("accuracy {}", acc as f32 / test_labels.len() as f32);
}

fn endode_train_vec(t: u8) -> NumpyVector {
    let mut v = vec![0.0; 10];
    v[t as usize] = 1.0;
    NumpyVector {
        data: v
    }
}
