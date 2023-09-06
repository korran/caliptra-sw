#!/bin/bash
echo Please enter the GitHub Actions jitconfig base64:
read line
echo Calling with jitconfig
./run.sh --jitconfig "${line}"
echo "GHA runner complete; shutting down"
shutdown -h now