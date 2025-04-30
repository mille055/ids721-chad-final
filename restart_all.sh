#!/bin/bash

# ========== CONFIG ==========
DOCKER_IMAGE="mille055/projectf:latest"
LLAMAFILE_BINARY="./phi-2.llamafile"  # Update as needed
LLAMAFILE_PORT=8080
APP_PORT=8000
LOG_FILE="llama.log"
# ============================

echo ">>> Checking for required tools..."

# ========== CHECK & INSTALL DOCKER IF NEEDED ==========
if ! command -v docker &> /dev/null; then
  echo ">>> Docker not found. Installing Docker..."
  sudo apt update
  sudo apt install -y docker.io
  sudo systemctl start docker
  sudo usermod -aG docker $USER
  echo ">>> Docker installed. You may need to log out and log back in for group permissions to take effect."
  echo ">>> Exiting for now. Please re-run this script after re-logging into the instance."
  exit 1
fi

# Check Llamafile binary exists
if [ ! -f "$LLAMAFILE_BINARY" ]; then
  echo "ERROR: Llamafile binary '$LLAMAFILE_BINARY' not found."
  echo "Please download it manually and make sure it is chmod +x."
  exit 1
fi

echo ">>> [1] Checking for any process using port $APP_PORT..."
PID_IN_USE=$(lsof -ti tcp:$APP_PORT)
if [ ! -z "$PID_IN_USE" ]; then
  echo ">>> Port $APP_PORT is in use by PID $PID_IN_USE. Stopping..."
  kill -9 $PID_IN_USE
  sleep 1
else
  echo ">>> Port $APP_PORT is free."
fi

echo ">>> Stopping old Docker containers using image: $DOCKER_IMAGE..."
OLD_CONTAINER_ID=$(docker ps --filter "ancestor=$DOCKER_IMAGE" --format "{{.ID}}")
if [ -n "$OLD_CONTAINER_ID" ]; then
  docker stop "$OLD_CONTAINER_ID"
  echo ">>> Stopped container ID: $OLD_CONTAINER_ID"
else
  echo ">>> No running Docker containers found for this image."
fi

echo ">>> Stopping any running llamafile processes..."
OLD_LLAMAFILE_PID=$(ps aux | grep "$LLAMAFILE_BINARY" | grep -v grep | awk '{print $2}')
if [ -n "$OLD_LLAMAFILE_PID" ]; then
  kill "$OLD_LLAMAFILE_PID"
  echo ">>> Stopped llamafile process ID: $OLD_LLAMAFILE_PID"
else
  echo ">>> No llamafile process found."
fi

echo ">>> Pulling latest Docker image: $DOCKER_IMAGE..."
docker pull "$DOCKER_IMAGE" 

sleep 2

echo ">>> Starting llamafile server silently..."
nohup stdbuf -oL "$LLAMAFILE_BINARY" --server --host 0.0.0.0 > "$LOG_FILE" 2>&1 < /dev/null &
disown
echo ">>> Llamafile started in background. Logs: $LOG_FILE"

sleep 5

echo ">>> Starting Docker app container in detached mode..."
docker run -d --platform linux/amd64 --network host \
  -e LLAMAFILE_URL=http://127.0.0.1:$LLAMAFILE_PORT \
  "$DOCKER_IMAGE"

echo ">>> Setup complete."
echo ""
echo "=== Monitoring Tips ==="
echo "- View llamafile logs: tail -f $LOG_FILE"
echo "- View app container logs:"
echo "    docker ps            # to get container ID"
echo "    docker logs -f <ID>  # to follow logs live"
