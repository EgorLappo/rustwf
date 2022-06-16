use rand::seq::SliceRandom;
use rand::prelude::*;

fn main() {
    run(wf, 500, 300, 0.5);
}

fn run(iteration: fn(&mut Vec<f64>), num_generations: usize, n: usize, p_init: f64) {
    let mut population = vec![0.0; n];

    let init_count = (p_init*n as f64) as usize;

    // fill the parental generation with values
    for i in 0..init_count {
        population[i] = 1.0;
    }

    for _ in 0..num_generations {
        iteration(&mut population);

        let mut s = 0.0;

        for i in 0..population.len() {
            s += population[i];
        }

        println!("{}", s/(n as f64));
    }
}

fn wf_classical(parents: &mut Vec<f64>, offspring: &mut Vec<f64>) {
    let mut rng = rand::thread_rng();
    for i in 0..parents.len() {
        offspring[i] = *parents.choose(&mut rng).unwrap();
    }
    parents.clone_from(&offspring);
}

fn binomial_sample(buffer: &mut Vec<f64>, p: f64) {
    let mut rng = rand::thread_rng();
    for i in 0..buffer.len() {
        let x: f64 = rng.gen();
        if x > p {
            buffer[i] = 0.0;
        } else {
            buffer[i] = 1.0;
        }
    }
}

fn wf(population: &mut Vec<f64>) {
    let p: f64 = population.iter().sum::<f64>()/population.len() as f64;
    println!("p is {}", p);
    binomial_sample(population, p);
}

