//! This crates trait definitions

use ::types::SQLType;

/// Implement this trait to pass the struct along to helper functions to push into a table.
pub trait SerializeSQL {
    /// Get this struct's data to pass the data to SQL.
    fn to_sql(&self) -> Vec<SQLType>;
    /// Make a new struct out of the given data that has been guarenteed to be compatible. (At
    /// least the types)
    fn from_sql(Vec<SQLType>) -> Self;
    /// Make a new struct out of the given data that hasn't been transformed into SQLType.
    fn from_sql_str(Vec<String>) -> Result<Self, String>;
    /// Get the sql reprensation of the struct for when creating a new table.
    /// SQLTypes may as well be blank, as the enum is only used as an identifier for this.
    fn new_sql_repr() -> Vec<SQLType>;
}

#[macro_export]
//TODO: Implement this macro.
macro_rules! impl_sql_serialize {
    ($name:ident; $($field:ident),+) => (
            impl $crate::SerializeSQL for $name {
                fn to_sql(&self) -> Vec<SQLType> {
                    vec![]
                }
                #[allow(unused_variables)]
                fn from_sql(list: Vec<SQLType>) -> Self {
                    Self::new()
                }
                #[allow(unused_variables)]
                fn from_sql_str(list: Vec<String>) -> Result<Self, String> {
                    Err("Not yet implemented!".to_string())
                }
                fn new_sql_repr() -> Vec<SQLType> {
                    vec![]
                }
            }
        );
}


struct TestStruct {
    name: String,
    id: i32,
    flag: i8
}
impl_sql_serialize!(TestStruct; name, id, flag);
impl TestStruct {
    pub fn new() -> Self {
        TestStruct {
            name: "".to_string(),
            id: 0,
            flag: 0,
        }
    }
}

