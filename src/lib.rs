pub mod sim {
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::error::Error;

    use rand::{Rng, SeedableRng};
    use rand::rngs::SmallRng;
    use csv::Writer;    

    /// function to run a single simulation in a single thread
    pub fn run(iteration: &Box<dyn Fn(&mut Vec<f64>, &mut SmallRng) -> f64  + Sync + Send>, num_generations: usize, n: usize, p_init: f64, seed: u64, output_folder: &PathBuf) {
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

        write_result(result, seed, output_folder).expect("Unable to write the simulation with id {seed} to .csv!");
    }

    // run simulation only to fixation
    pub fn run_fix(iteration: &Box<dyn Fn(&mut Vec<f64>, &mut SmallRng) -> f64  + Sync + Send>, num_generations: usize, n: usize, p_init: f64, seed: u64, output_folder: &PathBuf) {
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
            // if we simulate to fixation, stop here and truncate the result vector
            if (result[i] <= 0.0) || result[i] >= 1.0 {
                result = result[0..i].to_vec();
                break;
            }
        }

        write_result(result, seed, output_folder).expect("Unable to write the simulation with id {seed} to .csv!");
    }

    /// wrapper functon to do make the types match in the parallel case
    pub fn run_arc(iteration: Arc<Box<dyn Fn(&mut Vec<f64>, &mut SmallRng) -> f64  + Sync + Send>>, num_generations: usize, n: usize, p_init: f64, to_fixation: bool, seed: u64, output_folder: &PathBuf) {
        if to_fixation {
            run(&*iteration, num_generations, n, p_init, seed, output_folder);
        } else {
            run_fix(&*iteration, num_generations, n, p_init, seed, output_folder);
        }
    }
 
    /// writes the result to a given folder as a .csv with two columns; the filename is given by the simulation seed
    fn write_result(result: Vec<f64>, seed: u64, output_folder: &PathBuf) -> Result<(), Box<dyn Error>> {
        let mut filename = output_folder.clone();
        filename.push(seed.to_string());
        filename.set_extension("csv");

        let mut wtr = Writer::from_path(filename)?;
        wtr.write_record(&["generation", "frequency"])?;

        for (i, x) in result.iter().enumerate() {
            wtr.write_record(&[(i+1).to_string(), x.to_string()])?;
        }
        wtr.flush()?;
        Ok(())
    }

    /// simple binomial sampler that writes the results in a given buffer
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

    // ----------- iteration functions -----------

    /// standard (netural) Wrght-Fisher simulation, see Ewens (2004) book, chapters 2, 3
    pub fn wf() -> Box<dyn Fn(&mut Vec<f64>, &mut SmallRng) -> f64  + Sync + Send>  {
        Box::new(|population: &mut Vec<f64>, rng: &mut SmallRng| {
            let p: f64 = population.iter().sum::<f64>()/population.len() as f64;
            binomial_sample(population, p, rng);
            p
        }
        )
    }

    /// Wrght-Fisher simulation with selection, see Ewens (2004) book, chapters 2, 3
    pub fn wf_selection(s: f64, h: f64) -> Box<dyn Fn(&mut Vec<f64>, &mut SmallRng) -> f64  + Sync + Send>  {
        Box::new(move |population: &mut Vec<f64>, rng: &mut SmallRng| {
            let p: f64 = population.iter().sum::<f64>()/population.len() as f64;
            let p_next = p + p*(1.-p)*((1.+s)*p+(1.+h*s)*(1.-2.*p) - (1.-p))/((1.+s)*p*p + 2.*(1.+h*s)*p*(1.-p)+ (1.-p)*(1.-p));
            binomial_sample(population, p_next, rng);
            p
        })
    }

    /// FPM of conformist transition, n=3, see Lappo, Denton, and Feldman (2022)
    pub fn fpm_conformity_3rm(s: f64, d: f64) -> Box<dyn Fn(&mut Vec<f64>, &mut SmallRng) -> f64  + Sync + Send>  {
        Box::new(move |population: &mut Vec<f64>, rng: &mut SmallRng| {
            let p: f64 = population.iter().sum::<f64>()/population.len() as f64;
            let p_next = (1.+s)*(p + d*p*(1.-p)*(2.*p-1.))/(1.+s*(p + d*p*(1.-p)*(2.*p-1.)));
            binomial_sample(population, p_next, rng);
            p
        })
    }

    /// FPM of conformist transition, n=5, see Lappo, Denton, and Feldman (2022)
    pub fn fpm_conformity_5rm(s: f64, d3: f64, d4: f64) -> Box<dyn Fn(&mut Vec<f64>, &mut SmallRng) -> f64  + Sync + Send>  {
        Box::new(move |population: &mut Vec<f64>, rng: &mut SmallRng| {
            let p: f64 = population.iter().sum::<f64>()/population.len() as f64;
            let p_next = (1.+s)*(p+(1.-p)*p*(2.*p-1.)*(d4-p*(1.-p)*(d4-2.*d3)))/(1.+s*(p+(1.-p)*p*(2.*p-1.)*(d4-p*(1.-p)*(d4-2.*d3))));
            binomial_sample(population, p_next, rng);
            p
        })
    }

    pub fn fds_linear(a: f64, b: f64) -> Box<dyn Fn(&mut Vec<f64>, &mut SmallRng) -> f64  + Sync + Send> {
        Box::new(move |population: &mut Vec<f64>, rng: &mut SmallRng| {
            let p: f64 = population.iter().sum::<f64>()/population.len() as f64;
            let p_next = p*(-1. - 2.*(b-1.)*p - (1. + a - 2.*b)*p*p)/((1. + 2.*b)*(-1. + p)*p + a*(-1. + 3.*p - 3.*p*p));
            binomial_sample(population, p_next, rng);
            p
        })
    }
}

/// module containing the funuctionality to launch simulations in various ways
/// currently can run them either in a single thread or in parallel
pub mod manager {
    use std::path::PathBuf;
    use std::sync::Arc;

    use rand::prelude::*;
    use threadpool::ThreadPool;
    use closure::closure;

    use crate::sim::*;

    /// launches all of the simulations in a given number of threads
    /// note that regardless of the number of threads, the seeds for each individual simulation are the same,
    /// so that the results are always reproducible
    pub fn launch(iteration: Box<dyn Fn(&mut Vec<f64>, &mut SmallRng) -> f64  + Sync + Send>, num_generations: usize, n: usize, num_rep: usize, p_init: f64, to_fixation: bool, output_folder: &PathBuf, num_threads: usize, seed: u64) {
        if num_threads == 1 {
            // no parallelism, just do all simulations one by one
            let mut rng = SmallRng::seed_from_u64(seed);

            for _ in 0..num_rep {
                let sim_seed: u64 = rng.gen_range(100000..999999);
                if to_fixation {
                    run(&iteration, num_generations, n, p_init, sim_seed, output_folder);
                } else {
                    run_fix(&iteration, num_generations, n, p_init, sim_seed, output_folder);
                }
            }
        } else {
            // parallel launch 
            // in this case, we need to worry about various precautions that rust enforces, 
            // since we want to pass the same boxed closure to all threads

            let pool = ThreadPool::new(num_threads);
            let mut rng = SmallRng::seed_from_u64(seed);
            // i am not sure this is necessary, but rust really complained if i didn't wrap the closure with Arc
            let iteration = Arc::new(iteration);

            for _ in 0..num_rep {
                let sim_seed: u64 = rng.gen_range(100000..999999);
                // i use the closure crate/macro so that it's clear what is done with each argument
                // i.e. numbers are moved, but the Arc'd boxed closure is copied
                // (if i didn't copy the iteration closure, i think i would have ran into the same problem as with GIL in python..., but i am not sure)
                pool.execute(closure!(clone iteration, move num_generations, move n, move p_init, move sim_seed, clone output_folder, || {                
                        run_arc(iteration, num_generations, n, p_init, to_fixation, sim_seed, &output_folder);
                    } 
                ))
            }

            pool.join();

        }

    }
}