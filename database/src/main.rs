use crate::seeders::database_seeder::SeederContext;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rustavel_artisan::db::seed::SeedArgs;
use rustavel_core::config::database::create_db_pool;
use rustavel_core::helper::terminal_ui::{Status, TitleKind, operation, title};
use std::time::Instant;

include!(concat!(env!("OUT_DIR"), "/generated_seeders_import.rs"));

mod factories;
mod migrations;
mod migrator;
mod seeders;

/// Root CLI definition.
///
/// Available commands:
///
///     cli-db migrate
///     cli-db migrate --fresh
///     cli-db migrate --fresh --seed
///     cli-db seed
///     cli-db seed --class UserSeeder
#[derive(Parser, Debug)]
#[command(name = "cli-db", version, about = "Database cli-db and seeding CLI")]
struct Cli {
    /// The command to execute.
    #[command(subcommand)]
    command: Command,
}

/// Available CLI commands.
///
/// cli-db and seeding are separate operations. However, the `migrate`
/// command can optionally run the default database seeder after cli-db.
#[derive(Subcommand, Debug)]
enum Command {
    /// Run database cli-dbs.
    ///
    /// Examples:
    ///
    ///     cli-db migrate
    ///     cli-db migrate --fresh
    ///     cli-db migrate --rollback 2
    ///     cli-db migrate --fresh --seed
    Migrate {
        /// Rollback the specified number of cli-db steps.
        // #[arg(long, default_value_t = 0)]
        // rollback: i64,
        #[arg(long, default_value_t = 0, value_parser = clap::value_parser!(i64).range(0..))]
        rollback: i64,

        /// Run cli-dbs in passive mode.
        #[arg(long)]
        passive: bool,

        /// Drop all tables and re-run all cli-dbs.
        ///
        /// Can be combined with `fresh`.
        #[arg(long)]
        fresh: bool,

        /// Run the default database seeder after cli-dbs.
        ///
        /// Example:
        ///
        ///     cli-db migrate fresh seed
        #[arg(long)]
        seed: bool,
    },

    /// Run database seeders independently from cli-dbs.
    ///
    /// Examples:
    ///
    ///     cli-db seed
    ///     cli-db seed --class UserSeeder
    Seed(SeedArgs),
}

/// Application entry point.
///
/// Tokio creates and manages the async runtime for the entire CLI.
/// This allows both cli-db and seeding operations to use `await`.
#[tokio::main]
async fn main() {
    // Load environment variables from `.env`, if available.
    dotenv::dotenv().ok();

    // Parse command-line arguments.
    let cli = Cli::parse();

    // Execute the selected command.
    // if let Err(error) = run(cli).await {
    //     logger::error(&format!("{:?}", error));
    // }

    if let Err(error) = run(cli).await {
        // the logger prints from a background task,which exit() would cut off
        eprintln!("\x1b[31m{:?}\x1b[0m", error);
        std::process::exit(1);
    }
}

/// Execute the selected CLI command.
///
/// This function only handles command orchestration. The actual cli-db
/// and seeding logic lives inside their respective modules.
async fn run(cli: Cli) -> Result<(), anyhow::Error> {
    match cli.command {
        // -------------------------------------------------------------
        // MIGRATE
        // -------------------------------------------------------------
        //
        // cli-db always runs first. If `--seed` is specified, the
        // default DatabaseSeeder runs after cli-db completes.
        Command::Migrate {
            rollback,
            passive,
            fresh,
            seed,
        } => {
            // Run cli-dbs first. If this fails, seeding is skipped.
            migrator::run_migrations(rollback, passive, fresh).await?;

            // `migrate --seed` runs the default DatabaseSeeder.
            //
            // Unlike the standalone `seed` command, no specific seeder
            // class is selected here.
            if seed {
                let start = Instant::now();
                title(TitleKind::Info, "Running seeders");
                match seeders::database_seeder::DatabaseSeeder::run().await {
                    Ok(_e) => {
                        operation("All seeder done", start.elapsed(), Status::Done);
                    }
                    Err(e) => {
                        operation(&format!("Seeding :{}", e), start.elapsed(), Status::Failed);
                        return Err(e);
                    }
                }
            }
        }

        // -------------------------------------------------------------
        // SEED
        // -------------------------------------------------------------
        //
        // Seeding is completely independent from cli-db.
        //
        // Examples:
        //
        //     cli-db seed
        //     cli-db seed --class UserSeeder
        Command::Seed(args) => {
            // The SeedArgs value can be used by DatabaseSeeder to decide
            // whether a specific seeder class should be executed.
            if let Some(class) = args.class {
                let ctx = SeederContext {
                    pool: create_db_pool().await?,
                };
                include!(concat!(env!("OUT_DIR"), "/generated_seeders.rs"));
            } else {
                seeders::database_seeder::DatabaseSeeder::run().await?;
            }
        }
    }

    Ok(())
}

pub async fn run_seeder<F, T>(name: &str, fut: F) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    let start = Instant::now();
    let out = fut.await;
    match &out {
        Ok(_output) => {
            operation(name, start.elapsed(), Status::Done);
        }
        Err(_e) => operation(name, start.elapsed(), Status::Failed),
    }
    out
}
