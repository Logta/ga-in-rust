use ga_prisoners_dilemma::core::errors::GAResult;
use ga_prisoners_dilemma::domain::simulation::Simulation;
use ga_prisoners_dilemma::interface::cli::CliArgs;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> GAResult<()> {
    let args = CliArgs::parse()?;

    if args.help {
        CliArgs::print_help();
        return Ok(());
    }

    let config = args.to_config_builder().build()?;
    let simulation = Simulation::new(config)?;
    let _result = simulation.run()?;

    Ok(())
}
