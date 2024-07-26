use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use xcb::{
    x::{self, DestroyNotifyEvent, MapRequestEvent},
    Connection, Event,
};

use xcb_util::ffi::ewmh;

use crate::wrapped::Connect;

pub struct WinManager {
    conn: Arc<Connect>,
}

impl WinManager {
    pub fn new() -> Self {
        Self {
            conn: connect(None),
        }
    }
}