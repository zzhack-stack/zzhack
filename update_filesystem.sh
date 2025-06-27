#!/bin/bash
# Script to regenerate filesystem metadata when data directory changes

echo "Regenerating filesystem metadata..."
node generate_metadata.js

if [ $? -eq 0 ]; then
    echo "Filesystem metadata updated successfully!"
    echo "Copying data directory to dist..."
    cp -r data dist/
    echo "Data directory copied to dist/"
    echo "You can now rebuild the application with: trunk build"
else
    echo "Error: Failed to generate filesystem metadata"
    exit 1
fi