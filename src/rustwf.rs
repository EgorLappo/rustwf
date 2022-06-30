use std::path::PathBuf;

use librustwf::sim;
use librustwf::manager;

mod cli;

fn main() {
    // get the command line arguments, as defined in cli.rs
    let m = cli::get_cli();

    // i am using .unwrap() here because presumably clap can handle the parsing errors upstream
    let n: usize = *m.get_one("N").unwrap();
    let p: f64 = *m.get_one("freq").unwrap();
    let num_generations: usize = *m.get_one("generations").unwrap();
    let seed: u64 = *m.get_one("seed").unwrap();
    let num_rep: usize = *m.get_one("rep").unwrap();
    let num_threads: usize = *m.get_one("threads").unwrap();
    let output_folder: &PathBuf = m.get_one("output").unwrap();

    let to_fixation: bool = *m.get_one("fixation").unwrap();

    // match on the subcommand, i.e. the simulation type
    // in each case, we read the cli arguments of that particular simulation type,
    // and launch the simulation with the launch function
    match m.subcommand() {
        Some(("wf", _)) => {
            manager::launch(sim::wf(), num_generations, n, num_rep, p, to_fixation, output_folder, num_threads, seed);
        },
        Some(("wfSelection", sub_m)) => {
            let s: f64 = *sub_m.get_one("S").unwrap();
            let h: f64 = *sub_m.get_one("dominance").unwrap();
            manager::launch(sim::wf_selection(s,h), num_generations, n, num_rep, p, to_fixation, output_folder, num_threads, seed);
        },
        Some(("conf3rm", sub_m)) => {
            let s: f64 = *sub_m.get_one("S").unwrap();
            let d: f64 = *sub_m.get_one("D").unwrap();
            manager::launch(sim::fpm_conformity_3rm(s, d), num_generations, n, num_rep, p, to_fixation, output_folder, num_threads, seed);
        },
        Some(("conf5rm", sub_m)) => {
            let s: f64 = *sub_m.get_one("S").unwrap();
            let d3: f64 = *sub_m.get_one("DTHREE").unwrap();
            let d4: f64 = *sub_m.get_one("DFOUR").unwrap();
            manager::launch(sim::fpm_conformity_5rm(s, d3, d4), num_generations, n, num_rep, p, to_fixation, output_folder, num_threads, seed);
        },
        _ => unreachable!("The simulation type you requested is not available or not yet implemented"),
    }
}





