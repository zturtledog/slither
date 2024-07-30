use crate::{manager, utils::{client::{self, Clients}, plugin::Plugin}};

use std::sync::{Arc, RwLock};
use xcb::{grab_server, query_tree, ungrab_server, ConfigureRequestEvent, MapRequestEvent};
use xcb_util::ewmh::Connection;

pub struct CorePlugin {
    //nothing yet
}

impl Plugin for CorePlugin {
    fn init(&self, conn: Arc<Connection>, clients: Arc<RwLock<Clients>>) {
        grab_server(&conn);

        let screen = manager::get_screen(&conn);
        let query: xcb::Reply<xcb::ffi::xcb_query_tree_reply_t> = query_tree(&conn, screen.root()).get_reply().expect("could not query tree");

        for tlw in query.children() {
            client::frame(&conn, &clients, *tlw, true)
        }

        ungrab_server(&conn);
    }

    fn on_configure_request(&self, conn: Arc<Connection>, clients: Arc<RwLock<Clients>>, event: &ConfigureRequestEvent) {
        // changes.x = e.x;
        // changes.y = e.y;
        // changes.width = e.width;
        // changes.height = e.height;
        // changes.border_width = e.border_width;
        // changes.sibling = e.above;
        // changes.stack_mode = e.detail;
        // // Grant request by calling XConfigureWindow().
        // XConfigureWindow(display_, e.window, e.value_mask, &changes);

        let clientx = clients.read().expect("could not aquire a read handle for clients");

        let mut values = vec![];

        values.push((xcb::CONFIG_WINDOW_X as u16, event.x() as u32));
        values.push((xcb::CONFIG_WINDOW_Y as u16, event.y() as u32));
        values.push((xcb::CONFIG_WINDOW_WIDTH as u16, event.width() as u32));
        values.push((xcb::CONFIG_WINDOW_HEIGHT as u16, event.width() as u32));
        values.push((xcb::CONFIG_WINDOW_BORDER_WIDTH as u16, event.border_width() as u32));
        values.push((xcb::CONFIG_WINDOW_SIBLING as u16, event.sibling() as u32));
        values.push((xcb::CONFIG_WINDOW_STACK_MODE as u16, event.stack_mode() as u32));

        if let Some(frm) = clientx.clients.get(&event.window()) {
            xcb::configure_window(&conn, frm.frame, &values);
        }

        xcb::configure_window(&conn, event.window(), &values);

        conn.flush();
    }

    fn on_map_request(&self, conn: Arc<Connection>, clients: Arc<RwLock<Clients>>, event: &MapRequestEvent) {
        client::frame(&conn, &clients, event.window(), false);
        xcb::map_window(&conn, event.window());
    }

    fn on_unmap_notify(&self, conn: Arc<xcb_util::ewmh::Connection>, clients: Arc<RwLock<Clients>>, event: &xcb::UnmapNotifyEvent) {
        let clientx = clients.read().expect("could not aquire a read handle for clients");
        if clientx.clients.contains_key(&event.window()) {
            return
        }
        let screen = manager::get_screen(&conn);
        if event.event() == screen.root() {
            return
        }
        client::unframe(&conn, &clients, event.window());
    }
}
