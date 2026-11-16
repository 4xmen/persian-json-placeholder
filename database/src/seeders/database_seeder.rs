use crate::seeders;
use crate::seeders::todo_seeder::TodoSeeder;
use anyhow::Result;
use rustavel_core::sql::database_client::create_db_pool;
use sqlx::AnyPool;

pub struct DatabaseSeeder;

#[allow(dead_code)]
pub struct SeederContext {
    pub(crate) pool: AnyPool,
}
impl DatabaseSeeder {
    /**
     * Seed the application's database.
     */
    pub async fn run() -> Result<()> {
        let _ = seeders!(TodoSeeder,);
        Ok(())
    }
}
