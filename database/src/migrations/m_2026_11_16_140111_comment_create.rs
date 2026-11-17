use crate::migrator::Migration;
use async_trait::async_trait;
use rustavel_core::db::schema::Schema;
use rustavel_core::sql::database_client::DbError;

pub struct CommentCreate;

#[async_trait]
impl Migration for CommentCreate {
    async fn up(&self, schema: &mut Schema) -> Result<(), DbError> {
        schema.create("comments", |table| {
            table.id();
            table.medium_text("body");
            table.string("author", 100).comment("comentator name");
            table.string("email", 100).comment("comentator email");
            table.morph("commentable","commentable");
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
        
        schema.drop_table("comments").await?;
        
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "m_2026_11_16_140111_comment_create"
    }
}