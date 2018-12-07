#!/bin/sh

fswatch -i 0.3 src | grep --line-buffered ".rs$" |  xargs -n 1 -- sh -c "clear; cargo test -q list"
