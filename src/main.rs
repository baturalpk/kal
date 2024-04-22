mod categories;
mod db_ops;

use chrono::Datelike;
use clap::{Args, Parser, Subcommand};
use db_ops::KalRecord;
use inquire::{Select, Text};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path of the SQLite database file
    #[arg(long)]
    db: String,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Commit,
    Reset,
    Ls(LsArgs),
}

#[derive(Args)]
struct LsArgs {
    /// Lists all records at the given year
    #[arg(short, long, value_name = "YEAR")]
    all: Option<u32>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let connection = db_ops::create_connection(&cli.db)?;

    let today = chrono::Local::now();
    let year = today.year() as u32;
    let ordinal_day = today.ordinal() as u32;

    match &cli.command {
        Commands::Init => db_ops::init_database(&connection)?,

        Commands::Commit => {
            let category_ans =
                Select::new("Select the category:", categories::get_all()).prompt()?;

            let details_ans = Text::new("Details: ")
                .prompt()
                .unwrap_or_else(|_| "".to_string());

            db_ops::upsert_record(
                &connection,
                &KalRecord {
                    year,
                    ordinal_day,
                    category: category_ans.to_string(),
                    details: (!details_ans.is_empty()).then(|| details_ans),
                },
            )?;
        }

        Commands::Reset => {
            db_ops::delete_records_ordinal(&connection, year, ordinal_day)?;
        }

        Commands::Ls(args) => {
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
