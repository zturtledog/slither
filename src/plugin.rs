use std::sync::Arc;

use xcb::x::{MapRequestEvent,DestroyNotifyEvent};

pub trait EventPlugin {
    fn map_request(conn: &Arc<xcb_util::ewmh::Connection>, req: MapRequestEvent);
    fn destroy_notify(conn: &Arc<xcb_util::ewmh::Connection>, req: DestroyNotifyEvent);
}