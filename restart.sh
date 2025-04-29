#!/bin/bash

# ========== CONFIG ==========
DOCKER_IMAGE="mille055/projectf:latest"
LLAMAFILE_BINARY="./phi-2.llamafile"
LLAMAFILE_PORT=8080
APP_PORT=8000
# =============================

echo ">>> Stopping old Docker containers using port $APP_PORT..."
OLD_CONTAINER_ID=$(docker ps --filter "ancestor=$DOCKER_IMAGE" --format "{{.ID}}")
if [ ! -z "$OLD_CONTAINER_ID" ]; then
  docker stop "$OLD_CONTAINER_ID"
  echo ">>> Docker container $OLD_CONTAINER_ID stopped."
else
  echo ">>> No existing Docker containers to stop."
fi

echo ">>> Stopping any old llamafile processes..."
OLD_LLAMAFILE_PID=$(ps aux | grep "$LLAMAFILE_BINARY" | grep -v grep | awk '{print $2}')
if [ ! -z "$OLD_LLAMAFILE_PID" ]; then
  kill "$OLD_LLAMAFILE_PID"
  echo ">>> Llamafile process $OLD_LLAMAFILE_PID killed."
else
  echo ">>> No llamafile process found."
fi

sleep 2

echo ">>> Restarting llamafile server (silent)..."
nohup stdbuf -oL $LLAMAFILE_BINARY --server --host 0.0.0.0 > phi2.log 2>&1 < /dev/null &
disown
sleep 5

echo ">>> Restarting Docker app (live logs)..."
docker run --platform linux/amd64 --network host -e LLAMAFILE_URL=http://127.0.0.1:$LLAMAFILE_PORT $DOCKER_IMAGE

echo ">>> Setup complete."
echo ">>> (Llamafile logs to phi2.log. App logs are shown here.)"

