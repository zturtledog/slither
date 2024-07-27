use nmanager::WinManager;

// mod manager;
mod plugin;
mod plugins;
mod nmanager;
// mod wrapped;

use plugins::core_plugin::CorePlugin;

#[tokio::main]
async fn main() {
    println!("Starting");

    WinManager::new()
        .plugin(CorePlugin {})
        .start()
}
