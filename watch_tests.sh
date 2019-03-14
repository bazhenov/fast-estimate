#!/bin/sh

fswatch -i 0.1 src/**.rs | xargs -n 1 sh -c "clear; cargo test -- --nocapture $1"
