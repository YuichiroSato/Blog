#[derive(Debug)]
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

    pub fn from_vec(vec: Vec<Vec<f32>>) -> Self {
        NumpyArray {
            row: vec.len(),
            column: vec[0].len(),
            data: vec,
        }
    }

    pub fn row_vec(vec: Vec<f32>) -> Self {
        NumpyArray {
            row: 1,
            column: vec.len(),
            data: vec![vec],
        }
    }

    pub fn column_vec(vec: Vec<f32>) -> Self {
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

    pub fn transpose(&self) -> Self {
        let mut arr = NumpyArray::new(self.column, self.row);
        for r in 0..self.row {
            for c in 0..self.column {
                arr.data[c][r] = self.data[r][c];
            }
        }
        arr
    }

    pub fn adds(&self, val: f32) -> Self {
        let mut arr = NumpyArray::new(self.row, self.column);
        for r in 0..self.row {
            for c in 0..self.column {
                arr.data[r][c] = self.data[r][c] + val;
            }
        }
        arr
    }

    pub fn apply(&self, f: &Fn(f32) -> f32) -> Self {
        let mut arr = NumpyArray::new(self.row, self.column);
        for r in 0..self.row {
            for c in 0..self.column {
                arr.data[r][c] = (*f)(self.data[r][c]);
            }
        }
        arr
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

pub fn add(a1: &NumpyArray, a2: &NumpyArray) -> NumpyArray {
    if a1.row != a2.row || a1.column != a2.column {
        panic!("matrix dimension error: ({}, {}) + ({}, {}) is not defined", a1.row, a1.column, a2.row, a2.column);
    }

    let mut arr = NumpyArray::new(a1.row, a2.column);

    for c in 0..a1.column {
        for r in 0..a1.row {
            arr.data[r][c] = a1.data[r][c] + a2.data[r][c];
        }
    }

    arr
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

#[test]
fn new_test() {
    let a = NumpyArray::new(2, 3);
    assert_eq!(a.row, 2);
    assert_eq!(a.column, 3);
    assert_eq!(a.data, vec![vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0]]);
}

#[test]
fn from_vec_test() {
    let a = NumpyArray::from_vec(vec![vec![0.0, 0.0], vec![0.0, 0.0], vec![0.0, 0.0]]);
    assert_eq!(a.row, 3);
    assert_eq!(a.column, 2);
    assert_eq!(a.data, vec![vec![0.0, 0.0], vec![0.0, 0.0], vec![0.0, 0.0]]);
}

#[test]
fn column_vec_test() {
    let a = NumpyArray::column_vec(vec![0.0, 0.0, 0.0]);
    assert_eq!(a.row, 3);
    assert_eq!(a.column, 1);
    assert_eq!(a.data, vec![vec![0.0], vec![0.0], vec![0.0]]);
}

#[test]
fn row_vec_test() {
    let a = NumpyArray::row_vec(vec![0.0, 0.0, 0.0]);
    assert_eq!(a.row, 1);
    assert_eq!(a.column, 3);
    assert_eq!(a.data, vec![vec![0.0, 0.0, 0.0]]);
}

#[test]
fn transpose_test() {
    let a = NumpyArray::column_vec(vec![0.0, 0.0, 0.0]);
    let result = a.transpose();
    let expected = NumpyArray::row_vec(vec![0.0, 0.0, 0.0]);
    assert_eq!(expected.row, result.row);
    assert_eq!(expected.column, result.column);
    assert_eq!(expected.data, result.data);
}

#[test]
fn adds_test() {
    let a = NumpyArray::new(2, 3);
    let result = a.adds(2.0);

    assert_eq!(result.data, vec![vec![2.0, 2.0, 2.0], vec![2.0, 2.0, 2.0]]);
}

#[test]
fn apply_test() {
    let a = NumpyArray::new(2, 3);
    let result = a.apply(&|x| { x + 2.0});

    assert_eq!(result.data, vec![vec![2.0, 2.0, 2.0], vec![2.0, 2.0, 2.0]]);
}

#[test]
fn max_test() {
    let a = NumpyArray::row_vec(vec![2.0, 3.0, 0.0]);
    assert_eq!(3.0, a.max());
}

#[test]
fn sum_test() {
    let a = NumpyArray::row_vec(vec![1.0, 3.0, 2.0]);
    assert_eq!(6.0, a.sum());
}

#[test]
fn add_test() {
    let a = NumpyArray::from_vec(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let b = NumpyArray::from_vec(vec![vec![3.0, 2.0, 1.0], vec![6.0, 5.0, 4.0]]);
    let result = add(&a, &b);

    assert_eq!(result.data, vec![vec![4.0, 4.0, 4.0], vec![10.0, 10.0, 10.0]]);
}

#[test]
fn dot_test() {
    let a1 = NumpyArray::from_vec(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let b1 = NumpyArray::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]);
    let expected1 = NumpyArray::from_vec(vec![vec![22.0, 28.0], vec![49.0, 64.0]]);
    let result1 = dot(&a1, &b1);

    assert_eq!(expected1.row, result1.row);
    assert_eq!(expected1.column, result1.column);
    assert_eq!(expected1.data, result1.data);

    let a2 = NumpyArray::row_vec(vec![1.0, 2.0, 3.0]);
    let b2 = NumpyArray::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]);
    let expected2 = NumpyArray::from_vec(vec![vec![22.0, 28.0]]);

    let result2 = dot(&a2, &b2);

    assert_eq!(expected2.row, result2.row);
    assert_eq!(expected2.column, result2.column);
    assert_eq!(expected2.data, result2.data);

    let a3 = NumpyArray::from_vec(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let b3 = NumpyArray::column_vec(vec![1.0, 2.0, 3.0]);
    let expected3 = NumpyArray::from_vec(vec![vec![14.0], vec![32.0]]);

    let result3 = dot(&a3, &b3);

    assert_eq!(expected3.row, result3.row);
    assert_eq!(expected3.column, result3.column);
    assert_eq!(expected3.data, result3.data);
}
