# `rustwf`: Wright-Fisher-type simulations in Rust

I have tried to adapt some simple simulation I use in my research ([FPM_diffusion](https://github.com/EgorLappo/FPM_diffusion)) into a Rust program. As this is mostly a learning project for me, I have tried to use several features of the language that are most probably not the most effective and necessary. Still, single-threaded performance of this code is 25x faster than the naive `python` implementation I have previously been using.

If, for some reason, you are interested in this, comments are welcome!

## Usage

To compile and create an executable, first make sure you have the [Rust toolchain](https://rustup.rs) installed on your computer. Then, run `cargo build --release && cp target/release/rustwf .` to compile the executable and move it to project root.

After compiling the program, the usage is nicely described by the CLI interface. Typing `./rustwf help`, you will see the following message:

```
rustwf 0.1.1
Egor Lappo, egor@ccrma.stanford.edu
Wright-Fisher type simulations written in pure rust

USAGE:
    rustwf [OPTIONS] --generations <NUMGENERATIONS> -n <N> <SUBCOMMAND>

OPTIONS:
    -f, --freq <FREQ>
            Initial frequency of allele/variant A in the population [default: 0.5]

    -g, --generations <NUMGENERATIONS>
            Number of generations to simulate

    -h, --help
            Print help information

    -n <N>
            Number of individuals in the population

    -o, --output <FOLDER>
            Folder for the simulation outputs [default: ./]

    -r, --rep <REP>
            Number of simulations to run [default: 1]

    -s, --seed <SEED>
            Random seed [default: 42]

    -t, --threads <THREADS>
            Number of parallel threads to run [default: 1]

    -V, --version
            Print version information

    -x, --fixation
            simulate to fixation, when frequency is first 0.0 or 1.0 since (in this case the value
            of the -g argument is an upper limit on the simulation length, so set it high)

SUBCOMMANDS:
    conf3rm        Simulate the finite population in which a cultural trait is transmitted with
                       an (anti-)conformist bias and n=3 role models
    conf5rm        Simulate the finite population in which a cultural trait is transmitted with
                       an (anti-)conformist bias and n=5 role models
    fdslin         Simulate the diploid population with an allele under linear
                       frequency-dependent selection (symmetric model from Altenberg (1991))
    help           Print this message or the help of the given subcommand(s)
    wf             Simulate neutral Wright-Fisher model
    wfSelection    Simulate the Wright-Fisher model with selection
```

Essentially, you input all of the essential parameters and also choose the simulation type as a subcommand. Each simulation type may have its own additional arguments, which can be seen by typing `./rustwf <SUBCOMMAND> help`.

## Notes

* The program is fully deterministic if the same seed is provided to it. 
* The output is saved as a `.csv` file in the output folder. The name of the `.csv` file is the seed used in that particular run. In my workflow, I create the folders according to the simulation parameters I want to simulate, and then run `rustwf` with corresponding parameters. 
* You can easily extend the program to other types of simulation: you need to add your iteration function to the `sim` module in `lib.rs`, then extend the `clap` CLI interface description in `cli.rs`, and finally add a pattern match on the new argument in `rustwf.rs`.

