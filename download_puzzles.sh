#!/bin/bash

resource_home=resources

echo -e "\033[1mPuzzle Downloader 🧩 - Advent of Code 2024 ⭐🎄\033[0m"
echo ""
# Check for curl
if ! command -v curl 2>&1 >/dev/null
then
    echo "curl could not be found !"
    exit 1
fi


read -sp "Enter session id (hidden): " SESSION_ID

PROFILE_HTML="$(curl -s --cookie "session=$SESSION_ID" https://adventofcode.com/ | grep -Eo "<div class=\"user\">.*<span class=\"star-count\">.*\*</span>")"
PROFILE_NAME="$(echo "$PROFILE_HTML" | grep -oP '\(.*?\)' | sed 's/[()]//g')"
PROFILE_STARS="$(echo "$PROFILE_HTML" | grep -oP '(?<=<span class="star-count">)[0-9]+\*')"

# Print profile info and prompt user to continue
echo -e "\033[A\033[2K"
#echo
echo -e "Found user \033[32m$PROFILE_NAME\033[0m with \033[1;33m$PROFILE_STARS stars\033[0m"
echo ""
read -p "Do you want to proceed with download? (y/n): " response

# Check the response
if [[ "$response" =~ ^[Yy]$ ]]; then
    echo -e "\033[A\033[2K"
else
    echo "Exiting..."
    exit 1
fi

for DAY in $(seq 1 14);
do
    DAY_LEADING_ZERO=$(printf "%02d" "$DAY")
    mkdir -p "$resource_home/day$DAY_LEADING_ZERO"
    PUZZLE_FILE=$resource_home/day$DAY_LEADING_ZERO/input.txt

    if [ -f "$PUZZLE_FILE" ]; then
    	echo "Puzzle for $DAY exists (Skipping)"
    else 
    	echo "Downloading puzzle for day $DAY"
        curl -s --cookie "session=$SESSION_ID" https://adventofcode.com/2024/day/$DAY/input > "$PUZZLE_FILE"
    fi
done