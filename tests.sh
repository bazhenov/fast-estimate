#!/bin/sh

fswatch -i 0.3 -0 src/*.rs | xargs -0 -n 1 -- sh -c "clear; echo 'Running tests...'; cargo test -q list"
