#[derive(Debug, Clone)]
pub struct NumpyArray {
    pub row: usize,
    pub column: usize,
    pub data: Vec<Vec<f32>>,
}

impl NumpyArray {
    pub fn new(row_size: usize, column_size: usize) -> Self {
        NumpyArray {
            row: row_size,
            column: column_size,
            data: vec![vec![0.0; column_size]; row_size],
        }
    }

    pub fn from_vec(vec: &Vec<Vec<f32>>) -> Self {
        NumpyArray {
            row: vec.len(),
            column: vec[0].len(),
            data: vec.to_vec(),
        }
    }

    pub fn row_vec(vec: &Vec<f32>) -> Self {
        NumpyArray {
            row: 1,
            column: vec.len(),
            data: vec![vec.to_vec()],
        }
    }

    pub fn column_vec(vec: &Vec<f32>) -> Self {
        let mut arr = vec![vec![0.0]; vec.len()];
        for c in 0..vec.len() {
            arr[c][0] = vec[c];
        }
        NumpyArray {
            row: vec.len(),
            column: 1,
            data: arr,
        }
    }

    pub fn to_vector(&self) -> NumpyVector {
        if self.row != 1 && self.column != 1 {
            panic!("({}, {}) is not a vector", self.row, self.column);
        }

        let mut data = Vec::new();
        for r in 0..self.row {
            for c in 0..self.column {
                data.push(self.data[r][c]);
            }
        }
        NumpyVector {
            data: data
        }
    }

    pub fn flatten(&self) -> NumpyVector {
        let mut data = Vec::new();
        for r in 0..self.row {
            for c in 0..self.column {
                data.push(self.data[r][c]);
            }
        }
        NumpyVector {
            data: data
        }
    }

    pub fn transpose(&self) -> Self {
        let mut arr = NumpyArray::new(self.column, self.row);
        for r in 0..self.row {
            for c in 0..self.column {
                arr.data[c][r] = self.data[r][c];
            }
        }
        arr
    }

    pub fn adds(&mut self, val: f32) -> &mut NumpyArray {
        for r in 0..self.row {
            for c in 0..self.column {
                self.data[r][c] += val;
            }
        }
        self
    }

    pub fn muls(&mut self, val: f32) -> &mut NumpyArray {
        for r in 0..self.row {
            for c in 0..self.column {
                self.data[r][c] *= val;
            }
        }
        self
    }

    pub fn map(&mut self, f: &Fn(f32) -> f32) -> &mut NumpyArray {
        for r in 0..self.row {
            for c in 0..self.column {
                self.data[r][c] = (*f)(self.data[r][c]);
            }
        }
        self
    }

    pub fn max(&self) -> f32 {
        let mut x = std::f32::NEG_INFINITY;
        for r in 0..self.row {
            for c in 0..self.column {
                x = f32::max(x, self.data[r][c]);
            }
        }
        x
    }

    pub fn sum(&self) -> f32 {
        let mut x = 0.0;
        for r in 0..self.row {
            for c in 0..self.column {
                x += self.data[r][c];
            }
        }
        x
    }
}

pub fn add<'a>(a1: &'a mut NumpyArray, a2: &NumpyArray) -> &'a mut NumpyArray {
    if a1.row != a2.row || a1.column != a2.column {
        panic!("matrix dimension error: ({}, {}) + ({}, {}) is not defined", a1.row, a1.column, a2.row, a2.column);
    }

    for c in 0..a1.column {
        for r in 0..a1.row {
            a1.data[r][c] += a2.data[r][c];
        }
    }
    a1
}

pub fn sub<'a>(a1: &'a mut NumpyArray, a2: &NumpyArray) -> &'a mut NumpyArray {
    if a1.row != a2.row || a1.column != a2.column {
        panic!("matrix dimension error: ({}, {}) - ({}, {}) is not defined", a1.row, a1.column, a2.row, a2.column);
    }

    for c in 0..a1.column {
        for r in 0..a1.row {
            a1.data[r][c] -= a2.data[r][c];
        }
    }
    a1
}

pub fn dot(a1: &NumpyArray, a2: &NumpyArray) -> NumpyArray {
    if a1.column != a2.row {
        panic!("matrix dimension error: ({}, {}) * ({}, {}) is not defined", a1.row, a1.column, a2.row, a2.column);
    }

    let mut arr = NumpyArray::new(a1.row, a2.column);

    for c in 0..a2.column {
        for r in 0..a1.row {
            let mut val = 0.0;
            for i in 0..a1.column {
                val += a1.data[r][i] * a2.data[i][c];
            }
            arr.data[r][c] = val;
        }
    }

    arr
}

pub struct NumpyVector {
    pub data: Vec<f32>
}

impl NumpyVector {
    pub fn new(length: usize) -> Self {
        NumpyVector {
            data: vec![0.0; length]
        }
    }

    pub fn from_vec(data: &Vec<f32>) -> Self {
        NumpyVector {
            data: data.to_vec()
        }
    }

    pub fn to_row(&self) -> NumpyArray {
        NumpyArray::row_vec(&self.data.to_vec())
    }

    pub fn to_column(&self) -> NumpyArray {
        NumpyArray::column_vec(&self.data.to_vec())
    }

    pub fn addv(&mut self, a: &NumpyVector) -> &mut NumpyVector {
      if self.data.len() != a.data.len() {
        panic!("vector dimension error: [{}] + [{}] is not defined", self.data.len(), a.data.len());
      }

      for i in 0..self.data.len() {
        self.data[i] += a.data[i];
      }
      self
    }

    pub fn subv(&mut self, a: &NumpyVector) -> &mut NumpyVector {
      if self.data.len() != a.data.len() {
        panic!("vector dimension error: [{}] - [{}] is not defined", self.data.len(), a.data.len());
      }
      for i in 0..self.data.len() {
        self.data[i] -= a.data[i];
      }
      self
    }

    pub fn muls(&mut self, v: f32) -> &mut NumpyVector {
      for i in 0..self.data.len() {
        self.data[i] *= v;
      }
      self
    }

    pub fn map(&mut self, f: &Fn(f32) -> f32) -> &mut NumpyVector {
        for i in 0..self.data.len() {
            self.data[i] = (*f)(self.data[i]);
        }
        self
    }

    pub fn max(&self) -> f32 {
        let mut x = std::f32::NEG_INFINITY;
        for i in 0..self.data.len() {
            x = f32::max(x, self.data[i]);
        }
        x
    }

    pub fn sum(&self) -> f32 {
        let mut x = 0.0;
        for i in 0..self.data.len() {
            x += self.data[i];
        }
        x
    }

    pub fn argmax(&self) -> usize {
        let mut i = 0;
        let mut x = self.data[0];
        for j in 0..self.data.len() {
            if x < self.data[j] {
                x = self.data[j];
                i = j;
            }
        }
        i
    }
}

pub fn norm(a1: &NumpyVector, a2: &NumpyVector) -> f32 {
    if a1.data.len() != a2.data.len() {
        panic!("matrix dimension error: [{}] * [{}] is not defined", a1.data.len(), a2.data.len());
    }

    let mut val = 0.0;
    for i in 0..a2.data.len() {
        val += a1.data[i] * a2.data[i];
    }
    val
}

#[test]
fn new_test() {
    let a = NumpyArray::new(2, 3);
    assert_eq!(a.row, 2);
    assert_eq!(a.column, 3);
    assert_eq!(a.data, vec![vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0]]);
}

#[test]
fn from_vec_test() {
    let a = NumpyArray::from_vec(&vec![vec![0.0, 0.0], vec![0.0, 0.0], vec![0.0, 0.0]]);
    assert_eq!(a.row, 3);
    assert_eq!(a.column, 2);
    assert_eq!(a.data, vec![vec![0.0, 0.0], vec![0.0, 0.0], vec![0.0, 0.0]]);
}

#[test]
fn column_vec_test() {
    let a = NumpyArray::column_vec(&vec![0.0, 0.0, 0.0]);
    assert_eq!(a.row, 3);
    assert_eq!(a.column, 1);
    assert_eq!(a.data, vec![vec![0.0], vec![0.0], vec![0.0]]);
}

#[test]
fn row_vec_test() {
    let a = NumpyArray::row_vec(&vec![0.0, 0.0, 0.0]);
    assert_eq!(a.row, 1);
    assert_eq!(a.column, 3);
    assert_eq!(a.data, vec![vec![0.0, 0.0, 0.0]]);
}

#[test]
fn to_vector_test() {
    let a = NumpyArray::row_vec(&vec![0.0, 0.0, 0.0]);
    let result = a.to_vector();
    assert_eq!(result.data, vec![0.0, 0.0, 0.0]);
}

#[test]
fn transpose_test() {
    let a = NumpyArray::column_vec(&vec![0.0, 0.0, 0.0]);
    let result = a.transpose();
    let expected = NumpyArray::row_vec(&vec![0.0, 0.0, 0.0]);
    assert_eq!(expected.row, result.row);
    assert_eq!(expected.column, result.column);
    assert_eq!(expected.data, result.data);
}

#[test]
fn adds_test() {
    let mut a = NumpyArray::new(2, 3);
    let result = a.adds(2.0);

    assert_eq!(result.data, vec![vec![2.0, 2.0, 2.0], vec![2.0, 2.0, 2.0]]);
}

#[test]
fn muls_test() {
    let mut a = NumpyArray::new(2, 3);
    let result = a.adds(2.0).muls(2.0);

    assert_eq!(result.data, vec![vec![4.0, 4.0, 4.0], vec![4.0, 4.0, 4.0]]);
}

#[test]
fn map_test() {
    let mut a = NumpyArray::new(2, 3);
    let result = a.map(&|x| { x + 2.0});

    assert_eq!(result.data, vec![vec![2.0, 2.0, 2.0], vec![2.0, 2.0, 2.0]]);
}

#[test]
fn max_test() {
    let a = NumpyArray::row_vec(&vec![2.0, 3.0, 0.0]);
    assert_eq!(3.0, a.max());
}

#[test]
fn sum_test() {
    let a = NumpyArray::row_vec(&vec![1.0, 3.0, 2.0]);
    assert_eq!(6.0, a.sum());
}

#[test]
fn add_test() {
    let mut a = NumpyArray::from_vec(&vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let b = NumpyArray::from_vec(&vec![vec![3.0, 2.0, 1.0], vec![6.0, 5.0, 4.0]]);
    let result = add(&mut a, &b);

    assert_eq!(result.data, vec![vec![4.0, 4.0, 4.0], vec![10.0, 10.0, 10.0]]);
}

#[test]
fn sub_test() {
    let mut a = NumpyArray::from_vec(&vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let b = NumpyArray::from_vec(&vec![vec![3.0, 2.0, 1.0], vec![6.0, 5.0, 4.0]]);
    let result = sub(&mut a, &b);

    assert_eq!(result.data, vec![vec![-2.0, 0.0, 2.0], vec![-2.0, 0.0, 2.0]]);
}

#[test]
fn dot_test() {
    let a1 = NumpyArray::from_vec(&vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let b1 = NumpyArray::from_vec(&vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]);
    let expected1 = NumpyArray::from_vec(&vec![vec![22.0, 28.0], vec![49.0, 64.0]]);
    let result1 = dot(&a1, &b1);

    assert_eq!(expected1.row, result1.row);
    assert_eq!(expected1.column, result1.column);
    assert_eq!(expected1.data, result1.data);

    let a2 = NumpyArray::row_vec(&vec![1.0, 2.0, 3.0]);
    let b2 = NumpyArray::from_vec(&vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]);
    let expected2 = NumpyArray::from_vec(&vec![vec![22.0, 28.0]]);

    let result2 = dot(&a2, &b2);

    assert_eq!(expected2.row, result2.row);
    assert_eq!(expected2.column, result2.column);
    assert_eq!(expected2.data, result2.data);

    let a3 = NumpyArray::from_vec(&vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let b3 = NumpyArray::column_vec(&vec![1.0, 2.0, 3.0]);
    let expected3 = NumpyArray::from_vec(&vec![vec![14.0], vec![32.0]]);

    let result3 = dot(&a3, &b3);

    assert_eq!(expected3.row, result3.row);
    assert_eq!(expected3.column, result3.column);
    assert_eq!(expected3.data, result3.data);
}

#[test]
fn vector_new_test() {
    let v = NumpyVector::new(20);
    assert_eq!(v.data, vec![0.0; 20]);
}

#[test]
fn vector_from_vec_test() {
    let v = NumpyVector::from_vec(&vec![1.0, 2.0, 3.0]);
    assert_eq!(v.data, vec![1.0, 2.0, 3.0]);
}

#[test]
fn vector_to_row_test() {
    let v = NumpyVector::from_vec(&vec![1.0, 2.0, 3.0]);
    let result = v.to_row();
    let expected = NumpyArray::row_vec(&vec![1.0, 2.0, 3.0]);
    assert_eq!(expected.row, result.row);
    assert_eq!(expected.column, result.column);
    assert_eq!(expected.data, result.data);
}

#[test]
fn vector_to_column_test() {
    let v = NumpyVector::from_vec(&vec![1.0, 2.0, 3.0]);
    let result = v.to_column();
    let expected = NumpyArray::column_vec(&vec![1.0, 2.0, 3.0]);
    assert_eq!(expected.row, result.row);
    assert_eq!(expected.column, result.column);
    assert_eq!(expected.data, result.data);
}

#[test]
fn vector_addv_test() {
    let mut v1 = NumpyVector::from_vec(&vec![1.0, 2.0, 3.0]);
    let v2 = NumpyVector::from_vec(&vec![1.0, 1.0, 0.0]);
    let result = v1.addv(&v2);
    assert_eq!(result.data, vec![2.0, 3.0, 3.0]);
}

#[test]
fn vector_subv_test() {
    let mut v1 = NumpyVector::from_vec(&vec![1.0, 2.0, 3.0]);
    let v2 = NumpyVector::from_vec(&vec![1.0, 1.0, 0.0]);
    let result = v1.subv(&v2);
    assert_eq!(result.data, vec![0.0, 1.0, 3.0]);
}

#[test]
fn vector_muls_test() {
    let mut v1 = NumpyVector::from_vec(&vec![1.0, 2.0, 3.0]);
    let result = v1.muls(2.0);
    assert_eq!(result.data, vec![2.0, 4.0, 6.0]);
}

#[test]
fn vector_map_test() {
    let mut a = NumpyVector::new(3);
    let result = a.map(&|x| { x + 2.0});

    assert_eq!(result.data, vec![2.0, 2.0, 2.0]);
}

#[test]
fn vector_max_test() {
    let a = NumpyVector::from_vec(&vec![2.0, 3.0, 0.0]);
    assert_eq!(3.0, a.max());
}

#[test]
fn vector_sum_test() {
    let a = NumpyVector::from_vec(&vec![1.0, 3.0, 2.0]);
    assert_eq!(6.0, a.sum());
}

#[test]
fn vector_norm_test() {
    let v1 = NumpyVector::from_vec(&vec![1.0, 2.0, 3.0]);
    let v2 = NumpyVector::from_vec(&vec![1.0, 1.0, 0.0]);
    let result = norm(&v1, &v2);
    assert_eq!(result, 3.0);
}

#[test]
fn vector_argmax_test() {
    let a = NumpyVector::from_vec(&vec![2.0, 3.0, 0.0]);
    let result = a.argmax();
    assert_eq!(1, result);
}
