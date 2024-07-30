use manager::WinManager;

// mod manager;
mod plugins;
mod manager;
mod utils;
// mod wrapped;

use plugins::core_plugin::CorePlugin;

#[tokio::main]
async fn main() {
    println!("Starting");

    let mut wm = WinManager::new();
    wm.plugin(CorePlugin {});
    wm.start()
}
