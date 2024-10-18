#!/bin/bash

# Start ChromeDriver in the background with additional flags and redirect output to /dev/null
/usr/bin/chromedriver --port=4444 &

# Run the Rust application
/app/uniqlo_scraper
