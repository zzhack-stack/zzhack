#!/bin/bash
# Build script for the terminal emulator using Trunk
# This script builds the Rust Yew application for production deployment

echo "Building terminal emulator with Trunk..."
echo "Installing dependencies if not available..."

# Check if trunk is installed, install if not
if ! command -v trunk &> /dev/null; then
    echo "Trunk not found. Installing trunk..."
    cargo install --locked trunk
fi

echo "Building for release..."
trunk build --release

echo "Generating TailwindCSS styles..."
npx tailwindcss@3.4.0 -i ./input.css -o ./dist/output.css --minify

echo "Build complete! The application is ready in the 'dist' directory."
echo "To serve the application locally, run:"
echo "trunk serve --open"
echo "Or serve the dist directory with any static file server."
