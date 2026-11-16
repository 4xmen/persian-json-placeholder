// Bug 12: drops EVERY table in the configured database (rustavel_test)
use rustavel_core::config::{CONFIG, database::DatabaseEngine};
use rustavel_core::db::schema::Schema;
use std::time::Instant;

#[tokio::test]
async fn bug12_drop_all_tables_with_foreign_keys() {
    if CONFIG.app.env != "dev" {
        eprintln!("skipped: SAFETY — destructive database test requires dev environment");
        return;
    }
    if CONFIG.database.connection != DatabaseEngine::Mysql {
        eprintln!("skipped: needs MySQL");
        return;
    }
    let mut s = Schema::new().await.unwrap();
    // leftovers from an earlier failed run
    for i in 0..6 {
        s.drop_table_if_exists(format!("reg_child{i}")).await;
    }
    s.drop_table_if_exists("reg_parent").await;
    for round in 1..=5 {
        s.create("reg_parent", |t| {
            t.id();
        });
        for i in 0..6 {
            s.create(format!("reg_child{i}"), |t| {
                t.id();
                t.big_integer("parent_id").unsigned();
                t.foreign("parent_id").on("reg_parent").reference("id");
            });
        }
        s.execute_migration("fk tables", &Instant::now().into())
            .await
            .unwrap();
        let r = s.drop_all_tables().await;
        assert!(
            r.is_ok(),
            "round {round}: {:?}",
            r.err().map(|e| e.to_string())
        );
        assert!(
            s.get_tables().await.unwrap().is_empty(),
            "round {round}: tables left"
        );
    }
}
