#!/bin/bash

set -e

REPO="xhon4/nightscape"
VERSION="v0.1.0"

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="linux"
    FILE="nightscape"
elif [[ "$OSTYPE" == "msys"* || "$OSTYPE" == "cygwin"* ]]; then
    PLATFORM="windows"
    FILE="nightscape.exe"
else
    echo "Unsupported platform: $OSTYPE"
    exit 1
fi

URL="https://github.com/$REPO/releases/download/$VERSION/$FILE"
DEST="/usr/local/bin/$FILE"

echo "Downloading $FILE from $URL..."
curl -L "$URL" -o "$FILE"
chmod +x "$FILE"

echo "Installing $FILE to $DEST..."
sudo mv "$FILE" "$DEST"

echo "Installation complete! Run 'nightscape' to start the program."
