# Raniz Advent of Code '2022
This repository contains my solutions for AoC '2022 in Rust.

Feel free to copy it as a starting point for your own solutions.

## Usage
Create a file name .env in the root of the project and put your AoC session token in there:
```shell
AOC_SESSION="..."
```
You can get the session from the cookie _session_ on the AoC website once you have logged in.

Then you can just run `./start-day.sh X` where X is the day you want to start and the script will create a new Rust project in the workspace and download the input there.