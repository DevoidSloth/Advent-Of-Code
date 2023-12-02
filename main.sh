#!/bin/bash

# Get the current year
current_year=$(date +"%Y")

# Check if a parameter is passed
if [ $# -eq 0 ]; then
  echo "No day number provided. Usage: ./create_project.sh <day_number>"
  exit 1
fi

# Use the provided parameter as the day number
day_number=$1

# Create the year directory if it doesn't exist
if [ ! -d "$current_year" ]; then
  mkdir "$current_year"
fi

# Check if the day number is odd
if [ $((day_number % 2)) -ne 0 ]; then
  # The day number is odd, create a Python project
  project_name="$current_year/day$day_number"
  mkdir -p "$project_name"
  touch "$project_name/main.py"
  touch "$project_name/in.txt" # Create the in.txt file
  echo "Python project '$project_name' with 'in.txt' file created."
else
  # The day number is even, create a Rust project
  project_name="$current_year/day$day_number"
  if [ -d "$project_name" ]; then
    echo "Error: The directory '$project_name' already exists."
    exit 1
  fi
  cargo new "$project_name"
  touch "$project_name/in.txt" # Create the in.txt file
  echo "Rust project '$project_name' with 'in.txt' file created."
fi
