use crate::config::ConfigBuilder;
use crate::error::{GAError, GAResult};
use std::env;

pub struct CliArgs {
    pub generations: Option<usize>,
    pub population: Option<usize>,
    pub mutation_rate: Option<f64>,
    pub dna_length: Option<usize>,
    pub report_interval: Option<usize>,
    pub elite_size: Option<usize>,
    pub help: bool,
}

impl CliArgs {
    pub fn parse() -> GAResult<Self> {
        let args: Vec<String> = env::args().collect();
        let mut cli_args = CliArgs {
            generations: None,
            population: None,
            mutation_rate: None,
            dna_length: None,
            report_interval: None,
            elite_size: None,
            help: false,
        };

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-h" | "--help" => {
                    cli_args.help = true;
                }
                "-g" | "--generations" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(GAError::InvalidChoice(
                            "Missing value for generations".to_string(),
                        ));
                    }
                    cli_args.generations = Some(args[i].parse().map_err(|_| {
                        GAError::InvalidChoice("Invalid generations value".to_string())
                    })?);
                }
                "-p" | "--population" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(GAError::InvalidChoice(
                            "Missing value for population".to_string(),
                        ));
                    }
                    cli_args.population = Some(args[i].parse().map_err(|_| {
                        GAError::InvalidChoice("Invalid population value".to_string())
                    })?);
                }
                "-m" | "--mutation-rate" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(GAError::InvalidChoice(
                            "Missing value for mutation rate".to_string(),
                        ));
                    }
                    cli_args.mutation_rate = Some(args[i].parse().map_err(|_| {
                        GAError::InvalidChoice("Invalid mutation rate value".to_string())
                    })?);
                }
                "-d" | "--dna-length" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(GAError::InvalidChoice(
                            "Missing value for DNA length".to_string(),
                        ));
                    }
                    cli_args.dna_length = Some(args[i].parse().map_err(|_| {
                        GAError::InvalidChoice("Invalid DNA length value".to_string())
                    })?);
                }
                "-r" | "--report-interval" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(GAError::InvalidChoice(
                            "Missing value for report interval".to_string(),
                        ));
                    }
                    cli_args.report_interval = Some(args[i].parse().map_err(|_| {
                        GAError::InvalidChoice("Invalid report interval value".to_string())
                    })?);
                }
                "-e" | "--elite-size" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(GAError::InvalidChoice(
                            "Missing value for elite size".to_string(),
                        ));
                    }
                    cli_args.elite_size = Some(args[i].parse().map_err(|_| {
                        GAError::InvalidChoice("Invalid elite size value".to_string())
                    })?);
                }
                _ => {
                    return Err(GAError::InvalidChoice(format!(
                        "Unknown argument: {}",
                        args[i]
                    )));
                }
            }
            i += 1;
        }

        Ok(cli_args)
    }

    pub fn to_config_builder(self) -> ConfigBuilder {
        let mut builder = ConfigBuilder::new();

        if let Some(generations) = self.generations {
            builder = builder.generations(generations);
        }
        if let Some(population) = self.population {
            builder = builder.population(population);
        }
        if let Some(mutation_rate) = self.mutation_rate {
            builder = builder.mutation_rate(mutation_rate);
        }
        if let Some(dna_length) = self.dna_length {
            builder = builder.dna_length(dna_length);
        }
        if let Some(report_interval) = self.report_interval {
            builder = builder.report_interval(report_interval);
        }
        if let Some(elite_size) = self.elite_size {
            builder = builder.elite_size(elite_size);
        }

        builder
    }

    pub fn print_help() {
        println!("GA Prisoner's Dilemma - Genetic Algorithm Simulation");
        println!();
        println!("USAGE:");
        println!("    ga_prisoners_dilemma [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("    -g, --generations <NUM>      Number of generations to run [default: 50000]");
        println!("    -p, --population <NUM>       Population size [default: 20]");
        println!("    -m, --mutation-rate <RATE>   Mutation rate (0.0-1.0) [default: 0.01]");
        println!("    -d, --dna-length <NUM>       DNA string length [default: 6]");
        println!("    -r, --report-interval <NUM>  Report every N generations [default: 5000]");
        println!("    -e, --elite-size <NUM>       Number of elite individuals [default: 2]");
        println!("    -h, --help                   Print this help message");
        println!();
        println!("EXAMPLES:");
        println!("    ga_prisoners_dilemma");
        println!("    ga_prisoners_dilemma -g 10000 -p 50 -m 0.05");
        println!("    ga_prisoners_dilemma --population 100 --mutation-rate 0.02");
    }
}
