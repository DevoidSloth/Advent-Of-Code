#!/bin/bash

# Get the current year
current_year=$(date +"%Y")

# Check if a day number is provided
if [ $# -eq 0 ]; then
  echo "Please provide a day number to run the code. Usage: ./run.sh <day_number> [year]"
  exit 1
fi

# Assign the provided day number to a variable
day_number=$1

# Check if a specific year is provided, otherwise use the current year
if [ $# -eq 2 ]; then
  year=$2
else
  year=$current_year
fi

project_directory="$year/day$day_number"

# Check if the project directory exists
if [ ! -d "$project_directory" ]; then
  echo "The project directory '$project_directory' does not exist."
  exit 1
fi

# Check if it's a Python project and run it
if [ -f "$project_directory/main.py" ]; then
  echo "Running Python project for '$year/day$day_number'."
  python3 "$project_directory/main.py"
  exit 0
fi

# Check if it's a Rust project and run it
if [ -f "$project_directory/src/main.rs" ]; then
  echo "Running Rust project for '$year/day$day_number'."
  cd "$project_directory"
  cargo run
  exit 0
fi

echo "No runnable code found for '$year/day$day_number'."
