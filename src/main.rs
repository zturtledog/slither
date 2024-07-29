use manager::WinManager;

// mod manager;
mod plugin;
mod plugins;
mod manager;
mod macros;
// mod wrapped;

use plugins::core_plugin::CorePlugin;

#[tokio::main]
async fn main() {
    println!("Starting");

    let mut wm = WinManager::new();
    wm.plugin(CorePlugin {});
    wm.start()
}
