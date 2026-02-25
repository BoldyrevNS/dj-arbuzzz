#!/bin/bash

set -e

echo "=== Starting Local Development Environment ==="
echo ""

# Check if .env exists
if [ ! -f .env ]; then
    echo "⚠️  .env file not found. Creating from .env.example..."
    cp .env.example .env
    echo "✅ .env created. Please edit it with your settings."
    echo ""
fi

# Start infrastructure
echo "1. Starting PostgreSQL and Redis..."
docker-compose up -d postgres redis

echo "   Waiting for PostgreSQL to be ready..."
sleep 3

# Run migrations
echo ""
echo "2. Running database migrations..."
docker-compose up migrations

echo ""
echo "3. Starting services..."
echo ""
echo "   ⚠️  You need to start backend and frontend manually in separate terminals:"
echo ""
echo "   Terminal 1 (Backend):"
echo "   └─ cd server && cargo run"
echo ""
echo "   Terminal 2 (Frontend):"
echo "   └─ cd client && pnpm dev"
echo ""
echo "   After starting both, open: http://localhost:3000"
echo ""
echo "=== Infrastructure is ready ==="
docker-compose ps
