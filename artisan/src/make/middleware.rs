use std::time::Instant;
use clap::Args;
use illuminate_str::Str;
use minijinja::Environment;
use minijinja::value::Serde;
use rustavel_core::helper::file_content::FileContent;
use rustavel_core::helper::terminal_ui::{operation, Status};
use crate::general::lib::{ensure_new_file, register_mod_file};
use crate::make::make_error::MakeError;

#[derive(Args, Debug)]
#[command(about = "Create a new HTTP middleware file")]
pub struct NewMiddlewareArgs {
    /// The name of the middleware
    pub name: String,
}


#[allow(unused)]
#[derive(serde::Serialize)]
struct MiddlewaresData {
    name: String,
}

#[allow(unused)]
const MIDDLEWARE_TEMPLATE: &str = include_str!("templates/middleware.rs.j2");

#[allow(dead_code)]
pub async fn middleware(args: &NewMiddlewareArgs) -> Result<(), MakeError> {
    let start = Instant::now();
    let mut mw_name = Str::snake(&Str::singular(&args.name),"_");

    if !mw_name.ends_with("_middleware") {
        mw_name.push_str("_middleware");
    }

    // add template and render
    let mut env = Environment::new();
    env.add_template("middleware", MIDDLEWARE_TEMPLATE)?;

    let ctx = MiddlewaresData {
        name: mw_name.clone(),
    };
    let rendered = env.get_template("middleware")?.render(Serde(&ctx))?;

    let base = std::env::current_dir()?.join("app/src/http/middleware");
    let generated_path = base.join(format!("{}.rs", &mw_name));

    // Write controller file
    ensure_new_file(&generated_path)?;
    FileContent::put(generated_path.to_str().unwrap(), &rendered).await?;

    // Register in mod.rs
    let mod_file = base.join("mod.rs");

    register_mod_file(&mod_file, &Str::snake(&mw_name, "_")).await?;

    operation(
        &format!("middleware made: {:?}", mw_name),
        start.elapsed(),
        Status::Done,
    );

    Ok(())
}
