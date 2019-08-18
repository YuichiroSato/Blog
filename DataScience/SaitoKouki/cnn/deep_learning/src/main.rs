extern crate rand;

mod calculus;
mod optimizer;
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
    let normalized_train_images: Vec<NumpyArray> = train_images.iter()
        .map(|arr| arr.map(&|x| x as f32 / 256.0))
        .collect();
    let encoded_train_labels: Vec<NumpyVector> = train_labels.iter()
        .map(|&v| endode_train_vec(v))
        .collect();
    let normalized_test_images: Vec<NumpyArray> = test_images.iter()
        .map(|arr| arr.map(&|x| x as f32 / 256.0))
        .collect();
    let encoded_test_labels: Vec<NumpyVector> = test_labels.iter()
        .map(|&v| endode_train_vec(v))
        .collect();
    let mut network = Network::new();

    let mut r = rand::thread_rng();

    println!("learning start");
    for n in 0..500 {
        let rs: Vec<f32> = (0..100).map(|_| r.gen() ).collect();
        let is: Vec<usize> = rs.iter().map(|&i| (i * train_size) as usize).collect();

        for j in 0..100 {
            let x = &normalized_train_images[is[j]];
            let t = &encoded_train_labels[is[j]];
            network.gradient(&x, &t);
        }

        network.update();

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
