use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use xcb::{
    ffi::xcb_generic_event_t,
    x::{self, DestroyNotifyEvent, MapRequestEvent},
    Connection, Event,
};

use crate::plugin;

pub struct Manager {
    // clients: Arc<Mutex<Clients>>,
    conn: Arc<xcb_util::ewmh::Connection>,
    // cursor: xcb::Cursor,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            conn: connect(None),
        }
    }

    pub fn start(self) {
        let screen: xcb::Screen = get_screen(&self.conn);

        //todo desktop stuff

        //redirect events
        if xcb::change_window_attributes_checked(
            &self.conn,
            screen.root(),
            &[(
                xcb::CW_EVENT_MASK,
                xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT | xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY,
            )],
        )
        .request_check()
        .is_err()
        {
            panic!("Unable to change window attributes. Is another window manager running?")
        }

        self.conn.flush();

        //todo setup keybindings

        // event_loop
        loop {
            let event = match conn.wait_for_event() {
                Err(xcb::Error::Connection(err)) => {
                    panic!("unexpected I/O error: {}", err);
                }
                Err(xcb::Error::Protocol(xcb::ProtocolError::X(
                    x::Error::Font(err),
                    _req_name,
                ))) => {
                    // may be this particular error is fine?
                    continue;
                }
                Err(xcb::Error::Protocol(err)) => {
                    panic!("unexpected protocol error: {:#?}", err);
                }
                Ok(eve) => eve,
            };
            tokio::spawn(Self::handle(conn, event));
        }
    }

    async fn handle(conn: Arc<xcb_util::ewmh::Connection>, event: xcb::Event) {
        match event {
            Event::X(eve) => match eve {
                //map request
                // xcb::x::Event::MapRequest(map_requested_event) => {
                //     plugin::map_request(&conn, map_requested_event);
                //     conn.flush();
                // }
                // //destroy notify
                // xcb::x::Event::DestroyNotify(destroy_notify_event) => {
                //     plugin::destroy_notify(&conn, destroy_notify_event);
                //     conn.flush();
                // }
                _ => {println!(eve)}
            },
            _ => {}
        }
    }
}

impl EventPlugin for Manager {
    fn map_request(conn: &Arc<xcb_util::ewmh::Connection>, req: MapRequestEvent) {
        //
    }

    fn destroy_notify(conn: &Arc<xcb_util::ewmh::Connection>, req: DestroyNotifyEvent) {
        //
    }
}

pub fn get_screen(conn: &xcb_util::ewmh::Connection) -> xcb::Screen {
    conn.get_setup()
        .roots()
        .next()
        .expect("Unable to find a screen.")
}

pub fn connect(display_name: Option<&str>) -> Arc<xcb_util::ewmh::Connection> {
    Arc::new(
        xcb_util::ewmh::Connection::connect(
            xcb::Connection::connect(None)
                .expect("Unable to access your display. Check your DISPLAY environment variable."),
        )
        .map_err(|(e, _)| e)
        .expect("Unable to create EWMH connection."),
    )
}
