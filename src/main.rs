use nmanager::WinManager;

// mod manager;
// mod plugin;
mod nmanager;
// mod wrapped;

#[tokio::main]
async fn main() {
    println!("Starting");

    WinManager::new().start()
}
