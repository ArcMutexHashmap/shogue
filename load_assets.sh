#!/bin/bash
echo "Downloading tileset..."
mkdir -p assets
curl -o assets/tiles.zip https://opengameart.org/sites/default/files/Dungeon%20Crawl%20Stone%20Soup%20Full_0.zip
unzip assets/tiles.zip -d assets
rm assets/tiles.zip
mv assets/Dungeon\ Crawl\ Stone\ Soup\ Full/* assets