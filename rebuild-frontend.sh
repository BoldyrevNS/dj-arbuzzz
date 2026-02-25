#!/bin/bash

echo "Rebuilding frontend..."
docker-compose build frontend

echo "Restarting frontend..."
docker-compose up -d frontend

echo "Watching logs (Ctrl+C to exit)..."
docker-compose logs -f frontend
