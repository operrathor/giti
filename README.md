# Giti

Indicates the working directory status of configured Git repositories.

## Install

```sh
cargo install --path .
```

## Configure

Create and edit `~/.config/giti/repositories.json`:

```json
[
    {
        "gitDir": "/absolute/path/to/repository/.git",
        "workTree": "/absolute/path/to/repository",
        "name": "Some alias"
    },
    {
        "gitDir": "/absolute/path/to/another_repository/.git",
        "workTree": "/absolute/path/to/another_repository",
        "name": "Some other alias"
    }
]
```

## Run

```sh
giti
```

### Sample output

```
👌 3rd
🤷 4th | Could not determine status of repository
👌 Notes
👎 dotfiles
```
