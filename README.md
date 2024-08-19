# alaconfig
A handy tool to edit your alacritty config from the command line without editing the config file.
## Installation
```sh
cargo install alaconfig
```

## Commands
There are only 2 commands
`set` => set a config parameter
> eg. 
> ```
> $> alaconfig set font jetbrains mono
> ```

`get` => gets a set config parameter
> eg.
> ```
> $> alaconfig get font
> family = "Jetbrains Mono"
> ```

## Progress

| command | status |
|---------|--------|
| font    | done   |
| opacity | done   |
| size    | done   |
| blur    | done   |
| theme   | done   |


