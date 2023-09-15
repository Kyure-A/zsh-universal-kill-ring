#!/usr/bin/env zsh -xeu

copy-line-as-kill ()
{
    zle kill-line
    path=$(dirname $0)
    exec=$path/copy.rs
    rust-script $exec $CUTBUFFER
}

paste-as-yank ()
{
    path=$(dirname $0)
    exec=$path/paste.rs
    rust-script $exec
}

zle -N copy-line-as-kill
zle -N paste-as-yank
