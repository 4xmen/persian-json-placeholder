use crate::general::lib::ensure_new_file;
use crate::make::make_error::MakeError;
use clap::Args;
use illuminate_str::Str;
use minijinja::Environment;
use minijinja::value::Serde;
use rustavel_core::helper::datetime::now_compact;
use rustavel_core::helper::file::project_root;
use rustavel_core::helper::file_content::FileContent;
use rustavel_core::helper::terminal_ui::{Status, operation};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::Instant;

#[allow(unused)]
const MIGRATION_TEMPLATE: &str = include_str!("templates/migration.rs.j2");

#[derive(Args, Debug)]
#[command(about = "Create a new migration file")]
pub struct NewMigArgs {
    /// The name of the migration
    pub name: String,

    /// The existing table to alter
    #[arg(short = 't', long)]
    pub table: Option<String>,

    /// The table to be created
    #[arg(short = 'c', long)]
    pub create: Option<String>,

    /// The location where the migration file should be created
    #[arg(short = 'p', long)]
    pub path: Option<String>,

    /// Indicate any provided migration file paths are pre-resolved absolute paths
    #[arg(long)]
    pub realpath: bool,
}

#[allow(unused)]
#[derive(serde::Serialize)]
struct MigrationContext {
    name: String,
    final_name: String,
    is_create: bool,
    is_table: bool,
    create: Option<String>,
    table: Option<String>,
}

/// Create and persist a new migration file from CLI (artisan) input.
///
/// This function does:
/// 1. Generate a timestamped and snake_cased migration filename.
/// 2. Build a rendering context from CLI arguments (create/table flags, names).
/// 3. Render the migration source code using the migration template.
/// 4. Resolve the target filesystem path for the migration file.
/// 5. Create parent directories if they do not exist.
/// 6. Write the rendered migration to disk.
/// 7. Register the new migration in `database/src/migrations/mod.rs`.
/// 8. Report the operation status and execution time.
#[allow(dead_code)]
pub async fn migrate(args: &NewMigArgs) -> Result<bool, MakeError> {
    let start = Instant::now();

    let final_name = format!("m_{}_{}", now_compact(), Str::snake(&args.name, "_"));

    let mut env = Environment::new();
    env.add_template("migration", MIGRATION_TEMPLATE)?;

    let ctx = MigrationContext {
        name: Str::studly(&args.name),
        final_name: final_name.clone(),
        is_create: args.create.is_some(),
        is_table: args.table.is_some(),
        create: args.create.clone(),
        table: args.table.clone(),
    };

    let rendered = env.get_template("migration")?.render(Serde(&ctx))?;

    // println!("{}", rendered);

    let file_name = format!("{}.rs", final_name);
    let target_path = resolve_target_path(&file_name, args)?;

    // println!("{}", target_path.display());

    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }

    ensure_new_file(&target_path)?;
    FileContent::put(target_path.as_path().to_str().unwrap(), &rendered).await?;

    // It measures execution time, not disk flush time.
    operation(
        &format!("migration created: {}", final_name),
        start.elapsed(),
        Status::Done,
    );

    register_new_migration(&final_name, &Str::studly(&args.name)).await?;

    Ok(true)
}

/// Resolve the final filesystem path for the migration file.
///
/// This function does:
/// 1. Use the default `database/src/migrations` directory when no path is provided.
/// 2. Treat the provided path as an absolute path when `--realpath` is set.
/// 3. Otherwise, resolve the provided path relative to the current working directory.
/// 4. Append the migration filename to the resolved base path.
fn resolve_target_path(file_name: &str, args: &NewMigArgs) -> Result<PathBuf, io::Error> {
    match &args.path {
        None => {
            let base = project_root()?.join("database/src/migrations");
            Ok(base.join(file_name))
        }

        Some(path) if args.realpath => Ok(PathBuf::from(path).join(file_name)),

        Some(path) => {
            let cwd = project_root()?;
            Ok(cwd.join(path).join(file_name))
        }
    }
}

/// Update `mod.rs` to register a new migration.
///
/// This function does:
/// 1. Locate `mod.rs` inside `database/src/migrations`.
/// 2. Validate that placeholders exist:
///    - `// #[add-mig-mods]`
///    - `// #[add-mig-trait]`
/// 3. Check duplicates for module and struct.
/// 4. Append new module and struct before placeholders.
///
#[allow(dead_code)]
pub async fn register_new_migration(final_name: &str, struct_raw: &str) -> io::Result<()> {
    // Locate mod.rs
    let mod_rs_path: PathBuf = project_root()?.join("database/src/migrations/registry.rs");

    let mut file_data = match FileContent::get(mod_rs_path.to_str().unwrap()).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    file_data = format!(
        "{}    migration!({},{});\n}}",
        file_data.trim().trim_matches('}'),
        final_name,
        struct_raw
    );

    FileContent::put(mod_rs_path.to_str().unwrap(), &file_data).await?;

    Ok(())
}
