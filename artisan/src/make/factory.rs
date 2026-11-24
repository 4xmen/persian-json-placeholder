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

#[derive(Args, Debug)]
#[command(about = "Create a new factory file")]
pub struct NewFactoryArgs {
    /// The name of the factory
    pub name: String,

    /// model name
    #[arg(long)]
    pub model: Option<String>,
}

#[allow(unused)]
#[derive(serde::Serialize)]
struct FactoryContext {
    name: String,
    model: String,
    has_model: bool,
}

#[allow(unused)]
const FACTORY_TEMPLATE: &str = include_str!("templates/factory.rs.j2");
#[allow(dead_code)]
pub async fn factory(args: &NewFactoryArgs) -> Result<(), MakeError> {
    let start = Instant::now();
    let factory_name = Str::ucfirst(&Str::singular(&args.name));

    let factory_name = if factory_name.ends_with("Factory") {
        factory_name
    } else {
        format!("{}Factory", factory_name)
    };

    let model_name = match &args.model {
        Some(model) => Str::ucfirst(model).to_string(),
        None => "".to_string(),
    };

    // add template and render
    let mut env = Environment::new();
    env.add_template("factory", FACTORY_TEMPLATE)?;

    let ctx = FactoryContext {
        name: factory_name.clone(),
        model: model_name.clone(),
        has_model: !model_name.is_empty(),
    };
    let rendered = env.get_template("factory")?.render(Serde(&ctx))?;

    let base = project_root()?.join("database/src/factories/");
    let generated_path = base.join(format!("{}.rs", Str::snake(&factory_name, "_")));

    // Write controller file
    ensure_new_file(&generated_path)?;
    FileContent::put(generated_path.to_str().unwrap(), &rendered).await?;

    // Register in mod.rs
    let mod_file = base.join("mod.rs");

    register_mod_file(&mod_file, &Str::snake(&factory_name, "_")).await?;

    operation(
        &format!("factory made: {:?}", factory_name),
        start.elapsed(),
        Status::Done,
    );

    Ok(())
}
