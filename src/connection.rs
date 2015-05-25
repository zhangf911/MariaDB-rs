//! A wrapper for the MariaDB / MySQL connection.

#![allow(dead_code)]

use ::ffi::mysql::{MYSQL, MYSQL_RES, MYSQL_ROW, mysql_init, mysql_real_connect, mysql_error, mysql_close, mysql_query, mysql_use_result, mysql_free_result, mysql_fetch_row};
use ::std::ptr;
use ::cbox::{CBox, from_cstr, to_cstr};

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
        if ! unsafe { mysql_real_connect(conn,
                                         to_cstr(address).get_raw(),
                                         to_cstr(user).get_raw(),
                                         to_cstr(password).get_raw(),
                                         to_cstr(database).get_raw(),
                                         0, ptr::null(), 0) }.is_null() {
            let err_msg = from_cstr(&CBox::from_raw(unsafe { mysql_error(conn) }));
            unsafe { mysql_close(conn) };
            return Err(format!("Failed to connect to SQL. Reason: {}", err_msg))
        }
        Ok(Connection {
            conn: conn,
            db: database.to_string(),
        })
    }
    
    /// Attempt to switch the active db to the given name.
    /// Returns Ok if it worked.
    pub fn switch_db(&mut self, new_db: String) -> Result<(), String> {
        match self.raw_query_no_res(&("use".to_string() + &new_db + ";")) {
            Ok(o)  => Ok(o),
            Err(_) => Err(format!("Switch to database of the name \"{}\" failed", new_db))
        }
    }

    /// Attempt to get a list of all tables that exist on this database.
    pub fn get_tables_list(&self) -> Result<Vec<String>, String> {
        let result = match self.raw_query("show tables;", 1) {
            Ok(r) => r,
            Err(err_msg) => return Err(err_msg),
        };

        //Simplify from the Vec<Vec<String>> to just Vec<String>
        Ok(result.into_iter().map(|e| e[0].clone()).collect())
    }
    
    #[allow(unused_variables)]
    /// Query for a list of all contents in a table with no delimiter.
    pub fn read_table_strings(&self, name: &str, width: isize) -> Result<Vec<Vec<String>>, String> {
        let query = "select * from ".to_string() + name + ";";
        self.raw_query(&query, width)
    }

    /// Attempts to create a table
    pub fn create_table(&self, table_name: &str, table_contents: &str) -> Result<(), String> {
        match self.raw_query_no_res(&("create table ".to_string() + table_name + " " + table_contents + ";")) {
            Ok(o)  => Ok(o),
            Err(_) => Err(format!("Create table of the name \"{}\" and contents \"{}\" failed.", table_name, table_contents))
        }
    }
    
    /// Sends the given string as a query to the SQL server.
    /// Not recommened to use directly, use other functions if possible.
    /// 
    /// Public only in case a feature is missing, might convert to private later.
    pub fn raw_query(&self, query: &str, wide: isize) -> Result<Vec<Vec<String>>, String> {
        if wide < 1 {
            return Err(format!("Invalid width for query. Must be larger than zero. Given width was {}", wide));
        }
        if unsafe { mysql_query(self.conn, to_cstr(query).get_raw()) } != 0 {
            return Err(format!("Query of ({}) failed.", query));
        }
        
        let mut vec = Vec::new();
        let result: *mut MYSQL_RES = unsafe { mysql_use_result(self.conn) };
        let mut row: MYSQL_ROW;

        loop {
            row = unsafe { mysql_fetch_row(result) };
            if row != ptr::null_mut() {
                let mut inner = Vec::new();
                for i in 0..wide {
                    inner.push(from_cstr(&CBox::from_raw_mut(unsafe { *row.offset(i) })));
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
        if unsafe { mysql_query(self.conn, to_cstr(query).get_raw()) } != 0 {
            Err(format!("Query of ({}) failed.", query))
        } else {
            Ok(())
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe { mysql_close(self.conn) }
    }
}

