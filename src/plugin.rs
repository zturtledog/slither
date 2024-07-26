use xcb::x::MapRequestEvent;
use xcb::x::DestroyNotifyEvent;

trait EventPlugin {
    fn map_request(conn: &Arc<xcb_util::ewmh::Connection>, req: MapRequestEvent);
    fn destroy_notify(conn: &Arc<xcb_util::ewmh::Connection>, req: DestroyNotifyEvent);
}