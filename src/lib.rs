pub mod sim {
    use rand::{Rng, SeedableRng};
    use rand::rngs::SmallRng;
    use std::path::PathBuf;

    pub fn run(iteration: Box<dyn Fn(&mut Vec<f64>, &mut SmallRng) -> f64>, num_generations: usize, n: usize, p_init: f64, seed: u64) {
        let mut population = vec![0.0; n];
        let mut result = vec![0.0; num_generations];
    
        let init_count = (p_init*n as f64) as usize;
    
        // fill the parental generation with values
        for i in 0..init_count {
            population[i] = 1.0;
        }
        // seed the RNG
        let mut rng = SmallRng::seed_from_u64(seed);
    
        // *main loop*:
        // at each step, record the current state of the population into the result vector,
        // and produce the next generation
        for i in 0..num_generations {
            result[i] = iteration(&mut population, &mut rng);
        }
    }

    fn binomial_sample(buffer: &mut Vec<f64>, p: f64, rng: &mut SmallRng) {
        for i in 0..buffer.len() {
            let x: f64 = rng.gen();
            if x > p {
                buffer[i] = 0.0;
            } else {
                buffer[i] = 1.0;
            }
        }
    }

    pub fn wf() -> Box<dyn Fn(&mut Vec<f64>, &mut SmallRng) -> f64>  {
        Box::new(|population: &mut Vec<f64>, rng: &mut SmallRng| {
            let p: f64 = population.iter().sum::<f64>()/population.len() as f64;
            binomial_sample(population, p, rng);
            p
        }
        )
    }

    pub fn wf_selection(s: f64, h: f64) -> Box<dyn Fn(&mut Vec<f64>, &mut SmallRng) -> f64>  {
        Box::new(move |population: &mut Vec<f64>, rng: &mut SmallRng| {
            let p: f64 = population.iter().sum::<f64>()/population.len() as f64;
            let p_next = p + p*(1.-p)*((1.+s)*p+(1.+h*s)*(1.-2.*p) - (1.-p))/((1.+s)*p*p + 2.*(1.+h*s)*p*(1.-p)+ (1.-p)*(1.-p));
            binomial_sample(population, p_next, rng);
            p
        })
    }

    pub fn fpm_conformity_3rm(s: f64, d: f64) -> Box<dyn Fn(&mut Vec<f64>, &mut SmallRng) -> f64>  {
        Box::new(move |population: &mut Vec<f64>, rng: &mut SmallRng| {
            let p: f64 = population.iter().sum::<f64>()/population.len() as f64;
            let p_next = (1.+s)*(p + d*p*(1.-p)*(2.*p-1.))/(1.+s*(p + d*p*(1.-p)*(2.*p-1.)));
            binomial_sample(population, p_next, rng);
            p
        })
    }

    pub fn fpm_conformity_5rm(s: f64, d3: f64, d4: f64) -> Box<dyn Fn(&mut Vec<f64>, &mut SmallRng) -> f64>  {
        Box::new(move |population: &mut Vec<f64>, rng: &mut SmallRng| {
            let p: f64 = population.iter().sum::<f64>()/population.len() as f64;
            let p_next = (1.+s)*(p+(1.-p)*p*(2.*p-1.)*(d4-p*(1.-p)*(d4-2.*d3)))/(1.+s*(p+(1.-p)*p*(2.*p-1.)*(d4-p*(1.-p)*(d4-2.*d3))));
            binomial_sample(population, p_next, rng);
            p
        })
    }
}

pub mod manager {
    use rand::{Rng, SeedableRng};
    use rand::rngs::SmallRng;
    use std::path::PathBuf;

    use crate::sim::run;

    pub fn launch(iteration: Box<dyn Fn(&mut Vec<f64>, &mut SmallRng) -> f64>, num_generations: usize, n: usize, num_rep: usize, p_init: f64, output_folder: &PathBuf, num_threads: usize, seed: u64) {
        run(iteration, num_generations, n, p_init, seed);
    }
}