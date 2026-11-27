use crate::general::generate_model::save_generated_model;
use crate::general::lib::ensure_new_file;
use crate::make::controller::{NewControllerArgs, controller};
use crate::make::factory::{NewFactoryArgs, factory};
use crate::make::make_error::MakeError;
use crate::make::migration::{NewMigArgs, migrate};
use crate::make::seeder::{NewSeederArgs, seeder};
use clap::Args;
use illuminate_str::Str;
use minijinja::{Environment, context};
use rustavel_core::helper::file::project_root;
use rustavel_core::helper::file_content::FileContent;
use rustavel_core::helper::terminal_ui::{Status, operation};
use tokio::time::Instant;

#[allow(unused)]
const MODEL_TEMPLATE: &str = include_str!("templates/model.rs.j2");
#[derive(Args, Debug)]
#[command(about = "Create a new model file")]
pub struct NewModelArgs {
    /// The name of the model
    pub name: String,

    /// has migration
    #[arg(short = 'm')]
    pub has_migration: bool,

    /// has factory
    #[arg(short = 'f')]
    pub has_factory: bool,

    /// has seeder
    #[arg(short = 's')]
    pub has_seeder: bool,

    /// has controller
    #[arg(short = 'c')]
    pub has_controller: bool,
}

#[allow(dead_code)]
pub async fn model(args: &NewModelArgs) -> Result<(), MakeError> {
    let start = Instant::now();
    let model_name = Str::ucfirst(&Str::singular(&args.name));

    // check before writing anything, so nothing is half-created
    let model_path = project_root()?
        .join("app/src/models")
        .join(format!("{}.rs", Str::snake(&model_name, "_")));
    ensure_new_file(&model_path)?;

    save_generated_model(model_name.clone(), None).await?;

    // let base = std::env::current_dir()?.join("app/src/models");
    // let model_path = base.join(format!("{}.rs", model_name.to_lowercase()));

    let mut env = Environment::new();
    env.add_template("model", MODEL_TEMPLATE)?;
    let rendered = env.get_template("model")?.render(context! {
        name => model_name.clone(),
    })?;
    FileContent::put(model_path.to_str().unwrap(), &rendered).await?;

    // check migration and create
    if args.has_migration {
        let migration_name = format!("{}Create", &model_name);
        // let table = Str::plural_studly(&model_name, 3).to_lowercase();
        // BlogPost -> blog_posts
        let table = Str::plural(&Str::snake(&model_name, "_"), 2, false);

        // println!("Creating migration {}", table);
        let mig_args = NewMigArgs {
            name: migration_name,
            create: Some(table),
            path: None,
            table: None,
            realpath: false,
        };
        _ = migrate(&mig_args).await?;
    }

    // check controller and create
    if args.has_controller {
        let controller_name = format!("{}Controller", &model_name);
        let cont_args = NewControllerArgs {
            name: controller_name,
            model: Some(model_name.clone()),
        };
        controller(&cont_args).await?;
    }

    // check factory and create
    if args.has_factory {
        let factory_name = format!("{}Factory", &model_name);
        let fact_args = NewFactoryArgs {
            name: factory_name,
            model: Some(model_name.clone()),
        };
        factory(&fact_args).await?;
    }

    // check seeder and create
    if args.has_seeder {
        let seeder_name = format!("{}Seeder", &model_name);
        let sed_args = NewSeederArgs {
            name: seeder_name,
            model: Some(model_name.clone()),
        };
        seeder(&sed_args).await?;
    }

    // WIP: create controller
    if args.has_migration {
        // create controller here
    }
    operation(
        &format!("model and raw generated made: {:?}", model_name),
        start.elapsed(),
        Status::Done,
    );

    Ok(())
}
