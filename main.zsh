#!/usr/bin/env zsh -xeu

SCRIPT_DIR=${0%/*}

copy-line-as-kill ()
{
    zle kill-line
    rust-script $SCRIPT_DIR/src/main.rs copy $CUTBUFFER
}

paste-as-yank ()
{
    rust-script $SCRIPT_DIR/src/main.rs paste
}

zle -N copy-line-as-kill
zle -N paste-as-yank
