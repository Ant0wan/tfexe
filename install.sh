#!/usr/bin/env bash
wget "$(wget --output-document - https://api.github.com/repos/Ant0wan/tfexe/releases/latest 2>/dev/null | grep browser_download_url | awk -F '": ' '{print $2}' | xargs echo)"
sudo install tfexe /usr/local/bin
rm tfexe
