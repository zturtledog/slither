use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use xcb::{
    create_window, destroy_window, get_geometry, get_window_attributes, map_window,
    reparent_window, unmap_window, Window,
};
use xcb_util::ewmh::Connection;

use crate::manager;

#[derive(Clone, Eq, PartialEq)]
pub struct Client {
    pub frame: xcb::Window,
    pub window: xcb::Window,
    pub workspace: Option<u8>,
    // pub visible: bool,
    // pub full_screen: bool,
}

pub struct Clients {
    pub conn: Arc<xcb_util::ewmh::Connection>,
    pub clients: HashMap<Window, Client>,
    pub active_workspace: u8,
}

impl Clients {
    pub fn new(conn: Arc<xcb_util::ewmh::Connection>) -> Self {
        Self {
            conn,
            clients: HashMap::new(),
            active_workspace: 1,
        }
    }

    pub fn _refresh(&mut self) {
        xcb_util::ewmh::set_client_list(
            &self.conn,
            0,
            &self
                .clients
                .iter()
                .map(|c| c.0.clone())
                .collect::<Vec<u32>>(),
        );
    }
}

pub fn frame(
    conn: &Arc<Connection>,
    clients: &Arc<RwLock<Clients>>,
    window: Window,
    pre_start: bool,
) {
    //obtain lock
    let mut clientx = clients.write().expect("could not ");

    //get win data
    let attr = get_window_attributes(&conn, window)
        .get_reply()
        .expect("could not get window attributes");
    let geometry = get_geometry(&conn, window)
        .get_reply()
        .expect("could not get window geometry");

    // pre existing windows
    if pre_start
        && (attr.override_redirect() || attr.map_state() != xcb::xproto::MAP_STATE_VIEWABLE as u8)
    {
        return;
    }

    //create frame

    let frame: Window = conn.generate_id();
    let screen = manager::get_screen(&conn);

    create_window(
        &conn,
        xcb::WINDOW_CLASS_COPY_FROM_PARENT as u8,
        frame,
        screen.root(),
        geometry.x(),
        geometry.y(),
        geometry.width(),
        geometry.height(),
        3,
        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
        screen.root_visual(),
        &[
            (xcb::xproto::CW_BORDER_PIXEL, 0xff0000),
            (xcb::xproto::CW_BACKING_PIXEL, 0x0000ff),
        ],
    );
    //todo event selection?

    //attach frame

    //todo save set

    reparent_window(&conn, window, frame, 0, 0);
    map_window(conn, frame);

    clientx.clients.insert(
        window,
        Client {
            frame,
            window,
            workspace: None,
        },
    );

    //todo grabs
}

pub fn unframe(conn: &Arc<Connection>, clients: &Arc<RwLock<Clients>>, window: Window) {
    let mut clientx = clients.write().expect("could not ");
    let screen = manager::get_screen(&conn);

    let cli = match clientx.clients.get(&window) {
        Some(x) => x,
        None => return,
    };

    unmap_window(&conn, cli.frame);

    reparent_window(&conn, window, screen.root(), 0, 0);

    //todo remove save set

    destroy_window(&conn, cli.frame);

    clientx.clients.remove(&window);
}
