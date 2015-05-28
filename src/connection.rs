//! A wrapper for the MariaDB / MySQL connection.

#![allow(dead_code)]

use ::ffi::mysql::{MYSQL, MYSQL_RES, MYSQL_ROW, mysql_init, mysql_real_connect, mysql_error, mysql_close, mysql_query, mysql_use_result, mysql_free_result, mysql_fetch_row};
use ::std::ptr;
use ::serialize::SerializeSQL;
use ::types::SQLType;
use ::cstr::{from_cstr, to_cstr};
use ::std::str::FromStr;

/// A connection to a MySQL server
pub struct Connection {
    /// The raw connection variable.
    conn: *mut MYSQL,
    /// The name of the database currently active.
    /// May be wrong, if you do not use switch_db() and instead do a raw_query()
    db: String
}

impl Connection {
    /// Attempts to connect to a server at the given address, with the given username, password,
    /// and database name.
    pub fn new(address: &str, user: &str, password: &str, database: &str) -> Result<Self, String> {
        let conn = unsafe { mysql_init(ptr::null_mut()) };
        if unsafe { mysql_real_connect(conn,
                                         to_cstr(address).as_ptr(),
                                         to_cstr(user).as_ptr(),
                                         to_cstr(password).as_ptr(),
                                         to_cstr(database).as_ptr(),
                                         0, ptr::null(), 0) }.is_null() {
            let err_msg = from_cstr(unsafe { mysql_error(conn) });
            unsafe { mysql_close(conn) };
            return Err(format!("Failed to connect to SQL. Reason: {} {}", err_msg, err_msg.len()))
        }
        Ok(Connection {
            conn: conn,
            db: database.to_string(),
        })
    }
    
    /// Attempt to switch the active db to the given name.
    /// Returns Ok if it worked.
    pub fn switch_db(&mut self, new_db: String) -> Result<(), String> {
        Ok(try!(self.raw_query_no_res(&format!("use {};", new_db))))
    }

    /// Attempt to get a list of all tables that exist on this database.
    pub fn get_tables_list(&self) -> Result<Vec<String>, String> {
        let result = try!(self.raw_query("show tables;", 1));

        //Simplify from the Vec<Vec<String>> to just Vec<String>
        Ok(result.into_iter().map(|e| e[0].clone()).collect())
    }
    
    /// Query for a list of all contents in a table with no delimiter.
    pub fn read_table_strings(&self, name: &str, width: isize) -> Result<Vec<Vec<String>>, String> {
        let query = format!("select * from {};", name);
        self.raw_query(&query, width)
    }

    /// Attempts to create a table from the currently active database.
    pub fn create_table(&self, table_name: &str, table_contents: &str) -> Result<(), String> {
        Ok(try!(self.raw_query_no_res(&format!("create table {} ({});", table_name, table_contents))))
    }

    /// Delete the given table from the currently active database.
    pub fn drop_table(&self, table_name: &str) -> Result<(), String> {
        Ok(try!(self.raw_query_no_res(&format!("drop table {};", table_name))))
    }
    
    /// Sends the given string as a query to the SQL server.
    /// Can use this directly, or any of the helper functions.  Up to you.
    pub fn raw_query(&self, query: &str, wide: isize) -> Result<Vec<Vec<String>>, String> {
        if wide < 1 {
            return Err(format!("Invalid width for query. Must be larger than zero. Given width was {}", wide));
        }
        if unsafe { mysql_query(self.conn, to_cstr(query).as_ptr()) } != 0 {
            return Err(from_cstr(unsafe { mysql_error(self.conn) }));
        }
        
        let mut vec = Vec::new();
        let result: *mut MYSQL_RES = unsafe { mysql_use_result(self.conn) };
        let mut row: MYSQL_ROW;

        loop {
            row = unsafe { mysql_fetch_row(result) };
            if row != ptr::null_mut() {
                let mut inner = Vec::new();
                for i in 0..wide {
                    inner.push(from_cstr(unsafe { *row.offset(i) }));
                }
                vec.push(inner);
            } else {
                break;
            }
        }
        
        //Free the result pointer.
        unsafe { mysql_free_result(result) };

        Ok(vec)
    }
    
    /// Sends the given string as a query to the SQL server.
    /// Does not even attempt to read a result.
    pub fn raw_query_no_res(&self, query: &str) -> Result<(), String> {
        if unsafe { mysql_query(self.conn, to_cstr(query).as_ptr()) } != 0 {
            let error = from_cstr(unsafe { mysql_error(self.conn) });
            Err(format!("Query of ({}) failed. Reason: {}", query, error))
        } else {
            Ok(())
        }
    }
    
    /// Insert an object into a table.
    pub fn insert_struct<T: SerializeSQL>(&self, table_name: &str, obj: &T) -> Result<(), String> {
        if try!(self.check_struct::<T>(table_name)) {
            let list = obj.to_sql();
            let mut ins = String::new();
            println!("{} -- {}", list[0].to_string().len(), list[0].to_string());
            for i in &list {
                ins = ins + &i.to_string() + ", ";
            }
            ins.pop(); ins.pop();

            try!(self.raw_query_no_res(&format!("insert into {} VALUES({});", table_name, ins)));
            
            return Ok(())
        } else {
            Err("Struct did not match what is in the table.".to_string())
        }
    }

    /// Checks if the struct and table match. 
    fn check_struct<T: SerializeSQL>(&self, table_name: &str) -> Result<bool, String> {
        let repr = T::new_sql_repr();
        let table_repr = try!(self.get_table_repr(table_name));
        if repr.len() != table_repr.len() {
            return Ok(false);
        }
        for i in 0..repr.len() {
            if repr[i].0 != &table_repr[i].0 {
                return Ok(false);
            }
            if repr[i].1.get_field_type() != table_repr[i].1.get_field_type() {
                return Ok(false);
            }
        }
        Ok(true)
    }
    fn get_table_repr(&self, table_name: &str) -> Result<Vec<(String, SQLType)>, String> {
        let list: Vec<(String, String)> = try!(self.raw_query(&format!("describe {};", table_name), 2))
            .into_iter().map(|e| (e[0].clone(), e[1].clone())).collect();
        let mut v = Vec::new();
        for i in &list {
            let temp = try!(SQLType::from_str(&i.1));
            v.push((i.0.clone(), temp));
        }
        Ok(v)
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe { mysql_close(self.conn) }
    }
}

#[test]
fn test_connection() {
    let server = "localhost";
    let user = "";
    let password = "";
    let database = "test";
    let conn = Connection::new(server, user, password, database).unwrap();
    conn.create_table("teststruct", "name VARCHAR(60), id INT, flag TINYINT").unwrap();
    conn.insert_struct("teststruct", &::serialize::TestStruct::new()).unwrap();
    conn.drop_table("teststruct").unwrap();
}

