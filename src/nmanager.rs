// COPYRIGHT NOTICE:
//  some setup/boilerplate code was taken from https://github.com/monroeclinton/mwm/



use std::sync::Arc;

// use xcb::Connection;

use crate::plugin::Plugin;

// use xcb_util::ffi::ewmh;

// use crate::wrapped::Connect;

pub struct WinManager {
    plugins: Arc<Vec<Box<dyn Plugin>>>,
    conn: Arc<xcb_util::ewmh::Connection>,
    cursor: xcb::Cursor,
}

impl WinManager {
    pub fn new() -> Self {
        let conn = connect(None);
        let cursor = xcb_util::cursor::create_font_cursor(&conn, xcb_util::cursor::LEFT_PTR);

        Self { cursor, conn, plugins: Arc::new(vec![]) }
    }

    pub fn plugin(mut self, plugin : impl Plugin) -> Self {
        self.plugins.push(Box::new(plugin));
        self
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
        xcb_util::ewmh::set_wm_name(&self.conn, window, "mwm");

        // xcb_util::ewmh::set_number_of_desktops(&self.conn, 0, 9);
        // xcb_util::ewmh::set_current_desktop(&self.conn, 0, 1);


        if xcb::change_window_attributes_checked(&self.conn, screen.root(), &[(
            xcb::CW_EVENT_MASK,
            xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT | xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY,
        )]).request_check().is_err() {
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

        self.conn.flush(); // just to be sure; shouldn't be nescesary but still

        loop {
            if let Some(event) = self.conn.wait_for_event() {
                let conn = self.conn.clone();
                let plugins = self.plugins.clone();

                tokio::spawn(Self::handle(conn, plugins, event));
            }
        }
    }

    async fn handle(
        conn: Arc<xcb_util::ewmh::Connection>,
        plugins: Arc<Vec<Box<dyn Plugin>>>,
        event: xcb::GenericEvent,
    ) {
        let mut handler = Handler::default();
    
        let response_type = event.response_type() & !0x80;
    
        match response_type {
    //         xcb::CLIENT_MESSAGE => handler.on_client_message(EventContext {
    //             clients,
    //             config,
    //             conn: conn.clone(),
    //             event: unsafe {
    //                 std::mem::transmute::<xcb::GenericEvent, xcb::ClientMessageEvent>(event)
    //             },
    //         }),
    //         xcb::KEY_PRESS => handler.on_key_press(EventContext {
    //             clients,
    //             config,
    //             conn: conn.clone(),
    //             event: unsafe {
    //                 std::mem::transmute::<xcb::GenericEvent, xcb::KeyPressEvent>(event)
    //             },
    //         }),
    //         xcb::CONFIGURE_REQUEST => handler.on_configure_request(EventContext {
    //             clients,
    //             config,
    //             conn: conn.clone(),
    //             event: unsafe {
    //                 std::mem::transmute::<xcb::GenericEvent, xcb::ConfigureRequestEvent>(event)
    //             },
    //         }),
    //         xcb::MAP_REQUEST => handler.on_map_request(EventContext {
    //             clients,
    //             config,
    //             conn: conn.clone(),
    //             event: unsafe {
    //                 std::mem::transmute::<xcb::GenericEvent, xcb::MapRequestEvent>(event)
    //             },
    //         }),
    //         xcb::PROPERTY_NOTIFY => handler.on_property_notify(EventContext {
    //             clients,
    //             config,
    //             conn: conn.clone(),
    //             event: unsafe {
    //                 std::mem::transmute::<xcb::GenericEvent, xcb::PropertyNotifyEvent>(event)
    //             },
    //         }),
    //         xcb::ENTER_NOTIFY => handler.on_enter_notify(EventContext {
    //             clients,
    //             config,
    //             conn: conn.clone(),
    //             event: unsafe {
    //                 std::mem::transmute::<xcb::GenericEvent, xcb::EnterNotifyEvent>(event)
    //             },
    //         }),
    //         xcb::UNMAP_NOTIFY => handler.on_unmap_notify(EventContext {
    //             clients,
    //             config,
    //             conn: conn.clone(),
    //             event: unsafe {
    //                 std::mem::transmute::<xcb::GenericEvent, xcb::UnmapNotifyEvent>(event)
    //             },
    //         }),
    //         xcb::DESTROY_NOTIFY => handler.on_destroy_notify(EventContext {
    //             clients,
    //             config,
    //             conn: conn.clone(),
    //             event: unsafe {
    //                 std::mem::transmute::<xcb::GenericEvent, xcb::DestroyNotifyEvent>(event)
    //             },
    //         }),
            // Events we do not care about
            _ => (),
        }
        conn.flush();
    }
}

fn connect(displayname: Option<&str>) -> Arc<xcb_util::ewmh::Connection> {
    Arc::new(
        xcb_util::ewmh::Connection::connect(
            xcb::Connection::connect(None)
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
