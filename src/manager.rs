use std::sync::{Arc, Mutex};

use xcb::{ffi::xcb_generic_event_t, Connection, Event};

pub struct Manager {
    // clients: Arc<Mutex<Clients>>,
    conn: Arc<xcb_util::ewmh::Connection>,
    cursor: xcb::Cursor,
}

impl Manager {
    pub fn new() -> Self {
        //establish connection
        let (conn, _) = xcb::Connection::connect(None)
            .expect("Unable to access your display. Check your DISPLAY environment variable.");

        let conn = xcb_util::ewmh::Connection::connect(conn)
            .map_err(|(e, _)| e)
            .expect("Unable to create EWMH connection.");

        let conn = Arc::new(conn);
        
        //create clients
        // let clients = Arc::new(Mutex::new(Clients::new(conn.clone())));

        //ensure cursor is not default
        let cursor = xcb_util::cursor::create_font_cursor(&conn, xcb_util::cursor::LEFT_PTR);

        Self {
            // clients,
            conn,
            cursor,
        }
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
        xcb_util::ewmh::set_wm_name(&self.conn, window, "slither");

        //todo: keys

        xcb_util::ewmh::set_number_of_desktops(&self.conn, 0, 1);
        xcb_util::ewmh::set_current_desktop(&self.conn, 0, 1);

        let values = [(
            xcb::CW_EVENT_MASK,
            xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT | xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY,
        )];

        let cookie = xcb::change_window_attributes_checked(&self.conn, screen.root(), &values);

        if cookie.request_check().is_err() {
            panic!("Unable to change window attributes. Is another window manager running?")
        }

        //todo: astos

        let values = [(xcb::CW_CURSOR, self.cursor)];

        let cookie = xcb::change_window_attributes_checked(&self.conn, screen.root(), &values);

        if cookie.request_check().is_err() {
            panic!("Unable to set cursor icon.")
        }

        loop {
            if let Some(event) = self.conn.wait_for_event() {
                // let clients = self.clients.clone();
                let conn = self.conn.clone();

                tokio::spawn(Self::handle(
                    // clients, 
                    conn, event));
            }
        }
    }

    pub async fn handle(conn : Arc<xcb_util::ewmh::Connection>, event : Event<xcb_generic_event_t>) {
        //
    }
}

pub fn get_screen(conn: &xcb_util::ewmh::Connection) -> xcb::Screen {
    conn.get_setup()
        .roots()
        .next()
        .expect("Unable to find a screen.")
}