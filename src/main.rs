use rand::prelude::*;

use sobol::params::JoeKuoD6;
use sobol::Sobol;

fn run_mk(n: usize) -> f64 {
    let mut rng = thread_rng();

    let mut counter: f64 = 0.0;
    for _ in 0..n {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();

        if x * x + y * y < 1.0 {
            counter += 1.0;
        }
    }

    4.0 * counter / (n as f64)
}

fn run_qmk(n: usize) -> f64 {
    let params = JoeKuoD6::minimal();
    let seq = Sobol::<f64>::new(2, &params);

    let mut counter: f64 = 0.0;
    for point in seq.take(n) {
        let x = point[0];
        let y = point[1];

        if x * x + y * y < 1.0 {
            counter += 1.0;
        }
    }

    4.0 * counter / (n as f64)
}

fn main() {
    println!("==============================");
    println!("===      Monte Carlo       ===");
    println!("==============================");
    for i in 1..11 {
        let pow = (10_i32).pow(i as u32) as usize;
        println!("N = 10^{}:  Pi = {}", i, run_mk(pow))
    }

    println!("\n");
    println!("=============================");
    println!("===   Quasi-Monte Carlo   ===");
    println!("=============================");
    for i in 1..11 {
        let pow = (10_i32).pow(i as u32) as usize;
        println!("N = 10^{}:  Pi = {}", i, run_qmk(pow))
    }
}
