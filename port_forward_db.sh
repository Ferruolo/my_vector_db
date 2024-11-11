#!/bin/bash

# Start redis port-forward in the background
sudo kubectl port-forward service/redis 6379:6379 &
REDIS_PID=$!

# Start cassandra port-forward in the foreground
sudo kubectl port-forward service/cassandra 9042:9042 &
CASSANDRA_PID=$!

# Function to handle script termination
cleanup() {
    echo "Stopping port-forwarding..."
    kill $REDIS_PID
    kill $CASSANDRA_PID
    exit 0
}

# Set up trap to catch termination signals
trap cleanup SIGINT SIGTERM

# Keep the script running
echo "Port forwarding active. Press Ctrl+C to stop."
wait

