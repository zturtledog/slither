use manager::Manager;

mod manager;

#[tokio::main]
async fn main() {
    println!("Starting");

    Manager::new().start()
}
