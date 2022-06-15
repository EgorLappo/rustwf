use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {

    let num_generations: usize  = 100;
    let n: usize = 300;
    let p_init: f64 = 0.5;
    
    //initialize the two main arrays for the simulation
    let mut parents = vec![0.0; n];
    let mut offspring = vec![0.0; n];

    let init_count = (p_init*n as f64) as usize;

    // fill the parental generation with values
    for i in 0..init_count {
        parents[i] = 1.0;
    }

    for _ in 0..num_generations {
        generation(&mut parents, &mut offspring);

        let mut s = 0.0;

        for i in 0..parents.len() {
            s += parents[i];
        }

        println!("{}", s/(n as f64));
    }
}

fn generation(parents: &mut Vec<f64>, offspring: &mut Vec<f64>) {
    let mut rng = thread_rng();
    for i in 0..parents.len() {
        offspring[i] = *parents.choose(&mut rng).unwrap();
    }
    parents.clone_from(&offspring);
}

