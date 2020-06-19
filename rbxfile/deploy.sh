#!/usr/bin/env bash

cargo build --release

if [ $? -ne 0 ]; then
    echo "Something went wrong building with cargo. Closing script"
    exit 1
fi

if [ ! -d "$HOME/bin" ]; then
    echo "Making a bin directory in $HOME"
    mkdir $HOME/bin
fi

echo "Moving executable to bin directory"
cp -f target/release/rbxfile $HOME/bin

echo "Deploy Finished!"
exit 0
