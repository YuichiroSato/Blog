pub fn val(x: f32, y: f32) -> f32 {
    x * x / 20.0 + y * y
}

pub fn grad(x: f32, y: f32) -> (f32, f32) {
    ((x / 10.0), 2.0 * y)
}

pub trait Optimizer {
    fn optimize(&mut self, x: f32, y: f32) -> (f32, f32);
}

pub struct Sgd {
    lr: f32
}

impl Optimizer for Sgd {
    fn optimize(&mut self, x: f32, y: f32) -> (f32, f32) {
        let (dx, dy) = grad(x, y);
        (x - self.lr * dx, y - self.lr * dy)
    }
}

impl Sgd {
    pub fn new(lr: f32) -> Self {
        Sgd {
            lr: lr
        }
    }
}

pub struct Momentum {
    lr: f32,
    momentum: f32,
    buf_x: f32,
    buf_y: f32,
}

impl Optimizer for Momentum {
    fn optimize(&mut self, x: f32, y: f32) -> (f32, f32) {
        let (dx, dy) = grad(x, y);
        self.buf_x = self.momentum * self.buf_x - self.lr * dx;
        self.buf_y = self.momentum * self.buf_y - self.lr * dy;
        (x + self.buf_x, y + self.buf_y)
    }
}

impl Momentum {
    pub fn new(lr: f32, momentum: f32) -> Self {
        Momentum {
            lr: lr,
            momentum: momentum,
            buf_x: 0.0,
            buf_y: 0.0,
        }
    }
}

pub struct AdaGrad {
    lr: f32,
    buf_x: f32,
    buf_y: f32,
}

impl Optimizer for AdaGrad {
    fn optimize(&mut self, x: f32, y: f32) -> (f32, f32) {
        let (dx, dy) = grad(x, y);
        self.buf_x += dx * dx;
        self.buf_y += dy * dy;
        let _dx = self.lr * dx / (self.buf_x.sqrt() + 0.00001);
        let _dy = self.lr * dy / (self.buf_y.sqrt() + 0.00001);
        (x - _dx, y - _dy)
    }
}

impl AdaGrad {
    pub fn new(lr: f32) -> Self {
        AdaGrad {
            lr: lr,
            buf_x: 0.0,
            buf_y: 0.0,
        }
    }
}
