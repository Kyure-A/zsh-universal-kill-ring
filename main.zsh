#!/usr/bin/env zsh -xeu

kill-line-arboard ()
{
    zle kill-line
    rust-script ./main.rs $CUTBUFFER
}

zle -N kill-line-arboard
