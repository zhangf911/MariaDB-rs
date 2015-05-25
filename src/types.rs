//! Enums for the possible types for SQL.
//! Might remove in the future. (Not sure if this is important or not yet)

use ::ffi::mysql::*;

/// An enum of the possible field types when working with SQL.
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SQLFieldType {
    Decimal    = MYSQL_TYPE_DECIMAL,
    Tiny       = MYSQL_TYPE_TINY,
    Short      = MYSQL_TYPE_SHORT,
    Long       = MYSQL_TYPE_LONG,
    Float      = MYSQL_TYPE_FLOAT,
    Double     = MYSQL_TYPE_DOUBLE,
    NULL       = MYSQL_TYPE_NULL,
    Timestamp  = MYSQL_TYPE_TIMESTAMP,
    LongLong   = MYSQL_TYPE_LONGLONG,
    Int24      = MYSQL_TYPE_INT24,
    Date       = MYSQL_TYPE_DATE,
    Time       = MYSQL_TYPE_TIME,
    DateTime   = MYSQL_TYPE_DATETIME,
    Year       = MYSQL_TYPE_YEAR,
    NewDate    = MYSQL_TYPE_NEWDATE,
    VarChar    = MYSQL_TYPE_VARCHAR,
    Bit        = MYSQL_TYPE_BIT,
    TimeStamp2 = MYSQL_TYPE_TIMESTAMP2,
    DateTime2  = MYSQL_TYPE_DATETIME2,
    Time2      = MYSQL_TYPE_TIME2,
    NewDecimal = MYSQL_TYPE_NEWDECIMAL,
    Enum       = MYSQL_TYPE_ENUM,
    Set        = MYSQL_TYPE_SET,
    TinyBlob   = MYSQL_TYPE_TINY_BLOB,
    MediumBlob = MYSQL_TYPE_MEDIUM_BLOB,
    LongBlob   = MYSQL_TYPE_LONG_BLOB,
    Blob       = MYSQL_TYPE_BLOB,
    VarString  = MYSQL_TYPE_VAR_STRING,
    String     = MYSQL_TYPE_STRING,
    Geometry   = MYSQL_TYPE_GEOMETRY
}

/// An enum for wrapping the currently supported types.
pub enum SQLType {
    /// Tiny, aka i8
    Tiny(i8),
    /// Short, aka i16
    Short(i16),
    /// Int, aka i32
    Int(i32),
    /// Long, aka i64
    Long(i64),
    /// Float, aka f32
    Float(f32),
    /// Double, aka f64
    Double(f64),
    /// VarChar, aka String (However, this also has a max length limit attached)
    VarChar(String, usize),
    /// Unsupported type,
    /// 
    /// String for its string representation, for passing to SQL.
    /// String for its string name when creating a new table.
    /// SQLFieldType for what type it is.
    Unsupported(String, String, SQLFieldType)
}

impl SQLType {
    pub fn to_string(&self) -> String {
        match *self {
            SQLType::Tiny(e)                  => e.to_string(),
            SQLType::Short(e)                 => e.to_string(),
            SQLType::Int(e)                   => e.to_string(),
            SQLType::Long(e)                  => e.to_string(),
            SQLType::Float(e)                 => e.to_string(),
            SQLType::Double(e)                => e.to_string(),
            SQLType::VarChar(ref e, _)        => e.clone(),
            SQLType::Unsupported(ref e, _, _) => e.clone(),
        }
    }
    pub fn get_name_of_enum(&self) -> String {
        match *self {
            SQLType::Tiny(_)                  => "Tiny".to_string(),
            SQLType::Short(_)                 => "Short".to_string(),
            SQLType::Int(_)                   => "Int".to_string(),
            SQLType::Long(_)                  => "Long".to_string(),
            SQLType::Float(_)                 => "Float".to_string(),
            SQLType::Double(_)                => "Double".to_string(),
            SQLType::VarChar(_, size)         => "VARCHAR(".to_string() + &size.to_string() + ")",
            SQLType::Unsupported(_, ref e, _) => e.clone()
        }
    }
    pub fn get_field_type(&self) -> SQLFieldType {
        match *self {
            SQLType::Tiny(_)                  => SQLFieldType::Tiny,
            SQLType::Short(_)                 => SQLFieldType::Short,
            SQLType::Int(_)                   => SQLFieldType::Long,    
            SQLType::Long(_)                  => SQLFieldType::LongLong,
            SQLType::Float(_)                 => SQLFieldType::Float,
            SQLType::Double(_)                => SQLFieldType::Double,
            SQLType::VarChar(_, _)            => SQLFieldType::VarChar,
            SQLType::Unsupported(_, _, e)     => e
        }
    }
    pub fn is_tiny(&self) -> bool {
        match *self {
            SQLType::Tiny(_) => true,
            _                => false
        }
    }
    pub fn is_short(&self) -> bool {
        match *self {
            SQLType::Short(_) => true,
            _                 => false
        }
    }
    pub fn is_int(&self) -> bool {
        match *self {
            SQLType::Int(_) => true,
            _               => false
        }
    }
    pub fn is_long(&self) -> bool {
        match *self {
            SQLType::Long(_) => true,
            _                => false
        }
    }
    pub fn is_float(&self) -> bool {
        match *self {
            SQLType::Float(_) => true,
            _                 => false
        }
    }
    pub fn is_double(&self) -> bool {
        match *self {
            SQLType::Double(_) => true,
            _                  => false
        }
    }
    pub fn is_varchar(&self) -> bool {
        match *self {
            SQLType::VarChar(_, _) => true,
            _                      => false
        }
    }
    pub fn is_unsupported(&self) -> bool {
        match *self {
            SQLType::Unsupported(_, _, _) => true,
            _                             => false
        }
    }
}
