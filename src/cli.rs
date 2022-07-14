use std::path::PathBuf;

use clap::*;


pub fn get_cli() -> ArgMatches {
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
                    .arg(arg!(-s <S> "Selection coefficient s")
                        .allow_hyphen_values(true)
                        .required(false)
                        .default_value("0.0")
                        .value_parser(value_parser!(f64)))
                    .arg(arg!(-d --dominance <H> "Dominance coefficient h")
                        .allow_hyphen_values(true)
                        .required(false)
                        .default_value("0.0")
                        .value_parser(value_parser!(f64)))
            ).subcommand(
                Command::new("conf3rm")
                    .about("Simulate the finite population in which a cultural trait is transmitted with an (anti-)conformist bias and n=3 role models")
                    .arg(arg!(-s <S> "Selection coefficient s")
                        .allow_hyphen_values(true)
                        .required(false)
                        .default_value("0.0")
                        .value_parser(value_parser!(f64)))
                    .arg(arg!(-d <D> "Conformity coefficient D")
                        .allow_hyphen_values(true)
                        .required(false)
                        .default_value("0.0")
                        .value_parser(value_parser!(f64)))
            ).subcommand(
                Command::new("conf5rm")
                    .about("Simulate the finite population in which a cultural trait is transmitted with an (anti-)conformist bias and n=5 role models")
                    .arg(arg!(-s <S> "Selection coefficient s")
                        .allow_hyphen_values(true)
                        .required(false)
                        .default_value("0.0")
                        .value_parser(value_parser!(f64)))
                    .arg(arg!(--dthree <DTHREE> "Conformity coefficient D(3)")
                        .allow_hyphen_values(true)
                        .required(false)
                        .default_value("0.0")
                        .value_parser(value_parser!(f64)))
                    .arg(arg!(--dfour <DFOUR> "Conformity coefficient D(4)")
                        .allow_hyphen_values(true)
                        .required(false)
                        .default_value("0.0")
                        .value_parser(value_parser!(f64)))
            ).subcommand(
                Command::new("fdslin")
                    .about("Simulate the diploid population with an allele under linear frequency-dependent selection (symmetric model from Altenberg (1991))")
                    .arg(arg!(-a <A> "Cefficient a")
                        .allow_hyphen_values(true)
                        .required(false)
                        .default_value("0.0")
                        .value_parser(value_parser!(f64)))
                    .arg(arg!(-b <B> "Coeffcient b")
                        .allow_hyphen_values(true)
                        .required(false)
                        .default_value("0.0")
                        .value_parser(value_parser!(f64)))
            ).arg(arg!(-o --output <FOLDER> "Folder for the simulation outputs")
                .required(false)
                .default_value("./")
                .value_parser(value_parser!(PathBuf)))
            .arg(arg!(-g --generations <NUMGENERATIONS> "Number of generations to simulate")
                .value_parser(value_parser!(usize)))
            .arg(arg!(-n <N> "Number of individuals in the population")
                .value_parser(value_parser!(usize)))
            .arg(arg!(-f --freq <FREQ> "Initial frequency of allele/variant A in the population")
                .required(false)
                .default_value("0.5")
                .value_parser(value_parser!(f64)))
            .arg(arg!(-s --seed <SEED> "Random seed")
                .required(false).default_value("42")
                .value_parser(value_parser!(u64)))
            .arg(arg!(-r --rep <REP> "Number of simulations to run")
                .required(false)
                .default_value("1")
                .value_parser(value_parser!(usize)))
            .arg(arg!(-t --threads <THREADS> "Number of parallel threads to run")
                .required(false)
                .default_value("1")
                .value_parser(value_parser!(usize)))
            .arg(arg!(-x --fixation "simulate to fixation, when frequency is first 0.0 or 1.0 since (in this case the value of the -g argument is an upper limit on the simulation length, so set it high)")
                .action(ArgAction::SetTrue))
            .get_matches();
    m
}