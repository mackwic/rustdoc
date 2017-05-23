extern crate clap;
extern crate rls_analysis as analysis;

use analysis::raw::DefKind;

use clap::{App, Arg, SubCommand};

use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};

struct Config {
    manifest_path: PathBuf,
    host: analysis::AnalysisHost,
}

impl Config {
    fn new(matches: &clap::ArgMatches) -> Result<Config, Box<std::error::Error>> {
        // unwrap is okay because we take a default value
        let manifest_path = PathBuf::from(matches.value_of("manifest-path").unwrap());
        let host = generate_analysis(&manifest_path)?;

        Ok(
            Config {
                manifest_path,
                host,
            }
        )
    }
}

fn main() {
    let version = env!("CARGO_PKG_VERSION");

    let matches = App::new("rustdoc")
        .version(version)
        .author("Steve Klabnik <steve@steveklabnik.com>")
        .about("Generate web-based documentation from your Rust code.")
        .arg(
            Arg::with_name("manifest-path")
                .long("manifest-path")
                // remove the unwrap in Config::new if this default_value goes away
                .default_value(".")
                .help("The path to the Cargo manifest of the project you are documenting.")
        )
        .subcommand(SubCommand::with_name("build").about("generates documentation"))
        .get_matches();

    let config = Config::new(&matches).unwrap_or_else(
        |err| {
            println!("Problem creating configuration: {}", err);
            process::exit(1);
        }
    );

    let result = match matches.subcommand_name() {
        Some("build") => build(&config),
        // default is to build
        None => build(&config),
        Some(_) => Err("Something strange is going on with subcommands, please file a bug!".into()),
    };

    if let Err(e) = result {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn build(config: &Config) -> Result<(), Box<std::error::Error>> {
    let roots = config.host.def_roots().unwrap();

    let &(id, _) = roots
        .iter()
        .find(|&&(_, ref name)| name == "example")
        .unwrap();

    println!("elements of this crate:");

    let defs = config
        .host
        .for_each_child_def(id, |_, def| def.clone())
        .unwrap();
    
    let kinds = vec![
        DefKind::Mod,
        DefKind::Static,
        DefKind::Const,
        DefKind::Enum,
        DefKind::Struct,
        DefKind::Union,
        DefKind::Trait,
        DefKind::Function,
        DefKind::Macro,
    ];

    for kind in kinds {
        println!("{:?}s:", kind);
        for def in defs.iter().filter(|def| def.kind == kind) {
            println!("{}", def.name);
        }
    }

    Ok(())
}

fn generate_analysis(manifest_path: &Path)
    -> Result<analysis::AnalysisHost, Box<std::error::Error>> {
    let mut command = Command::new("cargo");

    let manifest_path = manifest_path.to_str().unwrap();

    command.arg("build");
    command.args(&["--manifest-path", manifest_path]);

    command.env("RUSTFLAGS", "-Z save-analysis");
    // TODO build an actual path
    command.env("CARGO_TARGET_DIR", &format!("{}/target/rls", manifest_path));

    // for now, just eat the output
    command.stdout(Stdio::null());
    command.stderr(Stdio::null());

    print!("generating save analysis data...");
    io::stdout().flush()?;

    command.spawn()?.wait()?;
    println!("done.");

    print!("loading save analysis data...");
    io::stdout().flush()?;
    let host = analysis::AnalysisHost::new(analysis::Target::Debug);
    host.reload(
            &PathBuf::from(manifest_path),
            &PathBuf::from(manifest_path),
            true,
        )
        .unwrap();
    println!("done.");

    Ok(host)
}
