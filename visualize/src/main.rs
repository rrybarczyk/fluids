// pub(crate) use gnuplot::{Figure, Caption, Color, PointSymbol, PlotOption::PointSize};
mod common;
use crate::common::*;

fn set_points(zw: usize, zh: usize) -> std::vec::Vec<f64> {

    let mut z = Vec::with_capacity((zw * zh) as usize);

    for i in 0..zh {
        for j in 0..zw {
            let y = 8.0 * (i as f64) / zh as f64 - 4.0;
            let x = 8.0 * (j as f64) / zw as f64 - 4.0;
            z.push(x + y);
        }
    }

    dbg!(&z);
    z
}

// fn hills_points(m: usize, n: usize) -> Vec<f64> {
//     // radius
//     let a = 1.0;
//     let u0 = 10.0;
//     let A: f64 = 15.0 / 2.0 * u0 * 1.0 / ( a * a);
//     dbg!(A);
//
//     let mut z = Vec::with_capacity(( m * n) as usize);
//     z.push(-30.0 as f64);
//     z.push(-9.375 as f64);
//     z.push(0.0 as f64);
//     z.push(9.375 as f64);
//     z.push(30.0 as f64);
//
//     z
// }

fn linspace(lower: f64, upper: f64, nPoints: usize) -> Vec<f64> {
    let mut v = Vec::with_capacity(nPoints);
    let del: f64 = (lower.abs() + upper.abs()) / ((nPoints - 1) as f64);

    for i in 0..nPoints {
        v.push((lower + del * (i as f64)));
    }
    v
}


fn hills_points(m: usize, n: usize, resolution: usize) -> Vec<f64> {
    let mut z = Vec::with_capacity((m*n));

    let a = 1.0;
    let u0 = 10.0;
    let A: f64 = 15.0 / 2.0 * u0 * 1.0 / ( a * a);

    // lower = -m = -1, upper = n = 2
    let v = linspace(-(m as f64), n as f64, resolution);
    dbg!(&v);

    for i in v {
        let u_int = 1.0/5.0 * A * 1.0 * ( (a * a) - (i * i) - (2.0 * i * i) );
        dbg!(u_int);
        z.push(u_int);
    }

    // let u_int = 1.0/5.0 * A * 1.0 * ( (a * a) - (-1.0 * -1.0) - (2.0 * -1.0 * -1.0) );
    // z.push(u_int);
    // dbg!(u_int);
    // let u_int: f64 = 1.0/5.0 * A * 1.0 * ( (a * a) - (-0.25 * -0.25) - (2.0 * -0.25 * -0.25) );
    // z.push(u_int);
    // dbg!(u_int);
    // let u_int: f64 = 1.0/5.0 * A * 1.0 * ( (a * a) - (0.0 * 0.0) - (2.0 * 0.0 * 0.0) );
    // z.push(u_int);
    // dbg!(u_int);
    // let u_int: f64 = 1.0/5.0 * A * 1.0 * ( (a * a) - (0.25 * 0.25) - (2.0 * 0.25 * 0.25) );
    // z.push(u_int);
    // dbg!(u_int);
    // let u_int = 1.0/5.0 * A * 1.0 * ( (a * a) - (1.0 * 1.0) - (2.0 * 1.0 * 1.0) );
    // z.push(u_int);
    // dbg!(u_int);

    z
    // // let u_int = 1/5 * A * 1 * ( (a * a) - (1 * 1) - (2 * 1 * 1) );
    // //
    // for i in 1..=3 {
    //     for j in 1..=3 {
    //         let u_int = 1/5 * A * j * ( (a * a) - (i * i) - (2 * j * j) );
    //         dbg!(u_int);
    //         // let v_int = 1/5 * A * i * j^2;
    //         z.push(1.0 as f64);
    //     }
    // }
}

fn make_plot(z: std::vec::Vec<f64>, zw: usize, zh: usize) {

    let mut fig = Figure::new();
    fig.axes2d().set_title("Img fg2.10", &[]).image(z.iter(), zw, zh, Some((-10.0, -10.0, 10.0, 10.0)), &[]);

    fig.show();
}

fn main() {
    // z slice location
    let zw = 2;
    let zh = 2;
    let resolution = 20;

    println!("hi");
    // make_plot(set_points(zw, zh), zw, zh);
    // hills_points(zw, zh);
    make_plot(hills_points(zw, zh, resolution), zw, zh);
    println!("bye");
}
