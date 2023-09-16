# universal-kill-ring.zsh
## Dependency
- [rust-script](https://rust-script.org)

## Motivation
Some environments do not have pbcopy or xsel (default Ubuntu does). We have created a command to use [**arboard**](https://github.com/1Password/arboard) as a means of using the clipboard without relying on these.

## Usage
It can be installed with any zsh plugin manager, or cloned and loaded with source. afterwards, add the following code to your `.zshrc`.

```zsh
bindkey '^k' copy-line-as-kill
bindkey '^y' paste-as-yank
```

Enjoy an Emacs-like kill-ring experience! (The first time you install this, this is slow because of compile, but after that this is stress-free and fast).


## Todo
- [x] Copy
- [x] Paste
  - [ ] Search and paste from history
- [ ] History in json format 
