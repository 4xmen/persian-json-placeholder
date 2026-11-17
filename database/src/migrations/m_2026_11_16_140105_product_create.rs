use crate::migrator::Migration;
use async_trait::async_trait;
use rustavel_core::db::schema::Schema;
use rustavel_core::sql::database_client::DbError;

pub struct ProductCreate;

#[async_trait]
impl Migration for ProductCreate {
    async fn up(&self, schema: &mut Schema) -> Result<(), DbError> {

        schema.create("products", |table| {
            table.id();
            table.string("name", 127).comment("product name");
            table.long_text("description").nullable().comment("product description");
            table
                .string("slug", 127)
                .unique()
                .index()
                .comment("product slug to route");
            table.big_integer("price").unsigned().default_int(0).comment("product price");
            table.integer("quantity").default_int(0).comment("product quantity");
            table.big_integer("category_id").unsigned().nullable();
            table.big_integer("user_id").unsigned().comment("author user");
            table.string("image",1024).nullable().default_null();
            table.string("thumb",1024).nullable().default_null();
            table.timestamps();

            table.foreign("category_id").on("categories").reference("id");
            table.foreign("user_id").on("users").reference("id");
            // validate table after create
            let warnings = table.validate();
            for w in warnings {
                eprintln!("warning: {}", w);
            }
        });


        Ok(())
    }

    async fn down(&self, schema: &mut Schema) -> Result<(), DbError> {
        
        schema.drop_table("products").await?;
        
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "m_2026_11_16_140105_product_create"
    }
}