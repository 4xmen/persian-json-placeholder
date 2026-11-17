use crate::migrator::Migration;
use async_trait::async_trait;
use rustavel_core::db::schema::Schema;
use rustavel_core::sql::database_client::DbError;

pub struct CategoryCreate;

#[async_trait]
impl Migration for CategoryCreate {
    async fn up(&self, schema: &mut Schema) -> Result<(), DbError> {
        schema.create("categories", |table| {
            table.id();
            table.string("name", 127).comment("category name");
            table.long_text("description").nullable().comment("category description");
            table
                .string("slug", 127)
                .unique()
                .index()
                .comment("category slug to route");
            table.big_integer("parent_id").unsigned().nullable();
            table.string("image",1024).nullable().default_null();
            table.string("thumb",1024).nullable().default_null();

            table.foreign("parent_id").on("categories").reference("id");
            // validate table after create
            let warnings = table.validate();
            for w in warnings {
                eprintln!("warning: {}", w);
            }
        });

        Ok(())
    }

    async fn down(&self, schema: &mut Schema) -> Result<(), DbError> {
        
        schema.drop_table("categories").await?;
        
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "m_2026_11_16_140054_category_create"
    }
}