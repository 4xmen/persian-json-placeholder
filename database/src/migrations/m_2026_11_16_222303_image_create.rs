use crate::migrator::Migration;
use async_trait::async_trait;
use rustavel_core::db::schema::Schema;
use rustavel_core::sql::database_client::DbError;

pub struct ImageCreate;

#[async_trait]
impl Migration for ImageCreate {
    async fn up(&self, schema: &mut Schema) -> Result<(), DbError> {
        schema.create("images", |table| {
            table.id();
            table
                .big_integer("product_id")
                .unsigned()
                .comment("product id");
            table.string("image", 1024).comment("product image path");
            table.string("thumb", 1024).comment("product image path");

            table.foreign("product_id").on("products").reference("id");

            // validate table after create
            let warnings = table.validate();
            for w in warnings {
                eprintln!("warning: {}", w);
            }
        });

        Ok(())
    }

    async fn down(&self, schema: &mut Schema) -> Result<(), DbError> {
        schema.drop_table("images").await?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "m_2026_11_16_222303_image_create"
    }
}
