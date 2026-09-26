cargo build -p game --target x86_64-pc-windows-gnu --release

WINDOWS_EXPORT_PATH="/mnt/c/Users/olivi/Documents/arpg-ai-lab/"

cp ./target/x86_64-pc-windows-gnu/release/game.exe "$WINDOWS_EXPORT_PATH"