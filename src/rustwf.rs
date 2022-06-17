use clap::*;

use std::path::PathBuf;

use librustwf::sim;
use librustwf::manager;

fn main() {
    let m = Command::new("rustwf")
            .author("Egor Lappo, egor@ccrma.stanford.edu")
            .version("0.1.0")
            .about("Wright-Fisher type simulations written in pure rust")
            .propagate_version(true)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                Command::new("wf")
                    .about("Simulate neutral Wright-Fisher model")
            ).subcommand(
                Command::new("wfSelection")
                    .about("Simulate the Wright-Fisher model with selection")
                    .arg(arg!(-s <S> "Selection coefficient s").default_value("0.0").value_parser(value_parser!(f64)))
                    .arg(arg!(-h <H> "Dominance coefficient h").default_value("0.0").value_parser(value_parser!(f64)))
            ).subcommand(
                Command::new("conf3rm")
                    .about("Simulate the finite population in which a cultural trait is transmitted with an (anti-)conformist bias and n=3 role models")
                    .arg(arg!(-s <S> "Selection coefficient s").default_value("0.0").value_parser(value_parser!(f64)))
                    .arg(arg!(-d <D> "Conformity coefficient D").default_value("0.0").value_parser(value_parser!(f64)))
            ).subcommand(
                Command::new("conf5rm")
                    .about("Simulate the finite population in which a cultural trait is transmitted with an (anti-)conformist bias and n=5 role models")
                    .arg(arg!(-s <S> "Selection coefficient s").default_value("0.0").value_parser(value_parser!(f64)))
                    .arg(arg!(--dthree <DTHREE> "Conformity coefficient D(3)").default_value("0.0").value_parser(value_parser!(f64)))
                    .arg(arg!(--dfour <DFOUR> "Conformity coefficient D(4)").default_value("0.0").value_parser(value_parser!(f64)))
            ).arg(arg!(-o --output <FOLDER> "Folder for the simulation outputs").default_value("./").value_parser(value_parser!(PathBuf)))
            .arg(arg!(-g --generations <NUMGENERATIONS> "Number of generations to simulate").value_parser(value_parser!(usize)))
            .arg(arg!(-n <N> "Number of individuals in the population").value_parser(value_parser!(usize)))
            .arg(arg!(-f --freq <FREQ> "Initial frequency of allele/variant A in the population").default_value("0.5").value_parser(value_parser!(f64)))
            .arg(arg!(-s --seed <SEED> "Random seed").default_value("42").value_parser(value_parser!(u64)))
            .arg(arg!(-r --rep <REP> "Number of simulations to run").default_value("1").value_parser(value_parser!(usize)))
            .arg(arg!(-t --threads <THREADS> "Number of parallel threads to run").default_value("1").value_parser(value_parser!(usize))).get_matches();

        // i am using .unwrap() here because presumably clap can handle the parsing errors upstream
        let n: usize = *m.get_one("N").unwrap();
        let p: f64 = *m.get_one("FREQ").unwrap();
        let num_generations: usize = *m.get_one("NUMGENERATIONS").unwrap();
        let seed: u64 = *m.get_one("SEED").unwrap();
        let num_rep: usize = *m.get_one("REP").unwrap();
        let num_threads: usize = *m.get_one("THREADS").unwrap();
        let output_folder: &PathBuf = m.get_one("FOLDER").unwrap();

        match m.subcommand() {
            Some(("wf", _)) => {
                manager::launch(sim::wf(), num_generations, n, num_rep, p, output_folder, num_threads, seed);
            },
            Some(("wfSelection", sub_m)) => {
                let s: f64 = *sub_m.get_one("S").unwrap();
                let h: f64 = *sub_m.get_one("H").unwrap();
                manager::launch(sim::wf_selection(s,h), num_generations, n, num_rep, p, output_folder, num_threads, seed);
            },
            Some(("conf3rm", sub_m)) => {
                let s: f64 = *sub_m.get_one("S").unwrap();
                let d: f64 = *sub_m.get_one("D").unwrap();
                manager::launch(sim::fpm_conformity_3rm(s, d), num_generations, n, num_rep, p, output_folder, num_threads, seed);
            },
            Some(("conf5rm", sub_m)) => {
                let s: f64 = *sub_m.get_one("S").unwrap();
                let d3: f64 = *sub_m.get_one("DTHREE").unwrap();
                let d4: f64 = *sub_m.get_one("DFOUR").unwrap();
                manager::launch(sim::fpm_conformity_5rm(s, d3, d4), num_generations, n, num_rep, p, output_folder, num_threads, seed);
            },
            _ => unreachable!("The simulation type you requested is not available or not yet implemented"),
        }
}





