# kill-ring.zsh
## Dependency
- [rust-script](https://rust-script.org)

## Motivation
Some environments do not have pbcopy or xsel (default Ubuntu does). We have created a command to use [**arboard**](https://github.com/1Password/arboard) as a means of using the clipboard without relying on these.

## Usage
It can be installed with any zsh plugin manager, or cloned and loaded with source. afterwards, add the following code to your `.zshrc`.

```zsh
bindkey '^k' kill-line-arboard
```
