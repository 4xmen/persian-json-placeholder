use crate::make::make_error::MakeError;
use illuminate_str::Str;
use minijinja::Environment;
use minijinja::value::Serde;
use rustavel_core::db::schema::ModelData;
use rustavel_core::helper::file::project_root;
use rustavel_core::helper::file_content::FileContent;
const MODEL_GENERATED_TEMPLATE: &str = include_str!("../make/templates/model_generated.rs.j2");

#[derive(serde::Serialize)]
struct ModelContext {
    name: String,
    fields: String,
    table: String,
    pkey: String,
    field_list: String,
    pkey_type: String,
}

pub async fn save_generated_model(
    model_name: String,
    data: Option<ModelData>,
) -> Result<(), MakeError> {
    let mut env = Environment::new();
    env.add_template("model_generated", MODEL_GENERATED_TEMPLATE)?;
    let base = project_root()?.join("app/src/models");
    // let generated_path = base.join(format!("{}_generated.rs", model_name.to_lowercase()));
    let generated_path = base.join(format!("{}_generated.rs", Str::snake(&model_name, "_")));

    let ctx = if let Some(model_data) = data {
        // let pkey_type = if is_mysql() { "u64" } else { "i64" }.into();
        let pkey_type = "PKeyType".to_string();
        ModelContext {
            name: model_name,
            fields: model_data.model.trim().to_owned(),
            field_list: model_data.columns,
            table: model_data.table,
            pkey: model_data.primary_key,
            pkey_type,
        }
    } else {
        ModelContext {
            name: model_name,
            fields: "".to_string(),
            field_list: "".to_string(),
            table: "".to_string(),
            pkey: "".to_string(),
            pkey_type: "".to_string(),
        }
    };

    let rendered = env.get_template("model_generated")?.render(Serde(&ctx))?;

    // println!("{}", rendered); // debug print

    FileContent::put(generated_path.to_str().unwrap(), &rendered).await?;

    Ok(())
}
