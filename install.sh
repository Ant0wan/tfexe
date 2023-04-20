#!/usr/bin/env bash
curl $(curl -s https://api.github.com/repos/Ant0wan/tfexe/releases/latest | grep browser_download_url | awk -F '": ' '{print $2}' | xargs echo) -o tfexe
sudo install tfexe /usr/local/bin
