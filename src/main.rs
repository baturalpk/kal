mod backup;
mod db_ops;

use std::env;
use std::path::Path;

use anyhow::anyhow;
use chrono::Datelike;
use clap::{Args, Parser, Subcommand};
use db_ops::KalRecord;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use inquire::{Select, Text};
use serde::Deserialize;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Applies initial migration(s) for a newly created database file.
    /// New database files must be created manually at the provided "db_path" location.
    Init,
    /// Creates a new KAL record.
    #[clap(visible_alias = "cm")]
    Commit,
    /// Deletes all KAL records committed today.
    Reset,
    /// Lists KAL records committed only today if no argument's provided.
    #[clap(visible_alias = "ls")]
    List(LsArgs),
}

#[derive(Args)]
struct LsArgs {
    /// Lists all records at the given year.
    #[arg(short, long, value_name = "YEAR")]
    all: Option<u32>,
}

#[derive(Deserialize)]
struct Config {
    db_path: String,
    backup_folder: String,
    categories: Vec<String>,
}

const CONFIG_PATH_ENV_KEY: &str = "KAL_CONFIG_PATH";
const CONFIG_FILE_NAME: &str = "kal.config.toml";

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config: Config = Figment::new()
        .merge(Toml::file(CONFIG_FILE_NAME))
        .join(Toml::file(
            Path::new(&env::var(CONFIG_PATH_ENV_KEY).unwrap_or(".".to_string()))
                .join(CONFIG_FILE_NAME),
        ))
        .extract()
        .map_err(|e| {
            anyhow!(
                "configuration file ('{}') does not exist or {}",
                CONFIG_FILE_NAME,
                e
            )
        })?;

    backup::check_folder(&config.backup_folder)?;

    let connection = db_ops::create_connection(&config.db_path)?;

    let today = chrono::Local::now();
    let year = today.year() as u32;
    let ordinal_day = today.ordinal() as u32;

    match &cli.command {
        Commands::Init => {
            db_ops::init_database(&connection)?;
            backup::take_backup(&config.db_path, &config.backup_folder)?;
        }

        Commands::Commit => {
            let category_ans = Select::new("Select the category:", config.categories).prompt()?;
            let details_ans = Text::new("Details:").prompt()?;

            db_ops::insert_record(
                &connection,
                &KalRecord {
                    year,
                    ordinal_day,
                    category: category_ans.to_string(),
                    details: (!details_ans.is_empty()).then(|| details_ans),
                },
            )?;

            backup::take_backup(&config.db_path, &config.backup_folder)?;
        }

        Commands::Reset => {
            db_ops::delete_records_ordinal(&connection, year, ordinal_day)?;
            backup::take_backup(&config.db_path, &config.backup_folder)?;
        }

        Commands::List(args) => {
            let records = match args.all {
                Some(year) => db_ops::query_records_year(&connection, year)?,
                None => db_ops::query_record_ordinal(&connection, year, ordinal_day)?,
            };

            for rec in records {
                println!("{rec}");
            }
        }
    }

    Ok(())
}
