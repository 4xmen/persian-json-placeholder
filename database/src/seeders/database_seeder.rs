use crate::seeders;
use crate::seeders::category_seeder::CategorySeeder;
use crate::seeders::comment_seeder::CommentSeeder;
use crate::seeders::post_seeder::PostSeeder;
use crate::seeders::product_seeder::ProductSeeder;
use crate::seeders::todo_seeder::TodoSeeder;
use crate::seeders::user_seeder::UserSeeder;
use anyhow::Result;
use rustavel_core::config::database::DbPoolType;
use rustavel_core::config::database::create_db_pool;

pub struct DatabaseSeeder;

#[allow(dead_code)]
pub struct SeederContext {
    pub(crate) pool: DbPoolType,
}
impl DatabaseSeeder {
    /**
     * Seed the application's database.
     */
    pub async fn run() -> Result<()> {
        let _ = seeders!(
            TodoSeeder,
            UserSeeder,
            CategorySeeder,
            ProductSeeder,
            PostSeeder,
            CommentSeeder
        );
        Ok(())
    }
}
