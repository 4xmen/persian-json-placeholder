use clap::Args;
use illuminate_str::Str;
use minijinja::Environment;
use minijinja::value::Serde;
use rustavel_core::helper::file::project_root;
use rustavel_core::helper::file_content::FileContent;
use rustavel_core::helper::terminal_ui::{Status, operation};
use std::time::Instant;
// use crate::general::lib::register_mod_file;
use crate::general::lib::{ensure_new_file, register_mod_file};
use crate::make::make_error::MakeError;

#[allow(unused)]
const CONTROLLER_TEMPLATE: &str = include_str!("templates/controller.rs.j2");
#[derive(Args, Debug)]
#[command(about = "Create a new controller file")]
pub struct NewControllerArgs {
    /// The name of the controller
    pub name: String,

    /// model name
    #[arg(long)]
    pub model: Option<String>,
}

#[allow(unused)]
#[derive(serde::Serialize)]
struct ControllerContext {
    model: String,
    has_model: bool,
}

#[allow(dead_code)]
pub async fn controller(args: &NewControllerArgs) -> Result<(), MakeError> {
    let start = Instant::now();
    let controller_name = Str::ucfirst(&Str::singular(&args.name));

    let model_name = match &args.model {
        Some(model) => Str::ucfirst(model).to_string(),
        None => "".to_string(),
    };

    let mut env = Environment::new();
    env.add_template("controller", CONTROLLER_TEMPLATE)?;

    let ctx = ControllerContext {
        model: model_name.clone(),
        has_model: !model_name.is_empty(),
    };

    let rendered = env.get_template("controller")?.render(Serde(&ctx))?;

    let base = project_root()?.join("app/src/http/controllers");
    let generated_path = base.join(format!("{}.rs", Str::snake(&controller_name, "_")));

    // Write controller file
    ensure_new_file(&generated_path)?;
    FileContent::put(generated_path.to_str().unwrap(), &rendered).await?;

    // Register in mod.rs
    let mod_file = base.join("mod.rs");

    register_mod_file(&mod_file, &Str::snake(&controller_name, "_")).await?;

    operation(
        &format!("controller made: {:?}", controller_name),
        start.elapsed(),
        Status::Done,
    );
    Ok(())
}
