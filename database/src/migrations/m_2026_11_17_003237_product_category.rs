use crate::migrator::Migration;
use async_trait::async_trait;
use rustavel_core::db::schema::Schema;
use rustavel_core::sql::database_client::DbError;

pub struct ProductCategory;

#[async_trait]
impl Migration for ProductCategory {
    async fn up(&self, schema: &mut Schema) -> Result<(), DbError> {
        
        
        schema.create("product_category", |table| {

            table.big_integer("category_id").unsigned().nullable();
            table.big_integer("product_id").unsigned().nullable();
            table.foreign("category_id").on("categories").reference("id");
            table.foreign("product_id").on("products").reference("id");

            // validate table after change
                let warnings = table.validate();
                for w in warnings {
                    eprintln!("warning: {}", w);
                }
        });
        
        Ok(())
    }

    async fn down(&self, schema: &mut Schema) -> Result<(), DbError> {


        schema.drop_table("product_category").await?;
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "m_2026_11_17_003237_product_category"
    }
}