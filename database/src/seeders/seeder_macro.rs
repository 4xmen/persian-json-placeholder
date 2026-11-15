#[macro_export]
macro_rules! seeders {
    () => {{
        rustavel_core::helper::terminal_ui::title(TitleKind::Info, "Noting to seed");
        Ok::<(), anyhow::Error>(())
    }};

    ($($seeder:ty),+ $(,)?) => {{
        $(
            {

                let ctx = SeederContext{
                    client: create_db_client().await?,
                };

                // Start timing immediately before the seeder runs.
                // This measures only the execution time of this specific seeder.
                let start = std::time::Instant::now();

                // Run the seeder and wait for it to complete.
                let result = <$seeder>::run(ctx).await;

                match result {
                    Ok(_) => {
                        // If the seeder succeeded, print the same success-style message
                        // together with the elapsed time.
                        rustavel_core::helper::terminal_ui::operation( stringify!($seeder),start.elapsed(), rustavel_core::helper::terminal_ui::Status::Done );
                    }
                    Err(err) => {
                        // If the seeder failed, print the error plus the elapsed time.
                        // Then stop execution by returning the error to the caller.
                        rustavel_core::helper::terminal_ui::operation( stringify!($seeder),start.elapsed(), rustavel_core::helper::terminal_ui::Status::Failed );
                        return Err(err);
                    }
                }
            }
        )+

        Ok::<(), anyhow::Error>(())
    }};
}
