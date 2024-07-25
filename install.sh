sudo apt-get update -qq
sudo apt-get install -y libx11-xcb-dev libxcb-ewmh-dev libxcb-icccm4-dev libxcb-keysyms1-dev
chmod +x run.sh
cargo build