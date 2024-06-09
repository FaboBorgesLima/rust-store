if [ "$IS_DEV" = true ]; then 
    echo "building with development config"
    cargo build
    cp /app/target/debug/app /build/app
else 
    echo "building with release config"
    cargo build -r
    cp /app/target/release/app /build/app
fi