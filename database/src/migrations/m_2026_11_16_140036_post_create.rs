use crate::migrator::Migration;
use async_trait::async_trait;
use rustavel_core::db::schema::Schema;
use rustavel_core::sql::database_client::DbError;

pub struct PostCreate;

#[async_trait]
impl Migration for PostCreate {
    async fn up(&self, schema: &mut Schema) -> Result<(), DbError> {
        schema.create("posts", |table| {
            table.id();
            table.string("title", 255).comment("post title");
            table
                .string("slug", 127)
                .unique()
                .index()
                .comment("post slug to route");
            table.long_text("body").nullable();
            table.big_integer("user_id").unsigned().comment("author user");
            table.foreign("user_id").on("users").reference("id");
            table.string("image", 1024).comment("post image");
            table.timestamps();
            // validate table after create
            let warnings = table.validate();
            for w in warnings {
                eprintln!("warning: {}", w);
            }
        });

        Ok(())
    }

    async fn down(&self, schema: &mut Schema) -> Result<(), DbError> {
        
        schema.drop_table("posts").await?;
        
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "m_2026_11_16_140036_post_create"
    }
}