use crate::optimizer::*;
use crate::numpy;
use crate::numpy::*;

pub fn identity(x: &NumpyVector) -> &NumpyVector {
    x
}

pub fn step(x: &NumpyVector) -> NumpyVector {
    x.map(&|x| if x > 0.0 { 1.0 } else { 0.0 })
}

pub fn sigmoid(x: &NumpyVector) -> NumpyVector {
    x.map(&|x| 1.0 / (1.0 + f32::exp(x)))
}

pub fn relu(x: &NumpyVector) -> NumpyVector {
    x.map(&|x| if x > 0.0 { x } else { 0.0 })
}

pub fn array_relu(x: &NumpyArray) -> NumpyArray {
    x.map(&|x| if x > 0.0 { x } else { 0.0 })
}

pub fn softmax(x: &NumpyVector) -> NumpyVector {
    let c = x.max();
    let s = x.map(&|x| f32::exp(x - c)).sum();
    x.map(&|x| f32::exp(x - c) / s)
}

pub fn mean_squared_error(input: &NumpyVector, train: &NumpyVector) -> f32 {
    0.5 * input.subv(&train).map(&|x| x * x).sum()
}

pub fn cross_entropy_error(input: &NumpyVector, train: &NumpyVector) -> f32 {
    - numpy::norm(&train, &(input.map(&|x| (x + 0.0000001).ln())))
}

pub fn cross_entropy_error_batch(inputs: &Vec<NumpyVector>, trains: &Vec<NumpyVector>) -> f32 {
    let mut e = 0.0;
    for i in 0..inputs.len() {
        e += cross_entropy_error(&inputs[i], &trains[i]);
    }
    e / (inputs.len() as f32)
}

pub trait Layer {
    fn forward(&mut self, x: &NumpyVector) -> NumpyVector;
    fn backward(&mut self, x: &NumpyVector) -> NumpyVector;
    fn update(&mut self);
}

pub struct ReluLayer {
    mask: NumpyVector,
}

impl Layer for ReluLayer {
    fn forward(&mut self, x: &NumpyVector) -> NumpyVector {
        let out = relu(&x);
        self.mask = out.clone();
        out
    }

    fn backward(&mut self, x: &NumpyVector) -> NumpyVector {
        let mut dx = NumpyVector::new(x.data.len());
        for i in 0..dx.data.len() {
            dx.data[i] = if self.mask.data[i] > 0.0 {
                x.data[i]
            } else {
                0.0
            }
        }
        dx
    }

    fn update(&mut self) {}
}

impl ReluLayer {
    pub fn new() -> Self {
        ReluLayer {
            mask: NumpyVector::new(0),
        }
    }
}

pub struct SigmoidLayer {
    out: NumpyVector,
}

impl Layer for SigmoidLayer {
    fn forward(&mut self, x: &NumpyVector) -> NumpyVector {
        self.out = sigmoid(&x);
        self.out.clone()
    }

    fn backward(&mut self, x: &NumpyVector) -> NumpyVector {
        let one = NumpyVector::from_vec(&vec![1.0; x.data.len()]);
        self.out.muls(numpy::norm(&self.out, &one.subv(&x)))
    }

    fn update(&mut self) {}
}

impl SigmoidLayer {
    pub fn new() -> Self {
        SigmoidLayer {
            out: NumpyVector::new(0),
        }
    }
}

pub struct AffineLayer {
    pub w: NumpyArray,
    pub b: NumpyVector,
    dw: NumpyArray,
    db: NumpyVector,
    x: NumpyVector,
    w_optimizer: ArrayRMSprop,
    b_optimizer: VectorRMSprop,
}

impl Layer for AffineLayer {
    fn forward(&mut self, x: &NumpyVector) -> NumpyVector {
        self.x = x.clone();
        numpy::dot(&self.x.to_row(), &self.w).to_vector().addv(&self.b)
    }

    fn backward(&mut self, x: &NumpyVector) -> NumpyVector {
        let dx = numpy::dot(&x.to_row(), &self.w.transpose()).to_vector();
        let dw = numpy::dot(&self.x.to_column(), &x.to_row());
        let db = x.clone();
        self.dw = numpy::add(&dw, &self.dw);
        self.db = db.addv(&self.db);
        dx
    }

    fn update(&mut self) {
        self.w = self.w_optimizer.optimize(&self.w, &self.dw);
        self.b = self.b_optimizer.optimize(&self.b, &self.db);
        self.dw.zeros();
        self.db.zeros();
    }
}

impl AffineLayer {
    pub fn new(w: NumpyArray, b: NumpyVector) -> Self {
        let r = w.row.clone();
        let c = w.column.clone();
        let l = b.data.len();
        AffineLayer {
            w: w,
            b: b,
            dw: NumpyArray::new(r, c),
            db: NumpyVector::new(l),
            x: NumpyVector::new(0),
            w_optimizer: ArrayRMSprop::new(0.001, 0.99, r, c),
            b_optimizer: VectorRMSprop::new(0.001, 0.99, l),
        }
    }
}

pub trait LastLayer {
    fn forward(&mut self, x: &NumpyVector, t: &NumpyVector) -> f32;
    fn backward(&mut self, x: &NumpyVector) -> NumpyVector;
}

pub struct SoftmaxWithLossLayer {
    y: NumpyVector,
    t: NumpyVector,
}

impl LastLayer for SoftmaxWithLossLayer {
    fn forward(&mut self, x: &NumpyVector, t: &NumpyVector) -> f32 {
        self.t = t.clone();
        self.y = softmax(x);
        cross_entropy_error(&self.y, &self.t)
    }

    fn backward(&mut self, _: &NumpyVector) -> NumpyVector {
        self.y.subv(&self.t)
    }
}

impl SoftmaxWithLossLayer {
    pub fn new() -> Self {
        SoftmaxWithLossLayer {
            y: NumpyVector::new(0),
            t: NumpyVector::new(0),
        }
    }
}

pub trait CnnLayer {
    fn forward(&mut self, x: &Vec<NumpyArray>) -> Vec<NumpyArray>;
    fn backward(&mut self, x: &Vec<NumpyArray>) -> Vec<NumpyArray>;
    fn update(&mut self);
}

pub struct Filter {
    pub channel: usize,
    pub row_size: usize,
    pub column_size: usize,
    pub w: NumpyVector,
    padding: usize,
    slide: usize,
    input_row_size: usize,
    input_column_size: usize,
    pub out_row_size: usize,
    pub out_column_size: usize,
    col: NumpyArray,
    x: Vec<NumpyArray>,
    dw: NumpyVector,
    w_optimizer: VectorRMSprop,
}

impl Filter {
    pub fn new(w: &mut Vec<NumpyArray>, padding: usize, slide: usize, input_row_size: usize, input_column_size: usize) -> Self {
        let row_size = w[0].row;
        let column_size = w[0].column;
        let flatten_w = Filter::flatten(w);
        let flatten_w_len = flatten_w.data.len();
        Filter {
            channel: w.len(),
            row_size: row_size,
            column_size: column_size,
            w: flatten_w,
            padding: padding,
            slide: slide,
            input_row_size: input_row_size,
            input_column_size: input_column_size,
            out_row_size: 1 + ((input_row_size + 2 * padding - row_size) as f32 / slide as f32) as usize,
            out_column_size: 1 + ((input_column_size + 2 * padding - column_size) as f32 / slide as f32) as usize,
            col: NumpyArray::new(0, 0),
            x: Vec::new(),
            dw: NumpyVector::new(flatten_w_len),
            w_optimizer: VectorRMSprop::new(0.001, 0.99, flatten_w_len),
        }
    }

    pub fn flatten(w: &mut Vec<NumpyArray>) -> NumpyVector {
        let mut v = NumpyVector::new(0);
        for i in 0..w.len() {
            for r in 0..w[i].row {
                v.appendv(&mut w[i].data[r]);
            }
        }
        v
    }

    pub fn forward(&mut self, x: &Vec<NumpyArray>) -> NumpyArray {
        let col = Filter::im2col(self, x);
        let out = numpy::dot(&col, &self.w.to_column());
        self.col = col;
        self.x = x.to_vec();
        out
    }

    pub fn im2col(&self, x: &Vec<NumpyArray>) -> NumpyArray {
        let mut arr = vec![vec![0.0; 1]; self.out_row_size * self.out_column_size];

        let mut j = 0;
        // TODO padding, slide
        for r in 0..(self.input_row_size - self.row_size + 1) {
            for c in 0..(self.input_column_size - self.column_size + 1) {
                let mut v: Vec<f32> = Vec::new();
                for i in 0..x.len() {
                    for r2 in r..(r + self.row_size) {
                        for c2 in c..(c + self.column_size) {
                            v.push(x[i].data[r2][c2]);
                        }
                    }
                }
                arr[j] = v;
                j += 1;
            }
        }

        NumpyArray::from_vec(&arr)
    }

    pub fn backward(&mut self, x: &NumpyArray) -> Vec<NumpyArray> {
        let flat_x = Filter::flatten(&mut vec![x.clone()]);
        let dw = numpy::dot(&flat_x.to_row(), &self.col).to_vector();
        self.dw = dw.addv(&self.dw);
        Filter::col2im(self, &numpy::dot(&flat_x.to_column(), &self.w.to_row()))
    }

    pub fn col2im(&self, x: &NumpyArray) -> Vec<NumpyArray> {
        let mut v: Vec<NumpyArray> = Vec::new();
        let mutation = self.row_size * self.column_size;
        let divide_to_each_channels = x.divide_vertical(mutation);
        let tidle = self.input_row_size - self.row_size;

        for i in 0..divide_to_each_channels.len() {
            let mut arr = NumpyArray::new(self.input_row_size, self.input_column_size);
            let divided = &divide_to_each_channels[i];
            let mut base_row = 0;
            let mut base_column = 0;
            for j in 0..divided.row {
                let mut k = 0;
                for r in 0..self.row_size {
                    for c in 0..self.column_size {
                        arr.data[base_row + r][base_column + c] += divided.data[j][k];
                        k += 1;
                    }
                }
                base_row += 1;
                if base_row > tidle {
                    base_row = 0;
                    base_column += 1;
                }
            }
            v.push(arr);
        }

        v
    }

    pub fn update(&mut self) {
        self.w = self.w_optimizer.optimize(&self.w, &self.dw);
        self.dw.zeros();
    }
}

#[test]
fn filter_flatten_test() {
    let a1 = NumpyArray::from_vec(&vec![vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0]]);
    let a2 = NumpyArray::from_vec(&vec![vec![1.0, 1.0, 1.0],vec![0.0, 0.0, 0.0],vec![0.0, 0.0, 0.0]]);

    let result = Filter::flatten(&mut vec![a1, a2]);
    let expected = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

    assert!(result.data == expected);
}

#[test]
fn filter_forward_test() {
    let a1 = NumpyArray::from_vec(&vec![vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0]]);
    let a2 = NumpyArray::from_vec(&vec![vec![1.0, 1.0, 1.0],vec![0.0, 0.0, 0.0],vec![0.0, 0.0, 0.0]]);

    let mut f = Filter::new(&mut vec![a1, a2], 0, 1, 4, 4);
    let i1 = NumpyArray::from_vec(&vec![vec![1.0, 2.0, 3.0, 4.0],vec![5.0, 6.0, 7.0, 8.0],vec![9.0,  10.0, 11.0, 12.0], vec![13.0, 14.0, 15.0, 16.0]]);
    let i2 = NumpyArray::from_vec(&vec![vec![17.0, 18.0, 19.0, 20.0],vec![21.0, 22.0, 23.0, 24.0],vec![25.0,  26.0, 27.0, 28.0], vec![29.0, 30.0, 31.0, 32.0]]);

    let result = f.forward(&vec![i1, i2]);
    println!("{:?}", result.data);
    //assert!(false);
}

#[test]
fn filter_im2col_test() {
    let a1 = NumpyArray::from_vec(&vec![vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0]]);
    let a2 = NumpyArray::from_vec(&vec![vec![1.0, 1.0, 1.0],vec![0.0, 0.0, 0.0],vec![0.0, 0.0, 0.0]]);
    let f = Filter::new(&mut vec![a1, a2], 0, 1, 4, 4);

    let i1 = NumpyArray::from_vec(&vec![vec![1.0, 2.0, 3.0, 4.0],vec![5.0, 6.0, 7.0, 8.0],vec![9.0,  10.0, 11.0, 12.0], vec![13.0, 14.0, 15.0, 16.0]]);
    let i2 = NumpyArray::from_vec(&vec![vec![17.0, 18.0, 19.0, 20.0],vec![21.0, 22.0, 23.0, 24.0],vec![25.0,  26.0, 27.0, 28.0], vec![29.0, 30.0, 31.0, 32.0]]);

    let result = f.im2col(&vec![i1, i2]);
    //assert!(false);
}

#[test]
fn filter_col2im_test() {
    let a1 = NumpyArray::from_vec(&vec![vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0]]);
    let a2 = NumpyArray::from_vec(&vec![vec![1.0, 1.0, 1.0],vec![0.0, 0.0, 0.0],vec![0.0, 0.0, 0.0]]);
    let f = Filter::new(&mut vec![a1, a2], 0, 1, 4, 4);

    let i1 = NumpyArray::from_vec(&vec![vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]]);

    let result = f.col2im(&i1);

    let expected = vec![vec![1.0, 2.0, 2.0, 1.0], vec![2.0, 4.0, 4.0, 2.0], vec![2.0, 4.0, 4.0, 2.0], vec![1.0, 2.0, 2.0, 1.0]];

    assert!(result[0].data == expected);
    assert!(result[1].data == expected);
}

#[test]
fn filter_backword_test() {
    let a1 = NumpyArray::from_vec(&vec![vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0]]);
    let a2 = NumpyArray::from_vec(&vec![vec![1.0, 1.0, 1.0],vec![0.0, 0.0, 0.0],vec![0.0, 0.0, 0.0]]);

    let mut f = Filter::new(&mut vec![a1, a2], 0, 1, 4, 4);
    let i1 = NumpyArray::from_vec(&vec![vec![1.0, 2.0, 3.0, 4.0],vec![5.0, 6.0, 7.0, 8.0],vec![9.0,  10.0, 11.0, 12.0], vec![13.0, 14.0, 15.0, 16.0]]);
    let i2 = NumpyArray::from_vec(&vec![vec![17.0, 18.0, 19.0, 20.0],vec![21.0, 22.0, 23.0, 24.0],vec![25.0,  26.0, 27.0, 28.0], vec![29.0, 30.0, 31.0, 32.0]]);

    f.forward(&vec![i1, i2]);

    let d1 = NumpyArray::from_vec(&vec![vec![1.0], vec![1.0], vec![1.0], vec![1.0]]);

    let result = f.backward(&d1);

    let expected0 = vec![vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 0.0, 2.0, 2.0], vec![0.0, 0.0, 2.0, 2.0], vec![0.0, 0.0, 1.0, 1.0]];
    let expected1 = vec![vec![1.0, 2.0, 2.0, 1.0], vec![1.0, 2.0, 2.0, 1.0], vec![0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0]];

    assert!(result[0].data == expected0);
    assert!(result[1].data == expected1);
}

pub struct ConvolutionLayer {
    channel: usize,
    pub filters: Vec<Filter>,
    pub bias: Vec<f32>,
    input_row_size: usize,
    input_column_size: usize,
    buf_x: Vec<NumpyArray>,
    db: Vec<f32>,
    bias_optimizer: VecRMSprop,
}

impl CnnLayer for ConvolutionLayer {
    fn forward(&mut self, x: &Vec<NumpyArray>) -> Vec<NumpyArray> {
        if self.channel != x.len() {
            panic!("The input depth is {}, not {}", x.len(), self.channel);
        }

        let mut out = Vec::new();
        for i in 0..self.filters.len() {
            let r = self.filters[i].forward(&x).adds(self.bias[i]);
            out.push(r.to_vector().to_array(self.filters[i].out_column_size));
        }
        self.buf_x = x.clone();
        out
    }

    fn backward(&mut self, x: &Vec<NumpyArray>) -> Vec<NumpyArray> {
        let mut db2 = Vec::new();
        let mut dout = vec![NumpyArray::new(self.input_row_size, self.input_column_size); self.channel];
        for i in 0..x.len() {
            let douta = self.filters[i].backward(&x[i]);
            for j in 0..douta.len() {
                dout[j] = numpy::add(&dout[j], &douta[j]);
            }
            db2.push(x[i].sum());
        }
        for i in 0..db2.len() {
            self.db[i] += db2[i];
        }
        dout
    }

    fn update(&mut self) {
        for i in 0..self.filters.len() {
            self.filters[i].update();
        }
        self.bias = self.bias_optimizer.optimize(&self.bias, &self.db);
        for i in 0..self.db.len() {
            self.db[i] = 0.0;
        }
    }
}

impl ConvolutionLayer {
    pub fn new(channel: usize, filters: Vec<Filter>, bias: Vec<f32>, padding: usize, slide: usize, input_row_size: usize, input_column_size: usize) -> Self {
        for f in filters.iter() {
            if channel != f.channel {
                panic!("The filter depth is {}, not {}", f.channel, channel);
            }
        }

        let filters_len = filters.len();
        ConvolutionLayer {
            channel: channel,
            filters: filters,
            bias: bias,
            input_row_size: input_row_size,
            input_column_size: input_column_size,
            buf_x: vec![NumpyArray::new(0, 0)],
            db: vec![0.0; filters_len],
            bias_optimizer: VecRMSprop::new(0.001, 0.99, filters_len),
        }
    }
}

#[test]
fn convolution_forward_test() {
    let a1 = NumpyArray::from_vec(&vec![vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0]]);
    let a2 = NumpyArray::from_vec(&vec![vec![1.0, 1.0, 1.0],vec![0.0, 0.0, 0.0],vec![0.0, 0.0, 0.0]]);

    let f1 = Filter::new(&mut vec![a1], 0, 1, 4, 4);
    let f2 = Filter::new(&mut vec![a2], 0, 1, 4, 4);

    let i1 = NumpyArray::from_vec(&vec![vec![1.0, 2.0, 3.0, 4.0],vec![5.0, 6.0, 7.0, 8.0],vec![9.0,  10.0, 11.0, 12.0], vec![13.0, 14.0, 15.0, 16.0]]);

    let mut l = ConvolutionLayer::new(1, vec![f1, f2], vec![0.0, -1.0], 0, 1, 4, 4);
    let result = l.forward(&vec![i1]);
    println!("{}", result.len());
    println!("{:?}", result[0].data);
    println!("{:?}", result[1].data);
    //assert!(false);
}

#[test]
fn convolution_backward_test() {
    let a1 = NumpyArray::from_vec(&vec![vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0]]);
    let a2 = NumpyArray::from_vec(&vec![vec![1.0, 1.0, 1.0],vec![0.0, 0.0, 0.0],vec![0.0, 0.0, 0.0]]);

    let f1 = Filter::new(&mut vec![a1], 0, 1, 4, 4);
    let f2 = Filter::new(&mut vec![a2], 0, 1, 4, 4);

    let i1 = NumpyArray::from_vec(&vec![vec![1.0, 2.0, 3.0, 4.0],vec![5.0, 6.0, 7.0, 8.0],vec![9.0,  10.0, 11.0, 12.0], vec![13.0, 14.0, 15.0, 16.0]]);

    let mut l = ConvolutionLayer::new(1, vec![f1, f2], vec![0.0, -1.0], 0, 1, 4, 4);
    let _ = l.forward(&vec![i1]);

    let d1 = NumpyArray::from_vec(&vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let d2 = NumpyArray::from_vec(&vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
    l.backward(&vec![d1, d2]);
    l.update();
    //assert!(false);
}

#[test]
fn convolution_backward_test2() {
    let a1 = NumpyArray::from_vec(&vec![vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0],vec![0.0, 0.0, 1.0]]);
    let a2 = NumpyArray::from_vec(&vec![vec![1.0, 1.0, 1.0],vec![0.0, 0.0, 0.0],vec![0.0, 0.0, 0.0]]);
    let a3 = NumpyArray::from_vec(&vec![vec![0.0, 0.0, 0.0],vec![0.0, 0.0, 0.0],vec![0.0, 0.0, 1.0]]);
    let a4 = NumpyArray::from_vec(&vec![vec![1.0, 0.0, 0.0],vec![0.0, 0.0, 0.0],vec![0.0, 0.0, 0.0]]);

    let f1 = Filter::new(&mut vec![a1, a2], 0, 1, 4, 4);
    let f2 = Filter::new(&mut vec![a3, a4], 0, 1, 4, 4);

    let i1 = NumpyArray::from_vec(&vec![vec![1.0, 2.0, 3.0, 4.0],vec![5.0, 6.0, 7.0, 8.0],vec![9.0,  10.0, 11.0, 12.0], vec![13.0, 14.0, 15.0, 16.0]]);
    let i2 = NumpyArray::from_vec(&vec![vec![1.0, 2.0, 3.0, 4.0],vec![5.0, 6.0, 7.0, 8.0],vec![9.0,  10.0, 11.0, 12.0], vec![13.0, 14.0, 15.0, 16.0]]);

    let mut l = ConvolutionLayer::new(2, vec![f1, f2], vec![0.0, -1.0], 0, 1, 4, 4);
    let _ = l.forward(&vec![i1, i2]);

    let d1 = NumpyArray::from_vec(&vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let d2 = NumpyArray::from_vec(&vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
    let result = l.backward(&vec![d1, d2]);
    println!("{:?}", result[0].data);
    println!("{:?}", result[1].data);
    //assert!(false);
}

pub struct CnnReluLayer {
    mask: Vec<NumpyArray>,
}

impl CnnLayer for CnnReluLayer {
    fn forward(&mut self, x: &Vec<NumpyArray>) -> Vec<NumpyArray> {
        let out: Vec<NumpyArray> = x.iter()
            .map(|a| { array_relu(a) })
            .collect();
        self.mask = out.clone();
        out
    }

    fn backward(&mut self, x: &Vec<NumpyArray>) -> Vec<NumpyArray> {
        let mut dx_vec = Vec::new();
        for i in 0..x.len() {
            let mut dx = NumpyArray::new(x[i].row, x[i].column);
            for r in 0..x[i].row {
                for c in 0..x[i].column {
                    dx.data[r][c] = if self.mask[i].data[r][c] > 0.0 {
                        x[i].data[r][c]
                    } else {
                        0.0
                    }
                }
            }
            dx_vec.push(dx);
        }
        dx_vec
    }

    fn update(&mut self) {}
}

impl CnnReluLayer {
    pub fn new() -> Self {
        CnnReluLayer {
            mask: vec![NumpyArray::new(9, 9)],
        }
    }
}

pub struct MaxPoolingLayer {
    pooling_row_size: usize,
    pooling_column_size: usize,
    input_row_size: usize,
    input_column_size: usize,
    buf: Vec<Vec<(usize, usize)>>,
}

impl CnnLayer for MaxPoolingLayer {
    fn forward(&mut self, x: &Vec<NumpyArray>) -> Vec<NumpyArray> {
        let out_row_size = (self.input_row_size as f32 / self.pooling_row_size as f32) as usize;
        let out_column_size = (self.input_column_size as f32 / self.pooling_column_size as f32) as usize;

        let mut out = Vec::new();

        self.buf.clear();
        for i in 0..x.len() {
            let mut o = vec![vec![0.0; out_row_size]; out_column_size];
            let mut b = Vec::new();
            for r in 0..out_row_size {
                for c in 0..out_column_size {
                    let (val, max_indexs) = x[i].max_in(r * self.pooling_row_size, c * self.pooling_column_size, self.pooling_row_size, self.pooling_column_size);
                    o[r][c] = val;
                    b.push(max_indexs);
                }
            }
            self.buf.push(b);
            out.push(NumpyArray::from_vec(&o));
        }

        out
    }

    fn backward(&mut self, x: &Vec<NumpyArray>) -> Vec<NumpyArray> {
        let mut back = Vec::new();

        for i in 0..x.len() {
            let mut b = vec![vec![0.0; self.input_row_size]; self.input_column_size];
            let mut j = 0;
            for r in 0..x[i].row {
                for c in 0..x[i].column {
                    let (buf_r, buf_c) = self.buf[i][j];
                    b[buf_r][buf_c] = x[i].data[r][c];
                    j += 1;
                }
            }
            back.push(NumpyArray::from_vec(&b));
        }

        back
    }

    fn update(&mut self) {}
}

impl MaxPoolingLayer {
    pub fn new(pooling_row_size: usize, pooling_column_size: usize, input_row_size: usize, input_column_size: usize,
        ) -> Self {
        MaxPoolingLayer{
            pooling_row_size: pooling_row_size,
            pooling_column_size: pooling_column_size,
            input_row_size: input_row_size,
            input_column_size: input_column_size,
            buf: vec![vec![(0, 0)]],
        }
    }
}

pub struct FullConnector {
    row_size: usize,
    column_size: usize,
}

impl FullConnector {
    pub fn new(row_size: usize, column_size: usize) -> Self {
        FullConnector {
            row_size: row_size,
            column_size: column_size,
        }
    }

    pub fn forward(&self, x: &Vec<NumpyArray>) -> NumpyVector {
        let mut v = Vec::new();

        for i in 0..x.len() {
            for r in 0..x[i].row {
                for c in 0..x[i].column {
                    v.push(x[i].data[r][c]);
                }
            }
        }
        
        NumpyVector::from_vec(&v)
    }

    pub fn backward(&self, x: &NumpyVector) -> Vec<NumpyArray> {
        let mut v = Vec::new();
        let mut i = 0;

        while i < x.data.len() {
            let mut arr = vec![vec![0.0; self.row_size]; self.column_size];
            for r in 0..self.row_size {
                for c in 0..self.column_size {
                    arr[r][c] = x.data[i];
                    i += 1;
                }
            }
            v.push(NumpyArray::from_vec(&arr));
        }

        v
    }
}

#[test]
fn identity_test() {
    let x = NumpyVector::from_vec(&vec![1.0, 2.0, 3.0]);
    assert_eq!(x.data, identity(&x).data);
}

#[test]
fn step_test() {
    let x = NumpyVector::from_vec(&vec![1.0, -2.0, 3.0]);
    let result = step(&x);
    let expected = vec![1.0, 0.0, 1.0];
    assert_eq!(expected, result.data);
}

#[test]
fn sigmoid_test() {
    let x = NumpyVector::from_vec(&vec![0.0, 0.0, 0.0]);
    let result = sigmoid(&x);
    let expected = vec![0.5, 0.5, 0.5];
    assert_eq!(expected, result.data);
}

#[test]
fn relu_test() {
    let x = NumpyVector::from_vec(&vec![1.0, -2.0, 3.0]);
    let result = relu(&x);
    let expected = vec![1.0, 0.0, 3.0];
    assert_eq!(expected, result.data);
}

#[test]
fn softmax_test() {
    let x = NumpyVector::from_vec(&vec![0.0, 1.0, 2.0]);
    let result = softmax(&x);

    println!("{:?}", result.data);
    assert!(result.data[0] > 0.0 && result.data[0] < 0.1);
    assert!(result.data[1] > 0.2 && result.data[1] < 0.3);
    assert!(result.data[2] > 0.66 && result.data[2] < 0.67);
}

#[test]
fn mean_squared_error_test() {
    let input = NumpyVector::from_vec(&vec![1.0, 2.0, 2.0, 3.0]);
    let train = NumpyVector::from_vec(&vec![3.0, 2.0, 2.0, 1.0]);
    let result = mean_squared_error(&input, &train);
    assert_eq!(4.0, result);
}

#[test]
fn cross_entropy_error_test() {
    let input = NumpyVector::from_vec(&vec![0.0, 0.6, 0.1, 0.3]);
    let train = NumpyVector::from_vec(&vec![0.0, 1.0, 0.0, 0.0]);
    let result = cross_entropy_error(&input, &train);
    assert!(result > 0.510 && result < 0.511);
}

#[test]
fn cross_entropy_error_batch_test() {
    let input1 = NumpyVector::from_vec(&vec![0.0, 0.6, 0.1, 0.3]);
    let train1 = NumpyVector::from_vec(&vec![0.0, 1.0, 0.0, 0.0]);
    let input2 = NumpyVector::from_vec(&vec![0.0, 0.6, 0.1, 0.3]);
    let train2 = NumpyVector::from_vec(&vec![0.0, 1.0, 0.0, 0.0]);
    let result = cross_entropy_error_batch(&vec![input1, input2], &vec![train1, train2]);
    assert!(result > 0.510 && result < 0.511);
}

#[test]
fn relu_layer_test() {
    let x = NumpyVector::from_vec(&vec![0.1, -0.2, 0.3]);
    let d = NumpyVector::from_vec(&vec![0.3, 0.2, 0.2]);
    let mut layer = ReluLayer::new();

    let out = layer.forward(&x);
    let dx = layer.backward(&d);

    let expected_out = vec![0.1, 0.0, 0.3];
    let expected_dx = vec![0.3, 0.0, 0.2];

    assert_eq!(expected_out, out.data);
    assert_eq!(expected_dx, dx.data);
}

#[test]
fn sigmoid_layer_test() {
    let x = NumpyVector::from_vec(&vec![0.1, 0.2, 0.3]);
    let d = NumpyVector::from_vec(&vec![0.1, 0.2, 0.3]);
    let mut layer = SigmoidLayer::new();

    let out = layer.forward(&x);
    let dx = layer.backward(&d);

    assert!(true);
}

#[test]
fn affine_layer_test() {
    let w = NumpyArray::from_vec(&vec![vec![0.1, 0.0, 0.1], vec![0.0, 0.0, 0.1]]);
    let b = NumpyVector::from_vec(&vec![-0.2, -0.2, -0.2]);
    let x = NumpyVector::from_vec(&vec![1.0, 2.0]);
    let d = NumpyVector::from_vec(&vec![0.1, 0.2, 0.3]);
    let mut layer = AffineLayer::new(w, b);

    let out = layer.forward(&x);
    let dx = layer.backward(&d);

    let expected_dw = vec![vec![0.1, 0.2, 0.3], vec![0.2, 0.4, 0.6]];
    let expected_db = vec![0.1, 0.2, 0.3];

    assert!(out.data[0] > -0.101 && out.data[0] < -0.099);
    assert!(out.data[1] > -0.201 && out.data[1] < -0.199);
    assert!(out.data[2] < 0.101 && out.data[2] > 0.099);

    assert_eq!(expected_dw, layer.dw.data);
    assert_eq!(expected_db, layer.db.data);

    assert!(dx.data[0] > 0.0399 && dx.data[0] < 0.0401);
    assert!(dx.data[1] > 0.0299 && dx.data[1] < 0.0301);
}

#[test]
fn softmax_with_loss_layer_test() {
    let x = NumpyVector::from_vec(&vec![0.1, 0.2, 0.3]);
    let t = NumpyVector::from_vec(&vec![0.0, 2.0, 0.0]);
    let d = NumpyVector::from_vec(&vec![0.1, 0.2, 0.3]);
    let mut layer = SoftmaxWithLossLayer::new();

    let out = layer.forward(&x, &t);
    let dx = layer.backward(&d);

    assert!(true);
}

#[test]
fn cnn_relu_layer_test() {
    let mut l = CnnReluLayer::new();
    let i1 = NumpyArray::from_vec(
        &vec![
            vec![-1.0, 2.0],
            vec![-1.0, 1.0],
        ]);
    let i2 = NumpyArray::from_vec(
        &vec![
            vec![1.0, -2.0],
            vec![-1.0, 1.0],
        ]);
    let expected_f1 = vec![
            vec![0.0, 2.0],
            vec![0.0, 1.0],
        ];
    let expected_f2 =vec![
            vec![1.0, 0.0],
            vec![0.0, 1.0],
        ];
    let d1 = NumpyArray::from_vec(
        &vec![
            vec![6.0, 7.0],
            vec![8.0, 9.0],
        ]);
    let d2 = NumpyArray::from_vec(
        &vec![
            vec![4.0, 3.0],
            vec![2.0, 1.0],
        ]);
    let expected_b1 =
        vec![
            vec![0.0, 7.0],
            vec![0.0, 9.0],
        ];
    let expected_b2 =
        vec![
            vec![4.0, 0.0],
            vec![0.0, 1.0],
        ];
    let result_f = l.forward(&vec![i1, i2]);
    let result_b = l.backward(&vec![d1, d2]);

    assert_eq!(expected_f1, result_f[0].data);
    assert_eq!(expected_f2, result_f[1].data);
    assert_eq!(expected_b1, result_b[0].data);
    assert_eq!(expected_b2, result_b[1].data);
}

#[test]
fn max_pooling_layer_test() {
    let mut l = MaxPoolingLayer::new(2, 2, 4, 4);
    let i1 = NumpyArray::from_vec(
        &vec![
            vec![1.0, 2.0, 3.0, 2.0],
            vec![1.0, 1.0, 3.0, 5.0],
            vec![1.0, 2.0, 3.0, 5.0],
            vec![5.0, 3.0, 2.0, 4.0],
        ]);
    let i2 = NumpyArray::from_vec(
        &vec![
            vec![1.0, 1.0, 3.0, 5.0],
            vec![1.0, 2.0, 3.0, 2.0],
            vec![1.0, 2.0, 4.0, 3.0],
            vec![5.0, 3.0, 2.0, 5.0],
        ]);
    let expected_f1 = vec![
            vec![2.0, 5.0],
            vec![5.0, 5.0],
        ];
    let expected_f2 =vec![
            vec![2.0, 5.0],
            vec![5.0, 5.0],
        ];
    let d1 =  NumpyArray::from_vec(
        &vec![
            vec![6.0, 7.0],
            vec![8.0, 9.0],
        ]);
    let d2 =  NumpyArray::from_vec(
        &vec![
            vec![4.0, 3.0],
            vec![2.0, 1.0],
        ]);
    let expected_b1 =
        vec![
            vec![0.0, 6.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 7.0],
            vec![0.0, 0.0, 0.0, 9.0],
            vec![8.0, 0.0, 0.0, 0.0],
        ];
    let expected_b2 =
        vec![
            vec![0.0, 0.0, 0.0, 3.0],
            vec![0.0, 4.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0],
            vec![2.0, 0.0, 0.0, 1.0],
        ];
    let result_f = l.forward(&vec![i1, i2]);
    let result_b = l.backward(&vec![d1, d2]);

    assert_eq!(expected_f1, result_f[0].data);
    assert_eq!(expected_f2, result_f[1].data);
    assert_eq!(expected_b1, result_b[0].data);
    assert_eq!(expected_b2, result_b[1].data);
}

#[test]
fn full_connector_test() {
    let l = FullConnector {
        row_size: 4,
        column_size: 4,
    };

    let i1 = NumpyArray::from_vec(
        &vec![
            vec![1.0, 2.0, 3.0, 2.0],
            vec![1.0, 1.0, 3.0, 5.0],
            vec![1.0, 2.0, 3.0, 5.0],
            vec![5.0, 3.0, 2.0, 4.0],
        ]);
    let i2 = NumpyArray::from_vec(
        &vec![
            vec![1.0, 1.0, 3.0, 5.0],
            vec![1.0, 2.0, 3.0, 2.0],
            vec![1.0, 2.0, 4.0, 3.0],
            vec![5.0, 3.0, 2.0, 5.0],
        ]);
    let d = NumpyVector::from_vec(&vec![
        1.0, 2.0, 3.0, 2.0,
        1.0, 1.0, 3.0, 5.0,
        1.0, 2.0, 3.0, 5.0,
        5.0, 3.0, 2.0, 4.0,
        1.0, 1.0, 3.0, 5.0,
        1.0, 2.0, 3.0, 2.0,
        1.0, 2.0, 4.0, 3.0,
        5.0, 3.0, 2.0, 5.0,
    ]);
    let result_f = l.forward(&vec![i1.clone(), i2.clone()]);
    let result_b = l.backward(&d);

    assert_eq!(d.data, result_f.data);
    assert_eq!(i1.data, result_b[0].data);
}
