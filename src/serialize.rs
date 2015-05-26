//! This crates trait definitions

use ::types::SQLType;
use ::std::str::FromStr;

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
    fn new_sql_repr() -> Vec<(&'static str, SQLType)>;
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
                fn new_sql_repr() -> Vec<(&'static str, SQLType)> {
                    vec![]
                }
            }
        );
}


pub struct TestStruct {
    name: String,
    id: i32,
    flag: i8
}
//impl_sql_serialize!(TestStruct; name, id, flag);
impl SerializeSQL for TestStruct {
    fn to_sql(&self) -> Vec<SQLType> {
        vec![
            SQLType::VarChar("'".to_string() + &self.name + "'", 60),
            SQLType::Int(self.id),
            SQLType::Tiny(self.flag),
        ]
    }
    fn from_sql(from: Vec<SQLType>) -> Self {
        TestStruct {
            name: from[0].get_string().unwrap(),
            id:   from[1].get_i32().unwrap(),
            flag: from[2].get_i8().unwrap(),
        }
    }
    fn from_sql_str(from: Vec<String>) -> Result<Self, String> {
        Ok(TestStruct {
            name: from[0].clone(),
            id: i32::from_str(&from[1]).unwrap(),
            flag: i8::from_str(&from[2]).unwrap(),
        })
    }
    fn new_sql_repr() -> Vec<(&'static str, SQLType)> {
        vec![
            ("name", SQLType::VarChar("".to_string(), 60)),
            ("id",   SQLType::Int(0)),
            ("flag", SQLType::Tiny(0)),
        ]
    }
}
impl TestStruct {
    pub fn new() -> Self {
        TestStruct {
            name: "Example".to_string(),
            id: 0,
            flag: 0,
        }
    }
}

