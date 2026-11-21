mod db;
mod general;
mod make;

// use std::env::current_dir;
use clap::{Args, Parser, Subcommand};
use rustavel_core::config::CONFIG;
use std::process::Command as ProcessCommand;
// use clap::Args;
use crate::general::lib::{generate_laravel_app_key, set_env_value};
use crate::make::controller::{NewControllerArgs, controller};
use crate::make::migration::{NewMigArgs, migrate};
use crate::make::model::{NewModelArgs, model};

use crate::db::seed::SeedArgs;
use crate::make::factory::{NewFactoryArgs, factory};
use crate::make::seeder::{NewSeederArgs, seeder};
use dialoguer::{Confirm, theme::ColorfulTheme};
use rustavel_core::helper::terminal_ui::{TitleKind, title};
use crate::make::middleware::{middleware, NewMiddlewareArgs};

fn confirm(message: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .default(false)
        .interact()
        // .unwrap()
        // no terminal (CI): treat as "no" instead of panicking
        .unwrap_or(false)
}

#[derive(Args, Debug)]
struct MigrateArgs {
    /// Rollback step count
    // #[arg(long, default_value_t = 0)]
    // rollback: i64,
    #[arg(long, default_value_t = 0, value_parser = clap::value_parser!(i64).range(0..))]
    rollback: i64,

    /// Drop all tables and re-run all migrations
    #[arg(long)]
    fresh: bool,

    /// Run migrations in passive mode
    #[arg(long)]
    passive: bool,

    /// Run seed after migrate
    #[arg(long)]
    seed: bool,
}

#[derive(Parser)]
#[command(name = "artisan")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// app key generate
    KeyGenerate,

    Migrate(MigrateArgs),

    Serv,
    Make {
        #[command(subcommand)]
        kind: MakeCmd,
    },
    Db {
        #[command(subcommand)]
        action: DbCmd,
    },
    // may add: Make { kind: String, name: String }, Seed, etc.
}

#[derive(Subcommand, Debug)]
enum MakeCmd {
    /// Create a new migration file
    Migration(NewMigArgs),
    /// create a new model
    Model(NewModelArgs),
    /// create new controller
    Controller(NewControllerArgs),
    /// create new factory
    Factory(NewFactoryArgs),
    /// create new seeder
    Seeder(NewSeederArgs),
    /// create new middleware
    Middleware(NewMiddlewareArgs),
}
#[derive(Subcommand, Debug)]
enum DbCmd {
    /// Seed the database with records
    Seed(SeedArgs),
    /// Display information about the database
    Show,
    /// Monitor the number of connections to the database
    Monitor,
    /// Display information about a database table
    Table,
    /// Drop all tables, views, and types from the database
    Wipe,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::KeyGenerate => {
            if !CONFIG.app.key.is_empty() && !confirm("Are you sure you want to regenerate key?") {
                title(TitleKind::Info, "Cancelled...");
                std::process::exit(0);
            }

            let app_key = generate_laravel_app_key();

            match set_env_value("APP_KEY", &app_key) {
                Ok(_) => {
                    // title(TitleKind::Error, "Application key set successfully.");
                    title(TitleKind::Success, "Application key set successfully.");
                }
                Err(e) => {
                    title(TitleKind::Error, &format!("failed to set APP_KEY: {}", e));
                }
            }
        }
        Commands::Migrate(migrate_args) => {
            if CONFIG.app.env == "production"
                && !confirm("Are you sure you want to run migration in production mode?")
            {
                title(TitleKind::Info, "Cancelled...");
                std::process::exit(0);
            }
            let mut args = vec!["run", "--package", "rustavel-db", "--bin", "database"];
            if migrate_args.rollback > 0
                || migrate_args.fresh
                || migrate_args.passive
                || migrate_args.seed
            {
                args.push("--");
            }
            args.push("migrate");
            let rollback_str = migrate_args.rollback.to_string();

            if migrate_args.rollback != 0 {
                args.push("--rollback");
                args.push(&rollback_str);
            }
            if migrate_args.fresh {
                args.push("--fresh");
            }
            if migrate_args.passive {
                args.push("--passive");
            }
            if migrate_args.seed {
                args.push("--seed");
            }
            // compile and run database
            let status = ProcessCommand::new("cargo").args(args).status().unwrap();
            if !status.success() {
                title(TitleKind::Error, "Migration compile/run failed");
                std::process::exit(status.code().unwrap_or(1));
            }
        }
        Commands::Serv => {
            println!("Starting rustavel-app with hot-reload (cargo watch)...");

            let status = ProcessCommand::new("cargo")
                .args([
                    "watch",
                    "-p",
                    "rustavel-app",
                    "--ignore",
                    "target",
                    "-x",
                    "run --package rustavel-app --bin rustavel-app",
                ])
                .status();

            match status {
                Ok(s) if s.success() => {}
                Ok(s) => title(
                    TitleKind::Success,
                    &format!("cargo watch exit with code: {} ", s.code().unwrap_or(-1)),
                ),
                Err(e) => title(TitleKind::Error, &format!("cargo watch can't run: {}", e)),
            }
        }
        Commands::Make { kind } => {
            // println!("what use did? {:?}", kind);

            match kind {
                MakeCmd::Migration(args) => {
                    let _ = migrate(&args).await.unwrap_or_else(|e| {
                        println!("{:?}", e);
                        title(TitleKind::Error, &format!("migration error: {:?}", e));
                        false
                    });
                }
                MakeCmd::Model(args) => {
                    model(&args).await.unwrap_or_else(|e| {
                        println!("{:?}", e);
                        title(TitleKind::Error, &format!("model error: {:?}", e));
                    });
                }
                MakeCmd::Controller(args) => {
                    controller(&args).await.unwrap_or_else(|e| {
                        println!("{:?}", e);
                        title(TitleKind::Error, &format!("controller error: {:?}", e));
                    });
                }
                MakeCmd::Factory(args) => {
                    factory(&args).await.unwrap_or_else(|e| {
                        println!("{:?}", e);
                        title(TitleKind::Error, &format!("factory error: {:?}", e));
                    });
                }
                MakeCmd::Seeder(args) => {
                    seeder(&args).await.unwrap_or_else(|e| {
                        println!("{:?}", e);
                        title(TitleKind::Error, &format!("seeder error: {:?}", e));
                    });
                }
                MakeCmd::Middleware(args) => {
                    middleware(&args).await.unwrap_or_else(|e| {
                        println!("{:?}", e);
                        title(TitleKind::Error, &format!("middleware error: {:?}", e));
                    });
                }
            }
        }
        Commands::Db { action } => {
            match action {
                DbCmd::Seed(seed_args) => {
                    let mut args = vec![
                        "run",
                        "--package",
                        "rustavel-db",
                        "--bin",
                        "database",
                        "--",
                        "seed",
                    ];
                    let class = seed_args.class;
                    if let Some(class) = class.as_deref() {
                        args.extend(["--class", class.trim()]);
                    }
                    // compile and run database
                    let status = ProcessCommand::new("cargo").args(args).status().unwrap();
                    if !status.success() {
                        title(TitleKind::Error, "Migration compile/run failed");
                        std::process::exit(status.code().unwrap_or(1));
                    }
                }
                _ => {
                    print!("Command under develope: {:?}", action);
                }
            }
        } // add another command here :)
    }
}
