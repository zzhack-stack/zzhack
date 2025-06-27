#!/bin/bash

# Development startup script for terminal emulator
# This script starts both the backend server and frontend development server

echo "Starting Terminal Emulator Development Environment..."

# Function to kill background processes on exit
cleanup() {
    echo "Stopping development servers..."
    kill $SERVER_PID $FRONTEND_PID 2>/dev/null
    exit
}

# Set up signal handlers
trap cleanup INT TERM

# Start the backend server
echo "Starting backend server on port 8081..."
cd server
cargo run &
SERVER_PID=$!

# Wait a moment for server to start
sleep 2

# Start the frontend development server
echo "Starting frontend development server on port 8080..."
cd ..
trunk serve --open &
FRONTEND_PID=$!

echo "Development environment started!"
echo "Backend: http://localhost:8081"
echo "Frontend: http://localhost:8080"
echo "Press Ctrl+C to stop both servers"

# Wait for either process to finish
wait $SERVER_PID $FRONTEND_PID