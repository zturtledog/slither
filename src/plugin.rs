use xcb::{
    ClientMessageEvent, 
    ConfigureRequestEvent, 
    GenericEvent, 
    KeyPressEvent, 
    MapRequestEvent,
    PropertyNotifyEvent,
    EnterNotifyEvent,
    UnmapNotifyEvent,
    DestroyNotifyEvent,
};
use std::ops::Deref;
use std::sync::Arc;

pub trait Plugin {
    fn init(&self) {}

    //events
    fn on_client_message(&self, _conn: Arc<xcb_util::ewmh::Connection>, _event: &ClientMessageEvent) {}
    fn on_key_press(&self, _conn: Arc<xcb_util::ewmh::Connection>, _event: &KeyPressEvent) {}
    fn on_configure_request(&self, _conn: Arc<xcb_util::ewmh::Connection>, _event: &ConfigureRequestEvent) {}
    fn on_map_request(&self, _conn: Arc<xcb_util::ewmh::Connection>, _event: &MapRequestEvent) {}
    fn on_property_notify(&self, _conn: Arc<xcb_util::ewmh::Connection>, _event: &PropertyNotifyEvent) {}
    fn on_enter_notify(&self, _conn: Arc<xcb_util::ewmh::Connection>, _event: &EnterNotifyEvent) {}
    fn on_unmap_notify(&self, _conn: Arc<xcb_util::ewmh::Connection>, _event: &UnmapNotifyEvent) {}
    fn on_destroy_notify(&self, _conn: Arc<xcb_util::ewmh::Connection>, _event: &DestroyNotifyEvent) {}

    fn unspecified_event(&self, _conn: Arc<xcb_util::ewmh::Connection>, _response_type: u8, _event: &GenericEvent) {}
}

pub struct StructuredPlugin{
    value: Box<dyn Plugin>
}
impl Deref for StructuredPlugin {
    type Target = Box<dyn Plugin>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl StructuredPlugin {
    pub fn new(value: impl Plugin + 'static) -> Self {
        Self {
            value:Box::new(value)
        }
    } 
}
unsafe impl Sync for StructuredPlugin {}
unsafe impl Send for StructuredPlugin {}