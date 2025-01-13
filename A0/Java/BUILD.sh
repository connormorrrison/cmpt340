#!/bin/bash

# Set source and output directory
SRC_DIR="./"
OUT_DIR="./bin"

# Create temporary output directory
mkdir -p $OUT_DIR

# Compile all .java files in the current directory
javac -d $OUT_DIR $SRC_DIR/*.java

# Run Main.java class
java -cp $OUT_DIR A0.Java.Main

# Delete bin folder after execution
rm -rf $OUT_DIR