macro_rules! event {
    ($handler:ident, $connection:ident, $clients:ident, $plugins:ident, $out_event:ty, $event:ident) => {{
        let eve = unsafe { std::mem::transmute::<xcb::GenericEvent, $out_event>($event) };
        $plugins
            .iter()
            .for_each(|plugin| plugin.$handler($connection.clone(), $clients.clone(), &eve));
    }};
}

pub(crate) use event;
