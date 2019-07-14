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
    let mut network = Network::new(0.01);

    let mut r = rand::thread_rng();

    println!("learning start");
    for n in 0..10009 {
        let rs: Vec<f32> = (0..100).map(|_| r.gen() ).collect();
        let is: Vec<usize> = rs.iter().map(|&i| (i * train_size) as usize).collect();

        let mut dw1 = vec![NumpyVector::new(25); 30];
        let mut db1 = vec![0.0; 30];
        let mut dw2 = NumpyArray::new(4320, 100);
        let mut db2 = NumpyVector::new(100);
        let mut dw3 = NumpyArray::new(100, 10);
        let mut db3 = NumpyVector::new(10);

        for j in 0..100 {
            let x = &normalized_train_images[is[j]];
            let t = &encoded_train_labels[is[j]];
            let ((_dw1, _db1), (_dw2, _db2), (_dw3, _db3)) = network.gradient(&x, &t);
            for i in 0..30 {
                dw1[i] = dw1[i].addv(&_dw1[i]);
                db1[i] = db1[i] + _db1[i];
            }
            dw2 = numpy::add(&dw2, &_dw2);
            db2 = db2.addv(&_db2);
            dw3 = numpy::add(&dw3, &_dw3);
            db3 = db3.addv(&_db3);
        }

        let filter_w = network.layer1.get_dw();
        for i in 0..30 {
            network.layer1.bias[i] = network.layer1.bias[i] + db1[i] * (-0.001);
            dw1[i] = filter_w[i].addv(&dw1[i].muls(-0.001));
        }
        network.layer1.set_dw(&dw1);
        network.layer5.w = numpy::add(&network.layer5.w, &dw2.muls(-0.001));
        network.layer7.w = numpy::add(&network.layer7.w, &dw3.muls(-0.001));
        network.layer5.b = network.layer5.b.addv(&db2.muls(-0.001));
        network.layer7.b = network.layer7.b.addv(&db3.muls(-0.001));

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
