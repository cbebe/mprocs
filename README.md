# mprocs

mprocs runs multiple commands in parallel and shows output of each command
separately.

## Screenshots

<img src="img/screenshot1.png" width="889" height="564" />
<img src="img/screenshot2.png" width="889" height="564" />

## Installation

[Download](https://github.com/pvolok/mprocs/releases) executable for your
platform and put it into a directory included in PATH.

## Usage

1. Create `mprocs.json` file
2. Run `mprocs` command

Example `mprocs.json`:

```json
{
  "procs": {
    "nvim": {
      "cmd": ["nvim"]
    },
    "server": {
      "shell": "nodemon server.js"
    },
    "webpack": {
      "shell": "webpack serve"
    },
    "tests": {
      "shell": "jest -w",
      "env": ["NODE_ENV=test"]
    }
  }
}
```

### Key bindings

Process list focused:
- `q` - Quit
- `C-a` - Focus output pane
- `x` - Kill selected process
- `X` - Force terminate selected process
- `s` - Start selected process, if it is not running
- `k` - Select previous process
- `j` - Select next process

Process output focused:
- `C-a` - Focus processes pane
