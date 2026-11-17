use crate::config::CONFIG;
use crate::config::database::DatabaseEngine;
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub comment: String,
    pub action: TableAction,
    foreign_keys: Vec<ForeignKey>,
    drop_columns: Vec<String>,
    drop_foreign_keys: Vec<String>,
    rename_columns: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TableAction {
    None,
    Create,
    Alter,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnDataType {
    DTId, // add id
    DTBoolean,
    DTTinyInteger,
    DTInteger,
    DTSmallInteger,
    DTMediumInteger,
    DTBigInteger,
    DTFloat,
    DTDouble,
    DTDecimal,
    DTString,
    DTText,
    DTTinyText,
    DTMediumText,
    DTLongText,
    DTJson,
    DTDate,
    DTDateTime,
    DTTime,
    DTTimestamp,
    DTTimestamps, // to add created_at and updated_at
    DTSoftDelete,
    DTEnum,
    DTSet,
    DTMorph,
    DTNone,
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: ColumnDataType,
    pub nullable: bool,
    pub option: ColumnOption,
    pub comment: String,
    pub unique: bool,
    pub index: bool,
    pub unsigned: bool,
    pub default: DefaultValue,
    pub change: bool,
    pub collation: String,
}

pub struct ColumnBuilder<'a> {
    table: &'a mut Table,
    column: Column,
}
pub struct ForeignKeyBuilder<'a> {
    table: &'a mut Table,
    key: ForeignKey,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ColumnOption {
    None,
    Length(i32),
    Precision(i8),
    Values(Vec<String>),
    Float((i8, i8)),
    Index(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DefaultValue {
    None,
    Null,
    JsonArray,
    Bool(bool),
    Int(i64),
    String(String),
    CurrenTimestamp,
}

#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub column_name: String,
    pub foreign_table: String,
    pub referenced_column: String,
    pub on_delete: bool,
    pub on_update: bool,
}

#[allow(dead_code)]
impl Table {
    /// create new instance of table
    pub fn new(table_name: &str) -> Self {
        Self {
            name: table_name.to_string(),
            columns: Vec::new(),
            foreign_keys: Vec::new(),
            comment: String::new(),
            action: TableAction::None,
            drop_columns: Vec::new(),
            drop_foreign_keys: Vec::new(),
            rename_columns: HashMap::new(),
        }
    }

    pub fn get_rename_columns(&self) -> &HashMap<String, String> {
        &self.rename_columns
    }

    pub fn get_drop_columns(&self) -> &Vec<String> {
        &self.drop_columns
    }

    pub fn get_foreign_keys(&self) -> &Vec<ForeignKey> {
        &self.foreign_keys
    }

    pub fn get_drop_foreign_keys(&self) -> &Vec<String> {
        &self.drop_foreign_keys
    }

    /// add comment to table
    pub fn table_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.comment = comment.into();
        self
    }

    /// create new column
    fn column(
        &mut self,
        name: impl Into<String>,
        column_data: ColumnDataType,
        option: ColumnOption,
    ) -> ColumnBuilder<'_> {
        ColumnBuilder {
            table: self,
            column: Column {
                name: name.into(),
                data_type: column_data,
                option,
                nullable: false,
                unique: false,
                index: false,
                default: DefaultValue::None,
                comment: String::new(),
                unsigned: false,
                change: false,
                collation: String::new(),
            },
        }
    }

    /// this function convert table to rust struct
    /// note: maybe need move this function to schema
    pub fn to_struct(&self) -> String {
        let mut result = "".to_string();

        for col in &self.columns {
            let field = Self::make_field(col);
            result += &format!("    {},\n", field);
        }
        result
    }

    /// make field datatype of table
    fn make_field(column: &Column) -> String {
        // map ColumnDataType -> Rust type
        let mut rust_type = match column.data_type {
            ColumnDataType::DTId => "i64".to_string(),
            ColumnDataType::DTBoolean => "bool".to_string(),
            ColumnDataType::DTTinyInteger => "i8".to_string(),
            ColumnDataType::DTSmallInteger => "i16".to_string(),
            ColumnDataType::DTMediumInteger => "i32".to_string(),
            ColumnDataType::DTInteger => "i32".to_string(),
            ColumnDataType::DTBigInteger => "i64".to_string(),
            ColumnDataType::DTFloat => "f32".to_string(),
            ColumnDataType::DTDouble => "f64".to_string(),
            ColumnDataType::DTDecimal => "f64".to_string(),
            ColumnDataType::DTString
            | ColumnDataType::DTText
            | ColumnDataType::DTTinyText
            | ColumnDataType::DTMediumText
            | ColumnDataType::DTLongText => "String".to_string(),
            ColumnDataType::DTJson => "serde_json::Value".to_string(),
            ColumnDataType::DTDate => "time::Date".to_string(),

            // ColumnDataType::DTDateTime
            // | ColumnDataType::DTTimestamp
            // | ColumnDataType::DTTimestamps => "time::PrimitiveDateTime".to_string(),
            ColumnDataType::DTDateTime | ColumnDataType::DTTimestamp => {
                "time::PrimitiveDateTime".to_string()
            }
            // created_at / updated_at are created as NULL-able columns
            ColumnDataType::DTTimestamps => "Option<time::PrimitiveDateTime>".to_string(),
            ColumnDataType::DTTime => "time::Time".to_string(),
            ColumnDataType::DTSoftDelete => "Option<time::PrimitiveDateTime>".to_string(),
            ColumnDataType::DTEnum | ColumnDataType::DTSet => "String".to_string(),
            // ColumnDataType::DTMorph => "i64".to_string(),
            ColumnDataType::DTMorph => if CONFIG.database.connection == DatabaseEngine::Mysql {
                "u64"
                // "i64"  // we choose this yet maybe need change
            } else {
                "i64"
            }
            .to_string(),
            ColumnDataType::DTNone => "()".to_string(),
        };

        if column.nullable && !rust_type.starts_with("Option") {
            rust_type = format!("Option<{}>", rust_type);
        }

        // if column.unsigned && CONFIG.database.connection == DatabaseEngine::Mysql {
        //     rust_type = rust_type.replace("i", "u");
        // }

        if column.unsigned && CONFIG.database.connection == DatabaseEngine::Mysql {
            // replace("i", "u") also rewrote names like `time::PrimitiveDateTime`
            for (signed, unsigned) in [("i8", "u8"), ("i16", "u16"), ("i32", "u32"), ("i64", "u64")]
            {
                if rust_type.contains(signed) {
                    rust_type = rust_type.replace(signed, unsigned);
                    break;
                }
            }
        }

        // if column.data_type == ColumnDataType::DTTimestamps {
        //     format!(
        //         "pub created_at: {},\n    pub updated_at: {}",
        //         rust_type, rust_type
        //     )
        // } else {
        if column.data_type == ColumnDataType::DTTimestamps {
            format!(
                "pub created_at: {},\n    pub updated_at: {}",
                rust_type, rust_type
            )
        } else if column.data_type == ColumnDataType::DTMorph {
            let type_ty = if column.nullable {
                "Option<String>"
            } else {
                "String"
            };
            format!(
                "pub {0}_type: {1},\n    pub {0}_id: {2}",
                column.name, type_ty, rust_type
            )
        } else {
            format!("pub {}: {}", column.name, rust_type)
        }
    }

    // --------------------------------------------------------------------------------------------
    /// add id column id
    pub fn id(&mut self) -> ColumnBuilder<'_> {
        self.column("id", ColumnDataType::DTId, ColumnOption::None)
            .unsigned()
    }

    /// add boolean column
    pub fn boolean(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTBoolean, ColumnOption::None)
    }

    /// add string column
    pub fn string(&mut self, name: impl Into<String>, len: i32) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTString, ColumnOption::Length(len))
    }

    /// add text column
    pub fn text(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTText, ColumnOption::None)
    }

    /// add tiny text column
    pub fn tiny_text(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTTinyText, ColumnOption::None)
    }

    /// add medium text column
    pub fn medium_text(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTMediumText, ColumnOption::None)
    }

    /// add long text column
    pub fn long_text(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTLongText, ColumnOption::None)
    }

    /// add json  column
    pub fn json(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTJson, ColumnOption::None)
    }

    // add integer column
    pub fn integer(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTInteger, ColumnOption::None)
    }

    /// add tiny integer column
    pub fn tiny_integer(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTTinyInteger, ColumnOption::None)
    }

    /// add small integer column
    pub fn small_integer(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTSmallInteger, ColumnOption::None)
    }

    /// add medium integer column
    pub fn medium_integer(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTMediumInteger, ColumnOption::None)
    }

    /// add big integer column
    pub fn big_integer(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTBigInteger, ColumnOption::None)
    }

    /// add double column
    pub fn double(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTDouble, ColumnOption::None)
    }

    /// add float column
    pub fn float(&mut self, name: impl Into<String>, precision: i8) -> ColumnBuilder<'_> {
        self.column(
            name,
            ColumnDataType::DTFloat,
            ColumnOption::Precision(precision),
        )
    }

    /// add decimal column
    pub fn decimal(&mut self, name: impl Into<String>, total: i8, place: i8) -> ColumnBuilder<'_> {
        self.column(
            name,
            ColumnDataType::DTDecimal,
            ColumnOption::Float((total, place)),
        )
    }

    /// add date column
    pub fn date(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTDate, ColumnOption::None)
    }

    /// add datetime column
    pub fn datetime(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTDateTime, ColumnOption::None)
    }

    /// add time column
    pub fn time(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTTime, ColumnOption::None)
    }

    /// add timestamp column
    pub fn timestamp(&mut self, name: impl Into<String>) -> ColumnBuilder<'_> {
        self.column(name, ColumnDataType::DTTimestamp, ColumnOption::None)
    }

    /// add timestamps columns (created_at, updated_at)
    pub fn timestamps(&mut self) -> ColumnBuilder<'_> {
        self.column("", ColumnDataType::DTTimestamps, ColumnOption::None)
    }

    /// add soft delete column (deleted_at)
    pub fn soft_delete(&mut self) -> ColumnBuilder<'_> {
        self.column(
            "deleted_at",
            ColumnDataType::DTSoftDelete,
            ColumnOption::None,
        )
    }

    /// add morph columns ({morph}_type, {morph}_id)
    pub fn morph(
        &mut self,
        name: impl Into<String>,
        index_name: impl Into<String>,
    ) -> ColumnBuilder<'_> {
        self.column(
            name,
            ColumnDataType::DTMorph,
            ColumnOption::Index(index_name.into()),
        )
    }

    /// add nullable morph columns ({morph}_type, {morph}_id)
    pub fn nullable_morphs(
        &mut self,
        name: impl Into<String>,
        index_name: impl Into<String>,
    ) -> ColumnBuilder<'_> {
        self.column(
            name,
            ColumnDataType::DTMorph,
            ColumnOption::Index(index_name.into()),
        )
        .nullable()
    }

    /// add enum column
    pub fn enums<I, S>(&mut self, name: impl Into<String>, values: I) -> ColumnBuilder<'_>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let values = values.into_iter().map(Into::into).collect();
        self.column(name, ColumnDataType::DTEnum, ColumnOption::Values(values))
    }

    /// add set column
    pub fn sets<I, S>(&mut self, name: impl Into<String>, values: I) -> ColumnBuilder<'_>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let values = values.into_iter().map(Into::into).collect();
        self.column(name, ColumnDataType::DTSet, ColumnOption::Values(values))
    }

    // --------------------------------------------------------------------------------------------

    /// add foreign key
    pub fn foreign(&mut self, name: impl Into<String>) -> ForeignKeyBuilder<'_> {
        ForeignKeyBuilder {
            table: self,
            key: ForeignKey {
                column_name: name.into(),
                foreign_table: String::new(),
                referenced_column: String::new(),
                on_update: false,
                on_delete: false,
            },
        }
    }

    // --------------------------------------------------------------------------------------------

    /// validate column created by developer
    pub fn validate(&self) -> Vec<String> {
        let mut errors = vec![];

        for fk in &self.foreign_keys {
            if !fk.validate() {
                errors.push(format!(
                    "invalid foreign key: {},{},{}",
                    self.name, fk.column_name, fk.referenced_column
                ));
            }
        }

        for col in &self.columns {
            if !col.validate() {
                errors.push(format!("invalid column: {},{}", self.name, col.name));
            }
        }

        if self
            .columns
            .iter()
            .filter(|c| c.data_type == ColumnDataType::DTTimestamps)
            .count()
            > 1
        {
            errors.push(format!("duplicate timestamps: {}", self.name));
        }

        errors
    }

    /// drop column
    pub fn drop_column(&mut self, name: impl Into<String>) {
        self.drop_columns.push(name.into());
    }

    pub fn rename_column(&mut self, old_name: impl Into<String>, new_name: impl Into<String>) {
        self.rename_columns.insert(old_name.into(), new_name.into());
    }

    pub fn drop_foreign(&mut self, constraint_name: impl Into<String>) {
        self.drop_foreign_keys.push(constraint_name.into());
    }
    // --------------------------------------------------------------------------------------------
}

impl Column {
    /// validate column fields
    fn validate(&self) -> bool {
        match self.data_type {
            ColumnDataType::DTNone => return false,

            ColumnDataType::DTString
            | ColumnDataType::DTLongText
            | ColumnDataType::DTMediumText
            | ColumnDataType::DTTinyText
            | ColumnDataType::DTJson => {
                // String types cannot be unsigned
                if self.unsigned {
                    return false;
                }

                // Handle Length option
                if let ColumnOption::Length(length) = &self.option {
                    if *length <= 0 {
                        return false;
                    }

                    // If length > 255, cannot be indexed or unique
                    if *length > 255 && (self.index || self.unique) {
                        return false;
                    }
                } else {
                    // If no Length and it's indexed or unique, invalid
                    if self.index || self.unique {
                        return false;
                    }
                }
            }

            ColumnDataType::DTBoolean => {
                if self.unique {
                    return false;
                }
            }
            _ => return true,
        }

        true
    }

    /// check is string type
    pub fn is_string_type(&self) -> bool {
        // match self.data_type {
        //     ColumnDataType::DTString
        //     | ColumnDataType::DTLongText
        //     | ColumnDataType::DTMediumText
        //     | ColumnDataType::DTTinyText
        //     | ColumnDataType::DTJson => true,
        //     _ => false,
        // }
        matches!(
            self.data_type,
            ColumnDataType::DTString
                | ColumnDataType::DTLongText
                | ColumnDataType::DTMediumText
                | ColumnDataType::DTTinyText
                | ColumnDataType::DTJson
        )
    }
}

impl Default for Column {
    /// make default column data
    fn default() -> Self {
        Column {
            name: String::new(),
            data_type: ColumnDataType::DTNone,
            nullable: false,
            comment: String::new(),
            unique: false,
            index: false,
            default: DefaultValue::None,
            unsigned: false,
            option: ColumnOption::None,
            change: false,
            collation: String::new(),
        }
    }
}

impl Default for ForeignKey {
    /// make default foreign data
    fn default() -> Self {
        ForeignKey {
            referenced_column: String::new(),
            column_name: String::new(),
            foreign_table: String::new(),
            on_delete: false,
            on_update: false,
        }
    }
}

impl ForeignKey {
    /// validate foreign key
    fn validate(&self) -> bool {
        if self.column_name.is_empty()
            || self.referenced_column.is_empty()
            || self.foreign_table.is_empty()
        {
            return false;
        }

        true
    }
}

impl<'a> ColumnBuilder<'a> {
    pub fn nullable(mut self) -> Self {
        self.column.nullable = true;
        self
    }

    pub fn unique(mut self) -> Self {
        self.column.unique = true;
        self
    }

    pub fn index(mut self) -> Self {
        self.column.index = true;
        self
    }

    pub fn unsigned(mut self) -> Self {
        self.column.unsigned = true;
        self
    }

    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.column.comment = comment.into();
        self
    }

    pub fn default_bool(mut self, value: bool) -> Self {
        self.column.default = DefaultValue::Bool(value);
        self
    }

    pub fn default_int(mut self, value: i64) -> Self {
        self.column.default = DefaultValue::Int(value);
        self
    }

    pub fn default_str(mut self, value: impl Into<String>) -> Self {
        self.column.default = DefaultValue::String(value.into());
        self
    }
    pub fn default_json_array(mut self) -> Self {
        self.column.default = DefaultValue::JsonArray;
        self
    }
    pub fn default_null(mut self) -> Self {
        self.column.default = DefaultValue::Null;
        self
    }

    pub fn default_current_timestamp(mut self) -> Self {
        self.column.default = DefaultValue::CurrenTimestamp;
        self
    }

    pub fn change(mut self) {
        self.column.change = true;
    }

    pub fn collation(mut self, collation: impl Into<String>) -> Self {
        // just work in mysql
        self.column.collation = collation.into();
        self
    }
}

impl<'a> ForeignKeyBuilder<'a> {
    pub fn reference(&mut self, referenced_column: impl Into<String>) -> &mut Self {
        self.key.referenced_column = referenced_column.into();
        self
    }

    pub fn on(&mut self, referenced_table_name: impl Into<String>) -> &mut Self {
        self.key.foreign_table = referenced_table_name.into();
        self
    }

    pub fn cascade_on_delete(&mut self) -> &mut Self {
        self.key.on_delete = true;
        self
    }

    pub fn cascade_on_update(&mut self) -> &mut Self {
        self.key.on_update = true;
        self
    }
}

impl<'a> Drop for ColumnBuilder<'a> {
    fn drop(&mut self) {
        self.table.columns.push(std::mem::take(&mut self.column));
    }
}
impl<'a> Drop for ForeignKeyBuilder<'a> {
    fn drop(&mut self) {
        self.table.foreign_keys.push(std::mem::take(&mut self.key));
    }
}
