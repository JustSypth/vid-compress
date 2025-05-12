#!/bin/bash

echo "----------------------------------------"
echo Installation Script for vid-compress
echo "----------------------------------------"
echo

SRC_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Prompt a user for agreement
while true
do
    echo "The installation will require sudo priviliges"
    read -p "Do you want to proceed with installation (y/n): " agree
    agree="${agree,,}"

    if [ "$agree" = "y" ]; then
        break
    elif [ "$agree" = "n" ]; then
        exit
    else
        echo "Invalid input"
    fi
done

echo
echo "Installing resources.."
sudo mkdir -p /usr/local/lib/vid-compress
sudo cp -r "$SRC_DIR/assets" /usr/local/lib/vid-compress/
sudo cp -r "$SRC_DIR/bin" /usr/local/lib/vid-compress/

echo
echo "Installing app.."
sudo cp "$SRC_DIR/vid-compress" /usr/local/bin/vid-compress
sudo chmod +x /usr/local/bin/vid-compress
sudo cp "$SRC_DIR/vid-compress.desktop" /usr/local/share/applications/vid-compress.desktop
sudo chmod +x /usr/local/share/applications/vid-compress.desktop

echo
echo "Installation Finished!"
