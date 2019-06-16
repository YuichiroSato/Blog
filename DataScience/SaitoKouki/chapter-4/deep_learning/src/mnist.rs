use crate::numpy::*;
use std::fs::File;
use std::io;
use std::io::Read;

pub fn load_mnist() -> io::Result<((Vec<NumpyArray>, Vec<u8>), (Vec<NumpyArray>, Vec<u8>))> {
    let train_images = load_images("../../mnist/train-images-idx3-ubyte")?;
    let train_labels = load_labels("../../mnist/train-labels-idx1-ubyte")?;
    let test_images = load_images("../../mnist/t10k-images-idx3-ubyte")?;
    let test_labels = load_labels("../../mnist/t10k-labels-idx1-ubyte")?;
    Ok(((train_images, train_labels), (test_images, test_labels)))
}

fn load_images(path: &str) -> io::Result<Vec<NumpyArray>> {
    let mut file = File::open(&path)?;

    let mut magic_number_bytes = [0u8; 4];
    let mut number_of_items_bytes = [0u8; 4];
    let mut number_of_rows_bytes = [0u8; 4];
    let mut number_of_columns_bytes = [0u8; 4];

    file.read(&mut magic_number_bytes)?;
    file.read(&mut number_of_items_bytes)?;
    file.read(&mut number_of_rows_bytes)?;
    file.read(&mut number_of_columns_bytes)?;

    let _magic_number = to_i32(&magic_number_bytes); 
    let number_of_items = to_i32(&number_of_items_bytes);
    let _number_of_rows = to_i32(&number_of_rows_bytes);
    let _number_of_columns = to_i32(&number_of_columns_bytes);

    let mut result: Vec<NumpyArray> = Vec::new();
    let mut buf = [0u8; 28];
    for _ in 0..number_of_items {
        let mut data: Vec<Vec<f32>> = Vec::new();
        for _ in 0..28 {
            file.read(&mut buf)?;
            let mut row: Vec<f32> = Vec::new();
            for i in 0..28 {
                row.push(buf[i] as f32);
            }
            data.push(row);
        }
        result.push(NumpyArray::from_vec(&data));
    }
    Ok(result)
}

fn load_labels(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(&path)?;

    let mut magic_number_bytes = [0u8; 4];
    let mut number_of_items_bytes = [0u8; 4];

    file.read(&mut magic_number_bytes)?;
    file.read(&mut number_of_items_bytes)?;

    let _magic_number = to_i32(&magic_number_bytes); 
    let number_of_items = to_i32(&number_of_items_bytes);

    let mut result: Vec<u8> = Vec::new();
    let mut buf = [0u8; 1];
    for _ in 0..number_of_items {
        file.read(&mut buf)?;
        result.push(buf[0]);
    }
    Ok(result)
}

fn to_i32(bs: &[u8; 4]) -> i32 {
    (((bs[0] as u32) << 24) + ((bs[1] as u32) << 16) + ((bs[2] as u32) << 8) + ((bs[3] as u32) << 0)) as i32
}

fn print_image(image: &NumpyArray) {
    for r in 0..image.row {
        for c in 0..image.column {
            if image.data[r][c] > 0.0 {
                print!("*");
            } else {
                print!("_");
            }
        }
        println!("");
    }
}
