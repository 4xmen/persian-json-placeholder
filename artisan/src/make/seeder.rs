use crate::general::lib::{ensure_new_file, register_mod_file};
use crate::make::make_error::MakeError;
use clap::Args;
use illuminate_str::Str;
use minijinja::Environment;
use minijinja::value::Serde;
use rustavel_core::helper::file::project_root;
use rustavel_core::helper::file_content::FileContent;
use rustavel_core::helper::terminal_ui::{Status, operation};
use std::time::Instant;

#[derive(Args, Debug)]
#[command(about = "Create a new seeder file")]
pub struct NewSeederArgs {
    /// The name of the seeder
    pub name: String,

    /// model name
    #[arg(long)]
    pub model: Option<String>,
}

#[allow(unused)]
#[derive(serde::Serialize)]
struct SeederContext {
    name: String,
    model: String,
    has_model: bool,
}

#[allow(unused)]
const SEEDER_TEMPLATE: &str = include_str!("templates/seeder.rs.j2");

#[allow(dead_code)]
pub async fn seeder(args: &NewSeederArgs) -> Result<(), MakeError> {
    let start = Instant::now();
    let seeder_name = Str::ucfirst(&Str::singular(&args.name));

    let seeder_name = if seeder_name.ends_with("Seeder") {
        seeder_name
    } else {
        format!("{}Seeder", seeder_name)
    };

    let model_name = match &args.model {
        Some(model) => Str::ucfirst(model).to_string(),
        None => "".to_string(),
    };

    // add template and render
    let mut env = Environment::new();
    env.add_template("seeder", SEEDER_TEMPLATE)?;

    let ctx = SeederContext {
        name: seeder_name.clone(),
        model: model_name.clone(),
        has_model: !model_name.is_empty(),
    };
    let rendered = env.get_template("seeder")?.render(Serde(&ctx))?;

    let base = project_root()?.join("database/src/seeders/");
    let generated_path = base.join(format!("{}.rs", Str::snake(&seeder_name, "_")));

    // Write controller file
    ensure_new_file(&generated_path)?;
    FileContent::put(generated_path.to_str().unwrap(), &rendered).await?;

    // Register in mod.rs
    let mod_file = base.join("mod.rs");

    register_mod_file(&mod_file, &Str::snake(&seeder_name, "_")).await?;

    operation(
        &format!("seeder made: {:?}", seeder_name),
        start.elapsed(),
        Status::Done,
    );

    Ok(())
}
