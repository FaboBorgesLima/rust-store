if [ "$IS_DEV" = true ]; then 
    npm run dev-build
else 
    npm run build
fi