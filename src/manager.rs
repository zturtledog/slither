// COPYRIGHT NOTICE:
//  some setup/boilerplate code was taken from https://github.com/monroeclinton/mwm/

use std::sync::{Arc, RwLock};

// use xcb::Connection;

use crate::utils::{client::Clients, macros, plugin::{Plugin, StructuredPlugin}};

// use xcb_util::ffi::ewmh;

// use crate::wrapped::Connect;

pub struct WinManager {
    plugins: Arc<RwLock<Vec<StructuredPlugin>>>,
    conn: Arc<xcb_util::ewmh::Connection>,
    clients: Arc<RwLock<Clients>>,
    cursor: xcb::Cursor,
}

impl WinManager {
    pub fn new() -> Self {
        let conn = connect(None);
        let cursor = xcb_util::cursor::create_font_cursor(&conn, xcb_util::cursor::LEFT_PTR);
        let clients = Arc::new(RwLock::new(Clients::new(conn.clone())));

        Self {
            cursor,
            conn,
            clients,
            plugins: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn plugin(&mut self, plugin: impl Plugin + 'static) {
        let mut pluginx = self.plugins.write().unwrap();
        pluginx.push(StructuredPlugin::new(plugin));
    }

    pub fn start(self) {
        let screen = get_screen(&self.conn);

        xcb_util::ewmh::set_supported(
            &self.conn,
            0,
            &[
                self.conn.SUPPORTED(),
                self.conn.SUPPORTING_WM_CHECK(),
                // self.conn.ACTIVE_WINDOW(),
                // self.conn.CLIENT_LIST(),
                // self.conn.CURRENT_DESKTOP(),
                // self.conn.DESKTOP_NAMES(),
                // self.conn.NUMBER_OF_DESKTOPS(),
                // self.conn.WM_STATE(),
                // self.conn.WM_STATE_FULLSCREEN(),
                // self.conn.WM_WINDOW_TYPE(),
                // self.conn.WM_WINDOW_TYPE_DIALOG(),
            ],
        );

        let window = self.conn.generate_id();

        xcb::create_window(
            &self.conn,
            xcb::WINDOW_CLASS_COPY_FROM_PARENT as u8,
            window,
            screen.root(),
            0,
            0,
            1,
            1,
            0,
            xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
            screen.root_visual(),
            &[],
        );

        xcb_util::ewmh::set_supporting_wm_check(&self.conn, screen.root(), window);
        xcb_util::ewmh::set_wm_name(&self.conn, window, "joid");

        // xcb_util::ewmh::set_number_of_desktops(&self.conn, 0, 9);
        // xcb_util::ewmh::set_current_desktop(&self.conn, 0, 1);

        if xcb::change_window_attributes_checked(
            &self.conn,
            screen.root(),
            &[(
                xcb::CW_EVENT_MASK,
                xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT | xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY,
            )],
        ).request_check().is_err(){
            panic!("Unable to change window attributes. Is another window manager running?")
        }

        // for program in &self.config.autostart {
        //     std::process::Command::new(program).spawn().unwrap();
        // }

        
        if xcb::change_window_attributes_checked(
            &self.conn,
            screen.root(),
            &[(xcb::CW_CURSOR, self.cursor)],
        ).request_check().is_err(){
            panic!("Unable to set cursor icon.")
        }
        
        /* i do not like rust multithreading */ {
            let pluginx = self.plugins.write().unwrap(); 
            for plugin in &*pluginx {
                plugin.init(self.conn.clone(),self.clients.clone())
            }
        }

        self.conn.flush(); // just to be sure; shouldn't be nescesary but still
        
        std::process::Command::new("xterminal-emulator").spawn().unwrap();

        loop {
            if let Some(event) = self.conn.wait_for_event() {
                let conn = self.conn.clone();
                let plugins = self.plugins.clone();
                let clients = self.clients.clone();

                tokio::spawn(Self::handle(conn, clients, plugins, event));
            }
        }
    }

    pub async fn handle(
        conn: Arc<xcb_util::ewmh::Connection>,
        clients: Arc<RwLock<Clients>>,
        plugins: Arc<RwLock<Vec<StructuredPlugin>>>,
        event: xcb::GenericEvent,
    ) {
        let response_type = event.response_type() & !0x80;
        let pluginx = plugins.read().unwrap();

        match response_type {
            xcb::CLIENT_MESSAGE => {macros::event!(on_client_message,conn,clients,pluginx,xcb::ClientMessageEvent,event);},
            xcb::KEY_PRESS => {macros::event!(on_key_press,conn,clients,pluginx,xcb::KeyPressEvent,event);},
            xcb::CONFIGURE_REQUEST => {macros::event!(on_configure_request,conn,clients,pluginx,xcb::ConfigureRequestEvent,event);},
            xcb::MAP_REQUEST => {macros::event!(on_map_request,conn,clients,pluginx,xcb::MapRequestEvent,event);},
            xcb::PROPERTY_NOTIFY => {macros::event!(on_property_notify,conn,clients,pluginx,xcb::PropertyNotifyEvent,event);},
            xcb::ENTER_NOTIFY => {macros::event!(on_enter_notify,conn,clients,pluginx,xcb::EnterNotifyEvent,event);},
            xcb::UNMAP_NOTIFY => {macros::event!(on_unmap_notify,conn,clients,pluginx,xcb::UnmapNotifyEvent,event);},
            xcb::DESTROY_NOTIFY => {macros::event!(on_destroy_notify,conn,clients,pluginx,xcb::DestroyNotifyEvent,event);},
            
            _ => {
                pluginx
                    .iter()
                    .for_each(|plugin| plugin.unspecified_event(conn.clone(), clients.clone(), response_type, &event));
            },
        }
        conn.flush();
    }
}

fn connect(displayname: Option<&str>) -> Arc<xcb_util::ewmh::Connection> {
    Arc::new(
        xcb_util::ewmh::Connection::connect(
            xcb::Connection::connect(displayname)
                .expect("Unable to access your display. Check your DISPLAY environment variable.")
                .0,
        )
        .map_err(|(e, _)| e)
        .expect("Unable to create EWMH connection."),
    )
}

pub fn get_screen(conn: &xcb_util::ewmh::Connection) -> xcb::Screen {
    conn.get_setup()
        .roots()
        .next()
        .expect("Unable to find a screen.")
}
