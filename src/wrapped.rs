use std::sync::Arc;
use std::mem;
use std::ptr;

use xcb::Connection;

use xcb_util::ffi::ewmh;
use xcb_util::ffi::ewmh::xcb_ewmh_connection_t;
use xcb::ffi::base::xcb_generic_error_t;

pub struct Connect {
    pub actual: Connection,
    ewmh: xcb_ewmh_connection_t,
}
unsafe impl Send for Connect {}
unsafe impl Sync for Connect {}
impl Connect {
    pub fn new(actual: Connection, ewmh: xcb_ewmh_connection_t) -> Self {Self {actual,ewmh}}
}

pub fn connect(display_name: Option<&str>) -> Arc<Connect> {
    let (conn, _) = xcb::Connection::connect(None)
        .expect("Unable to access your display. Check your DISPLAY environment variable.");

    let ret: (Connection, xcb_ewmh_connection_t) = unsafe {
        let mut ewmh: xcb_ewmh_connection_t = mem::zeroed();
        let mut err: *mut xcb_generic_error_t = ptr::null_mut();

        let cookie = ewmh::xcb_ewmh_init_atoms(conn.get_raw_conn(), &mut ewmh);
        ewmh::xcb_ewmh_init_atoms_replies(&mut ewmh, cookie, &mut err);

        if err.is_null() {
            Ok((conn, ewmh))
        } else {
            Err(())
        }
    }
    .expect("Unable to create EWMH connection.");
    Arc::new(Connect::new(ret.0, ret.1))
}
