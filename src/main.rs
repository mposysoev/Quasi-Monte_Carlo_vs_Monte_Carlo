use rand::prelude::*;

use sobol::params::JoeKuoD6;
use sobol::Sobol;

fn run_mk(N: usize) -> f64 {
    let mut rng = thread_rng();

    let mut counter = 0;
    for _ in 0..N {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();

        if x * x + y * y < 1.0 {
            counter += 1;
        }
    }

    return 4.0 * (counter as f64) / (N as f64);
}

fn run_qmk(N: usize) -> f64 {
    let params = JoeKuoD6::minimal();
    let seq = Sobol::<f64>::new(2, &params);

    let mut counter = 0;
    for point in seq.take(N) {
        let x = point[0];
        let y = point[1];

        if x * x + y * y < 1.0 {
            counter += 1;
        }
    }

    return 4.0 * (counter as f64) / (N as f64);
}

fn main() {

    println!("===============================");
    println!("===   Обычный Монте-Карло   ===");
    println!("===============================");
    for i in 1..11 { 
        let pow = (10 as i32).pow(i as u32) as usize; 
        println!("N = 10^{}:  Pi = {}", i, run_mk(pow))
    }

    println!("\n");
    println!("==============================");
    println!("===   Квази  Монте-Карло   ===");
    println!("==============================");
    for i in 1..11 { 
        let pow = (10 as i32).pow(i as u32) as usize; 
        println!("N = 10^{}:  Pi = {}", i, run_qmk(pow))
    }

}
