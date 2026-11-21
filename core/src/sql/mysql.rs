use super::generator::SqlGenerator;
use crate::config::CONFIG;
use crate::db::table::{
    Column, ColumnDataType, ColumnOption, DefaultValue, ForeignKey, TableAction,
};
use crate::logger;
use std::string::String;

#[derive(Debug)]
#[allow(dead_code)]
pub struct MySqlGenerator;

/// Escapes a value placed inside '...' in MySQL DDL.
fn quote_str(value: &str) -> String {
    value.replace('\\', "\\\\").replace('\'', "''")
}

#[allow(dead_code)]
impl MySqlGenerator {
    #[inline]
    fn db(&self) -> &'static str {
        "DATABASE()"
    }
}

impl SqlGenerator for MySqlGenerator {
    fn drop_table_if_exists(&self, table_name: &str) -> String {
        format!("DROP TABLE IF EXISTS `{}`;", table_name)
    }

    fn get_tables(&self) -> String {
        "
        SELECT table_name
        FROM information_schema.tables
        WHERE table_schema = DATABASE()
          AND table_type = 'BASE TABLE'
        "
        .to_string()
    }

    fn get_views(&self) -> String {
        "
        SELECT table_name
        FROM information_schema.views
        WHERE table_schema = DATABASE()
        "
        .to_string()
    }

    fn get_column_listing(&self, table_name: &str) -> String {
        format!(
            "
        SELECT column_name
        FROM information_schema.columns
        WHERE table_schema = DATABASE() AND TABLE_NAME = '{}';
        ",
            table_name
        )
    }

    fn get_foreign_keys(&self, table_name: &str) -> String {
        format!(
            "SELECT
            kcu.CONSTRAINT_NAME AS fk_name,
            kcu.TABLE_NAME AS table_name,
            kcu.COLUMN_NAME AS column_name,
            kcu.ORDINAL_POSITION AS ordinal_position,
            kcu.REFERENCED_TABLE_SCHEMA AS referenced_schema,
            kcu.REFERENCED_TABLE_NAME AS referenced_table,
            kcu.REFERENCED_COLUMN_NAME AS referenced_column,
            rc.UPDATE_RULE AS on_update,
            rc.DELETE_RULE AS on_delete
        FROM information_schema.KEY_COLUMN_USAGE AS kcu
        LEFT JOIN information_schema.REFERENTIAL_CONSTRAINTS AS rc
            ON rc.CONSTRAINT_SCHEMA = kcu.CONSTRAINT_SCHEMA
            AND rc.TABLE_NAME = kcu.TABLE_NAME
            AND rc.CONSTRAINT_NAME = kcu.CONSTRAINT_NAME
        WHERE
            kcu.CONSTRAINT_SCHEMA = DATABASE()
            AND kcu.TABLE_NAME = '{}'
            AND kcu.REFERENCED_TABLE_NAME IS NOT NULL
        ORDER BY
            kcu.CONSTRAINT_NAME,
            kcu.ORDINAL_POSITION;
            ",
            table_name
        )
    }

    fn drop_table(&self, table_name: &str) -> String {
        format!("DROP TABLE `{}`;", table_name)
    }

    fn drop_view(&self, view_name: &str) -> String {
        format!("DROP VIEW `{}`;", view_name)
    }

    // fn drop_all_tables(&self) -> String {
    //     "
    //     SET FOREIGN_KEY_CHECKS = 0;
    //     SELECT CONCAT('DROP TABLE IF EXISTS `', table_name, '`;')
    //     FROM information_schema.tables
    //     WHERE table_schema = DATABASE();
    //     SET FOREIGN_KEY_CHECKS = 1;
    //     "
    //     .to_string()
    // }

    // fn drop_all_views(&self) -> String {
    //     "
    //     SELECT CONCAT('DROP VIEW IF EXISTS `', table_name, '`;')
    //     FROM information_schema.views
    //     WHERE table_schema = DATABASE();
    //     "
    //     .to_string()
    // }

    fn has_column(&self, table_name: &str, column_name: &str) -> String {
        format!(
            "
            SELECT '1'
            FROM information_schema.columns
            WHERE table_schema = DATABASE()
              AND table_name = '{}'
              AND column_name = '{}'
            LIMIT 1
            ",
            table_name, column_name
        )
    }

    fn has_table(&self, table_name: &str) -> String {
        format!(
            "
            SELECT '1'
            FROM information_schema.tables
            WHERE table_schema = DATABASE()
              AND table_name = '{}'
            LIMIT 1
            ",
            table_name
        )
    }

    fn has_view(&self, table_name: &str) -> String {
        format!(
            "
            SELECT '1'
            FROM information_schema.views
            WHERE table_schema = DATABASE()
              AND table_name = '{}'
            LIMIT 1
            ",
            table_name
        )
    }

    fn has_index(&self, table_name: &str, columns_name: Vec<&str>) -> String {
        let cols = columns_name
            .iter()
            .map(|c| format!("'{}'", c))
            .collect::<Vec<_>>()
            .join(",");

        let count = columns_name.len();

        format!(
            "
            SELECT '1'
            FROM information_schema.statistics
            WHERE table_schema = DATABASE()
              AND table_name = '{}'
              AND column_name IN ({})
            GROUP BY index_name
            HAVING COUNT(*) = {}
            LIMIT 1
            ",
            table_name, cols, count
        )
    }

    fn create_database(&self, db_name: &str) -> String {
        format!("CREATE DATABASE `{}`;", db_name)
    }

    fn drop_database_if_exists(&self, db_name: &str) -> String {
        format!("DROP DATABASE IF EXISTS `{}`;", db_name)
    }

    fn disable_foreign_key_constraints(&self) -> String {
        "SET FOREIGN_KEY_CHECKS = 0;".to_string()
    }

    fn enable_foreign_key_constraints(&self) -> String {
        "SET FOREIGN_KEY_CHECKS = 1;".to_string()
    }

    fn rename(&self, old_table_name: &str, new_table_name: &str) -> String {
        format!("RENAME TABLE `{}` TO `{}`;", old_table_name, new_table_name)
    }

    fn column(
        &self,
        column: &Column,
        table_name: &str,
        action: &TableAction,
    ) -> (String, String, String, String) {
        // for multi-col field like morph
        let mut extra_cols: Vec<String> = Vec::new();
        let mut column_sql = String::new();
        // let mut footer_sql = String::new();
        // let nullable = match column.nullable {

        // a column can need several keys/constraints (e.g. JSON check + index)
        let mut footers: Vec<String> = Vec::new();
        let nullable = match column.nullable {
            true => "NULL",
            false => "NOT NULL",
        };

        let mut collation = if column.collation.is_empty() {
            ""
        } else {
            &column.collation
        };

        let unsigned = if column.unsigned { "UNSIGNED" } else { "" };

        let def = match &column.default {
            DefaultValue::Null => "DEFAULT NULL",
            // DefaultValue::String(str) => &format!(" DEFAULT '{}' ", str),
            DefaultValue::String(str) => &format!(" DEFAULT '{}' ", quote_str(str)),
            DefaultValue::JsonArray => "DEFAULT json_array()",
            DefaultValue::CurrenTimestamp => "DEFAULT current_timestamp()",
            DefaultValue::Bool(bool_val) => {
                if *bool_val {
                    "DEFAULT b'1'"
                } else {
                    "DEFAULT  b'0'"
                }
            }
            DefaultValue::Int(int_val) => &format!("DEFAULT '{}'", int_val),
            _ => "",
        };

        match column.data_type {
            ColumnDataType::DTId => {
                column_sql = "`id` BIGINT(20) UNSIGNED NOT NULL AUTO_INCREMENT".to_string();
                // footer_sql = "PRIMARY KEY (`id`)".to_string();
                footers.push("PRIMARY KEY (`id`)".to_string());
            }
            ColumnDataType::DTBoolean => {
                // cuz some time problem in insert
                // column_sql = format!("`{}` BIT(1) {} {}", column.name, nullable, def);
                column_sql = format!("`{}` TINYINT(1) {} {}", column.name, nullable, def);
            }
            ColumnDataType::DTTinyInteger => {
                column_sql = format!(
                    "`{}` TINYINT {} {} {}",
                    column.name, unsigned, nullable, def
                );
            }
            ColumnDataType::DTInteger => {
                column_sql = format!("`{}` INT {} {} {}", column.name, unsigned, nullable, def);
            }
            ColumnDataType::DTSmallInteger => {
                column_sql = format!(
                    "`{}` SMALLINT {} {} {}",
                    column.name, unsigned, nullable, def
                );
            }
            ColumnDataType::DTMediumInteger => {
                column_sql = format!(
                    "`{}` MEDIUMINT {} {} {}",
                    column.name, unsigned, nullable, def
                );
            }
            ColumnDataType::DTBigInteger => {
                // BIGINT(20) why `20` ? cuz compatible with id foreign key
                column_sql = format!(
                    "`{}` BIGINT(20) {} {} {}",
                    column.name, unsigned, nullable, def
                );
            }
            ColumnDataType::DTFloat => {
                column_sql = format!("`{}` FLOAT {} {} {}", column.name, unsigned, nullable, def);
            }
            ColumnDataType::DTDouble => {
                column_sql = format!("`{}` DOUBLE {} {} {}", column.name, unsigned, nullable, def);
            }
            ColumnDataType::DTDecimal => {
                let (precision, scale) = match &column.option {
                    ColumnOption::Float((p, s)) => (i32::from(*p), i32::from(*s)),
                    _ => (20, 6),
                };
                column_sql = format!(
                    "`{}` DECIMAL({},{}) {} {} {}",
                    column.name, precision, scale, unsigned, nullable, def
                );
            }
            ColumnDataType::DTString => {
                let len = match column.option {
                    ColumnOption::Length(l) => l,
                    _ => 127,
                };
                column_sql = format!("`{}` VARCHAR({}) {} {}", column.name, len, nullable, def);
            }
            ColumnDataType::DTText => {
                column_sql = format!("`{}` TEXT {} {}", column.name, nullable, def);
            }
            ColumnDataType::DTTinyText => {
                column_sql = format!("`{}` TINYTEXT {} {}", column.name, nullable, def);
            }
            ColumnDataType::DTMediumText => {
                column_sql = format!("`{}` MEDIUMTEXT {} {}", column.name, nullable, def);
            }
            ColumnDataType::DTLongText => {
                column_sql = format!("`{}` LONGTEXT {} {}", column.name, nullable, def);
            }
            ColumnDataType::DTJson => {
                column_sql = format!("`{}` LONGTEXT {} {}", column.name, nullable, def);
                // footer_sql = format!("CONSTRAINT `{}` CHECK (json_valid(`data`))", column.name);
                footers.push(format!(
                    "CONSTRAINT `{}_{}_json` CHECK (json_valid(`{}`))",
                    table_name, column.name, column.name
                ));
                if collation.is_empty() {
                    collation = "utf8mb4_bin";
                }
            }
            ColumnDataType::DTDate => {
                column_sql = format!("`{}` DATE {} {}", column.name, nullable, def);
            }
            ColumnDataType::DTDateTime => {
                column_sql = format!("`{}` DATETIME {} {}", column.name, nullable, def);
            }
            ColumnDataType::DTTime => {
                column_sql = format!("`{}` TIME {} {}", column.name, nullable, def);
            }
            ColumnDataType::DTTimestamp => {
                column_sql = format!("`{}` TIMESTAMP {} {}", column.name, nullable, def);
            }
            // Use DATETIME instead of TIMESTAMP for system-managed columns
            // (created_at, updated_at, deleted_at).
            //
            // Reasons:
            // - DATETIME has a wider supported date range than TIMESTAMP.
            // - DATETIME does not perform implicit timezone conversions
            //   during database storage and retrieval.
            // - DATETIME maps directly to time::PrimitiveDateTime in SQLx,
            //   ensuring type compatibility during encoding and decoding.
            // - Using UTC as a consistent application-level convention
            //   avoids timezone ambiguity across different environments.
            //
            // All system-managed datetime values must be stored in UTC.
            ColumnDataType::DTTimestamps => {
                column_sql = "`created_at` DATETIME NULL DEFAULT NULL, `updated_at` DATETIME NULL DEFAULT NULL".to_string();
                extra_cols.push("`updated_at` DATETIME NULL DEFAULT NULL".to_string());
            }
            ColumnDataType::DTSoftDelete => {
                column_sql = "`deleted_at` DATETIME NULL DEFAULT NULL".to_string();
            }
            ColumnDataType::DTEnum => {
                let enums = match &column.option {
                    ColumnOption::Values(items) => items
                        .iter()
                        // .map(|item| format!("'{}'", item))
                        .map(|item| format!("'{}'", quote_str(item)))
                        .collect::<Vec<_>>()
                        .join(", "),
                    _ => "''".to_string(),
                };
                column_sql = format!("`{}` ENUM({}) {} {}", column.name, enums, nullable, def);
            }
            ColumnDataType::DTSet => {
                let sets = match &column.option {
                    ColumnOption::Values(items) => items
                        .iter()
                        // .map(|item| format!("'{}'", item))
                        .map(|item| format!("'{}'", quote_str(item)))
                        .collect::<Vec<_>>()
                        .join(", "),
                    _ => "''".to_string(),
                };
                column_sql = format!("`{}` SET({}) {} {}", column.name, sets, nullable, def);
            }
            ColumnDataType::DTMorph => {
                column_sql = format!(
                    "`{}_type` VARCHAR(255) {} {} ,\
                `{}_id` BIGINT(20) UNSIGNED {} {}",
                    column.name, nullable, def, column.name, nullable, def
                );
                footers.push(format!(
                    "INDEX `morph_{}_type_{}_id_index` (`{}_type`, `{}_id`)",
                    column.name, column.name, column.name, column.name
                ));
            }
            _ => {}
        }

        if !column.comment.is_empty() {
            // column_sql = format!("{} COMMENT '{}' ", column_sql, column.comment);
            column_sql = format!("{} COMMENT '{}' ", column_sql, quote_str(&column.comment));
        }
        if !collation.is_empty() && column.is_string_type() {
            column_sql = format!("{} COLLATE '{}' ", column_sql, collation);
        }

        // if column.index {
        //     footer_sql = format!("INDEX `{}` (`{}`)", column.name, column.name);
        // }
        // if column.unique {
        //     footer_sql = format!(
        //         "UNIQUE INDEX `{}_{}_unique` (`{}`)",
        //         table_name, column.name, column.name
        //     );
        // }
        // if *action == TableAction::Alter {
        //     if column.change {
        //         column_sql = format!("CHANGE COLUMN `{}` {}", column.name, column_sql);
        //     } else {
        //         column_sql = format!("ADD COLUMN {}", column_sql);
        //     }
        //     if column.index {
        //         footer_sql = format!("ADD {}",footer_sql);
        //     }
        // } else {
        //     if column.change {
        //         logger::warn(&format!(
        //             "You can't change column while trying to create table, column: `{}`",
        //             column.name
        //         ));
        //     }
        // }
        // (column_sql, footer_sql, String::new(),String::from(",\n"))

        if column.unique {
            footers.push(format!(
                "UNIQUE INDEX `{}_{}_unique` (`{}`)",
                table_name, column.name, column.name
            ));
        } else if column.index {
            footers.push(format!("INDEX `{}` (`{}`)", column.name, column.name));
        }
        if *action == TableAction::Alter {
            if column.change {
                column_sql = format!("CHANGE COLUMN `{}` {}", column.name, column_sql);
            } else {
                column_sql = format!("ADD COLUMN {}", column_sql);
            }
            // inside ALTER TABLE every key/constraint needs its own ADD
            footers = footers.into_iter().map(|f| format!("ADD {}", f)).collect();
        } else if column.change {
            logger::warn(&format!(
                "You can't change column while trying to create table, column: `{}`",
                column.name
            ));
        }
        (
            column_sql,
            footers.join(",\n"),
            String::new(),
            String::from(",\n"),
        )
    }

    fn foreign_key(&self, key: &ForeignKey, table_name: &str, action: &TableAction) -> String {
        // let update = match key.on_update {
        //     true => "ON DELETE CASCADE",
        //     false => "",
        // };
        // let delete = match key.on_delete {
        //     true => "ON UPDATE CASCADE",
        //     false => "",
        // };

        let update = match key.on_update {
            true => "ON UPDATE CASCADE",
            false => "",
        };
        let delete = match key.on_delete {
            true => "ON DELETE CASCADE",
            false => "",
        };

        let prefix = match *action {
            TableAction::Alter => "ADD ",
            _ => "",
        };
        format!(
            "{} CONSTRAINT `{}_{}_foreign` FOREIGN KEY (`{}`) \
        REFERENCES `{}` (`{}`) \
         {} {}",
            prefix,
            table_name,
            key.column_name,
            key.column_name,
            key.foreign_table,
            key.referenced_column,
            update,
            delete
        )
    }

    // fn drop_column(&self, column_name: &str) -> String {
    fn drop_column(&self, _table_name: &str, column_name: &str) -> String {
        format!("DROP COLUMN `{}`", column_name)
    }

    fn table_sql(
        &self,
        table_name: &str,
        body_sql: &str,
        post_sql: &str,
        action: &TableAction,
    ) -> String {
        match action {
            TableAction::Create => {
                let collection = CONFIG.database.collection.clone();
                format!(
                    "CREATE TABLE `{}` ( \n {} \n ) COLLATE='{}' ENGINE=InnoDB; \n {}",
                    table_name, body_sql, collection, post_sql
                )
            }
            TableAction::Alter => {
                format!(
                    "ALTER TABLE `{}` \n {} ; \n {}",
                    table_name, body_sql, post_sql
                )
            }
            _ => "".to_string(),
        }
    }

    fn get_ran(&self, table: &str) -> String {
        format!("SELECT `migration` FROM `{}`", table)
    }
    fn get_ran_gt(&self, table: &str) -> String {
        format!("SELECT `migration` FROM `{}` WHERE `batch` > ?", table)
    }

    fn get_next_batch_number(&self, table: &str) -> String {
        format!("SELECT MAX(batch) AS 'batch' FROM `{}`", table)
    }

    fn add_migrated_table(&self, table: &str) -> String {
        format!(
            "INSERT INTO `{}` (`migration`, `batch`) VALUES (?, ?)",
            table
        )
    }

    fn rem_migrated_table(&self, table: &str) -> String {
        format!("DELETE FROM `{}` WHERE  `migration` = ?", table)
    }

    fn record_exists(&self, table: &str, column: &str) -> String {
        format!(
            "SELECT COUNT(*) AS 'count' FROM `{}` WHERE `{}` = ?",
            table, column
        )
    }
    fn record_exists_except(&self, table: &str, column: &str, except: &str) -> String {
        format!(
            "SELECT COUNT(*) AS 'count' FROM `{}` WHERE `{}` = ? AND `{}` <> ?",
            table, column, except
        )
    }

    fn get_db_tables_columns_struct(&self) -> String {
        "SELECT CONCAT(TABLE_NAME, '.', COLUMN_NAME) AS name
        FROM INFORMATION_SCHEMA.COLUMNS
        WHERE TABLE_SCHEMA = DATABASE()
        ORDER BY TABLE_NAME, ORDINAL_POSITION;"
            .into()
    }

    fn rename_column(&self, _table_name: &str, old_col_name: &str, new_col_name: &str) -> String {
        format!("RENAME COLUMN `{}` TO `{}`", old_col_name, new_col_name)
    }

    fn drop_foreign_key(&self, constraint_name: &str) -> String {
        format!("DROP FOREIGN KEY {}", constraint_name)
    }
}
