use crate::kick_assembler::config::Config;
use anyhow::{Context, bail};
use clap::{Args, Parser, Subcommand};
use figment::Figment;
use figment::providers::{Format, Toml};
use serde::Deserialize;
use std::env;
use std::fs::create_dir;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// The argument parser.
pub struct Cli<A: Args, S: Subcommand> {
    #[command(flatten)]
    custom_args: A,

    #[command(subcommand)]
    command: CliCommand<S>,
}

#[derive(Subcommand)]
enum CliCommand<S: Subcommand> {
    /// Build the program (alias b)
    #[command(alias = "b")]
    Build,
    /// Build and run the program (alias r)
    #[command(alias = "r")]
    Run,
    #[command(flatten)]
    CustomCommand(S),
}

impl<A: Args, S: Subcommand> Deref for Cli<A, S> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.custom_args
    }
}

impl<A: Args, S: Subcommand> Cli<A, S> {
    /// Execute a program within the cli framework.
    ///
    /// # Errors
    ///
    /// Reading/writing/executing files.
    pub fn execute<B, C>(build: B, custom_command: C) -> anyhow::Result<()>
    where
        B: FnOnce(Config, A) -> anyhow::Result<PathBuf>,
        C: FnOnce(Config, A, S) -> anyhow::Result<()>,
    {
        let cli = Self::parse();

        let Some(home_dir) = env::home_dir() else {
            bail!("can't determine home directory")
        };

        let config_source: ConfigSource = Figment::new()
            .merge(Toml::file_exact(home_dir.join(".atari-8bit-tools.toml")))
            .merge(Toml::file("atari-8bit-tools.toml"))
            .extract()?;
        let config = Config {
            java: config_source.java.as_path(),
            kick_assembler: config_source.kick_assembler.as_path(),
            source_directory: config_source.source_directory.as_path(),
            output_directory: config_source.output_directory.as_path(),
        };

        if !config.output_directory.is_dir() {
            create_dir(config.output_directory).context("can't create output directory")?;
        }

        if let CliCommand::CustomCommand(c) = cli.command {
            custom_command(config, cli.custom_args, c)?;
        } else {
            let file = build(config, cli.custom_args)?;

            if matches!(cli.command, CliCommand::Run) {
                Command::new(&config_source.atari800)
                    .args(&config_source.atari800_args)
                    .arg(file)
                    .stdin(Stdio::null())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .with_context(|| "Failed to run atari800")?;
            }
        }

        Ok(())
    }
}

#[derive(Deserialize)]
struct ConfigSource {
    // kickassembler config
    pub java: PathBuf,
    pub kick_assembler: PathBuf,
    pub source_directory: PathBuf,
    pub output_directory: PathBuf,
    // emulator config
    pub atari800: PathBuf,
    #[serde(default)]
    pub atari800_args: Vec<String>,
}
