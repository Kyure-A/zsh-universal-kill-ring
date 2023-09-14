#!/usr/bin/env zsh -xeu

kill-line-arboard ()
{
    zle kill-line
    path="$(dirname "$0")"
    exec="$path/main.rs"
    rust-script $exec $CUTBUFFER
}

zle -N kill-line-arboard
