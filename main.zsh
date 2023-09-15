#!/usr/bin/env zsh -xeu

SCRIPT_DIR=${0%/*}

copy-line-as-kill ()
{
    zle kill-line
    rust-script $SCRIPT_DIR/copy.rs $CUTBUFFER
}

paste-as-yank ()
{
    rust-script $SCRIPT_DIR/paste.rs
}

zle -N copy-line-as-kill
zle -N paste-as-yank
