mod optimizer;

use crate::optimizer::*;

fn main() {
    let mut rx = -7.0;
    let mut ry = 2.0;
    let mut rmsprop = RMSprop::new(0.01, 0.99);

    let mut ax = -7.0;
    let mut ay = 2.0;
    let mut adaGrad = AdaGrad::new(0.01);

    let mut mx = -7.0;
    let mut my = 2.0;
    let mut momentum = Momentum::new(0.01, 0.99);

    let mut sx = -7.0;
    let mut sy = 2.0;
    let mut sgd = Sgd::new(0.01);

    for _ in 0..10 {
        for _ in 0..200 {
            let (tem_rx, tem_ry) = rmsprop.optimize(rx, ry);
            rx = tem_rx;
            ry = tem_ry;
            let (tem_ax, tem_ay) = adaGrad.optimize(ax, ay);
            ax = tem_ax;
            ay = tem_ay;
            let (tem_mx, tem_my) = momentum.optimize(mx, my);
            mx = tem_mx;
            my = tem_my;
            let (tem_sx, tem_sy) = sgd.optimize(sx, sy);
            sx = tem_sx;
            sy = tem_sy;
        }
        println!("RMSprop: {} {}", rx, ry);
        println!("AdaGrad: {} {}", ax, ay);
        println!("Momentum: {} {}", mx, my);
        println!("SDG: {} {}", sx, sy);
        println!("");
    }
}
