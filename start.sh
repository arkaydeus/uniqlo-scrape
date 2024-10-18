#!/bin/bash

# Start ChromeDriver in the background with additional flags and redirect output to /dev/null
chromedriver --port=4444 --whitelisted-ips="" --verbose > /dev/null 2>&1 &

# Wait for ChromeDriver to start
sleep 5

# Run the Rust application
./uniqlo_scraper
