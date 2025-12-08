#!/bin/bash
cargo clippy --workspace -q --message-format json | grep ^"{\"reason\":\"compiler-message\"" | tee clippy.json

# File containing JSON data
json_file="clippy.json"
runner=$1

# Loop through each object in the JSON array
while read -r obj; do
  # Extract each field and assign it to a variable
  message=$(echo "$obj" | jq -r '.message.message')
  level=$(echo "$obj" | jq -r '.message.level')
  file_name=$(echo "$obj" | jq -r '.message.spans[0].file_name')
  line_start=$(echo "$obj" | jq -r '.message.spans[0].line_start')

  # Now you can use the variables inside the loop
  # echo "Message: $message"
  # echo "Level: $level"
  # echo "File: $file_name"
  # echo "Line: $line_start"
  # echo "-------------------------"
  echo "::${level} file=${file_name},line=${line_start}::${message}"

  # Additional processing can be done here with the variables
done <$json_file
