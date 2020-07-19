//
// Rust Firebird Client
//
// Connection functions
//

use std::cell::Cell;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;
use std::os::raw::c_short;
use std::os::raw::c_void;
use std::ptr;
use std::result::Result;

use super::error::FbError;
use super::ibase;
use super::transaction::Transaction;

pub struct Connection {
    pub handle: Cell<ibase::isc_db_handle>,
}

impl Connection {
    /// Open a new connection to the remote database
    pub fn open(
        host: String,
        port: u32,
        db_name: String,
        user: String,
        pass: String,
    ) -> Result<Connection, FbError> {
        let handle = Cell::new(0 as u32);

        unsafe {
            let mem_alloc = 1 + user.len() + 2 + pass.len() + 2;
            let mut dpb: *mut c_char = libc::malloc(mem_alloc) as *mut c_char;
            let mut dpb_len = 1 as c_short;
            let dpb_len_p = &mut dpb_len as *mut c_short;

            *dpb = ibase::isc_dpb_version1 as c_char;

            let c_user = match CString::new(user.clone()) {
                Ok(c) => c.into_raw(),
                Err(e) => {
                    return Err(FbError {
                        code: -1,
                        msg: e.to_string(),
                    })
                }
            };
            ibase::isc_modify_dpb(
                &mut dpb,
                dpb_len_p,
                ibase::isc_dpb_user_name,
                c_user,
                user.len() as c_short,
            );

            let c_pass = match CString::new(pass.clone()) {
                Ok(c) => c.into_raw(),
                Err(e) => {
                    return Err(FbError {
                        code: -1,
                        msg: e.to_string(),
                    })
                }
            };
            ibase::isc_modify_dpb(
                &mut dpb,
                dpb_len_p,
                ibase::isc_dpb_password,
                c_pass,
                pass.len() as c_short,
            );

            let host_db = format!("{}/{}:{}", host, port, db_name);
            let c_host_db = match CString::new(host_db.clone()) {
                Ok(c) => c.into_raw(),
                Err(e) => {
                    return Err(FbError {
                        code: -1,
                        msg: e.to_string(),
                    })
                }
            };

            let status: *mut ibase::ISC_STATUS_ARRAY =
                libc::malloc(mem::size_of::<ibase::ISC_STATUS_ARRAY>())
                    as *mut ibase::ISC_STATUS_ARRAY;
            let handle_ptr = handle.as_ptr();
            if ibase::isc_attach_database(status, 0, c_host_db, handle_ptr, dpb_len, dpb) != 0 {
                return Err(FbError::from_status(status));
            }

            libc::free(dpb as *mut c_void);
            libc::free(status as *mut c_void);
        }

        Ok(Connection { handle: handle })
    }

    /// Open a new connection to the local database
    pub fn open_local(db_name: String) -> Result<Connection, FbError> {
        let handle = Cell::new(0 as u32);

        unsafe {
            let c_db_name = match CString::new(db_name.clone()) {
                Ok(c) => c.into_raw(),
                Err(e) => {
                    return Err(FbError {
                        code: -1,
                        msg: e.to_string(),
                    })
                }
            };

            let status: *mut ibase::ISC_STATUS_ARRAY =
                libc::malloc(mem::size_of::<ibase::ISC_STATUS_ARRAY>())
                    as *mut ibase::ISC_STATUS_ARRAY;
            let handle_ptr = handle.as_ptr();
            if ibase::isc_attach_database(status, 0, c_db_name, handle_ptr, 0, ptr::null()) != 0 {
                return Err(FbError::from_status(status));
            }

            libc::free(status as *mut c_void);
        }

        Ok(Connection { handle: handle })
    }

    /// Create a new local database
    pub fn create_local(db_name: String) -> Result<(), FbError> {
        let local = Connection {
            handle: Cell::new(0 as u32),
        };

        let local_tr = Transaction {
            handle: Cell::new(0 as u32),
            conn: &local,
        };

        let sql = format!("create database \"{}\"", db_name);

        if let Err(e) = local_tr.execute_immediate(sql) {
            return Err(e);
        }

        local.close()
    }

    /// Drop the current database
    pub fn drop_database(self) -> Result<(), FbError> {
        unsafe {
            let status: *mut ibase::ISC_STATUS_ARRAY =
                libc::malloc(mem::size_of::<ibase::ISC_STATUS_ARRAY>())
                    as *mut ibase::ISC_STATUS_ARRAY;

            let handle_ptr = self.handle.as_ptr();
            if ibase::isc_drop_database(status, handle_ptr) != 0 {
                return Err(FbError::from_status(status));
            }

            libc::free(status as *mut c_void);
        }

        Ok(())
    }

    // Drop the database, if exists, and create a new empty
    pub fn recreate_local(db_name: String) -> Result<(), FbError> {
        if let Ok(conn) = Self::open_local(db_name.clone()) {
            if let Err(e) = conn.drop_database() {
                return Err(e);
            }
        }

        Self::create_local(db_name)
    }

    /// Close the current connection
    pub fn close(self) -> Result<(), FbError> {
        unsafe {
            let status: *mut ibase::ISC_STATUS_ARRAY =
                libc::malloc(mem::size_of::<ibase::ISC_STATUS_ARRAY>())
                    as *mut ibase::ISC_STATUS_ARRAY;

            let handle_ptr = self.handle.as_ptr();
            if ibase::isc_detach_database(status, handle_ptr) != 0 {
                return Err(FbError::from_status(status));
            }

            libc::free(status as *mut c_void);
        }

        Ok(())
    }

    pub fn start_transaction(&self) -> Result<Transaction, FbError> {
        Transaction::start_transaction(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn local_connection() {
        Connection::recreate_local("test.fdb".to_string())
            .expect("Error on recreate the test database");

        let conn = Connection::open_local("test.fdb".to_string())
            .expect("Error on connect the test database");

        conn.close().expect("error on close the connection");
    }
}