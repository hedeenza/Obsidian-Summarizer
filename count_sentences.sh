#!/bin/bash

# Assign argument 1 to a variable
FILE=$1

# Use grep to find all sentences ending in a period, exclamation point, or question mark
# Only print the matching parts of the line (-o)
# Pipe the result into the line count to get the number of sentences
SENTENCES=$(grep -o '[$.!?]' $FILE | wc -l)

# Print the result to the terminal
echo "$FILE = $SENTENCES sentences"
