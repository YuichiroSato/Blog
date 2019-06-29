mod optimizer;

use crate::optimizer::*;

fn main() {
    let mut x = -7.0;
    let mut y = 2.0;
    let mut optimizer = AdaGrad::new(0.1);
    //let mut optimizer = Momentum::new(0.1, 0.9);
    //let mut optimizer = Sgd::new(0.1);
    for _ in 0..5000 {
        let (tmpx, tmpy) = optimizer.optimize(x, y);
        x = tmpx;
        y = tmpy;
        println!("{} {}", x, y);
    }
}
