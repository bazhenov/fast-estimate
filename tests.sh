#!/bin/sh

fswatch src | xargs -n 1 -- sh -c "clear; cargo test -q list"
