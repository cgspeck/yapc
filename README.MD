# Yet Another Pushover Client

Inspired by [Ntfy](https://github.com/dschep/ntfy) & [Ntf](https://github.com/hrntknr/ntf).

* single pre-compiled binary
* uses layed config, getting settings from `/etc/default/yapc.yaml`, `~/.config/yapc.yaml` or the environment
* only supports pushover
* bash shell hook (WIP)
* Linux only


## Install

Download latest binary from release page, or to compile from source:

```shell
cargo build --release
```

Copy it to your path.

Then:

```shell
$ yapc send --help 
sends a message

Usage: yapc send [OPTIONS] <BODY> [SUBJECT]

Arguments:
  <BODY>     
  [SUBJECT]  

Options:
  -u, --url <URL>              
  -t, --url-title <URL_TITLE>  
  -p, --prioity <PRIOITY>      
  -s, --sound <SOUND>          [possible values: pushover, bike, bugle, cashregister, classical, cosmic, falling, gamelan, incoming, intermission, magic, mechanical, pianobar, siren, spacealarm, tugboat, alien, climb, persistent, echo, updown, vibrate, none]
  -h, --help                   Print help
```

## Shell integration

`yapc` has support for **automatically** sending notifications when long
running commands finish in bash and zsh. In bash it emulates zsh's preexec and
precmd functionality with `rcaloras/bash-preexec <https://github.com/rcaloras/bash-preexec>`_.

To enable it add the following to your ``.bashrc`` or ``.zshrc``:

```shell
eval "$(yapc shell-integration bash)"
# or
eval "$(yapc shell-integration zsh)"
```

To avoid unnecessary notifications:
 
* listed programs are excluded;
* notifications are not sent for commands that return in less then 60 seconds

The defaults are set is in [shell-hook](src/assets/shell-hook) and may be overridden as follows:

```shell
# e.g. increase default notification threshold to 90 seconds
export AUTO_YAPC_DONE_LONGER_THAN=90
# e.g. set ignore list to following
export AUTO_YAPC_DONE_IGNORE="vim screen meld"
```

## Configuring `yapc`

The following places are serched for configuration (in order of priofity):

1. the environment variables `YAPC_APP_TOKEN`, `YAPC_USER_KEY`, `YAPC_PRIOTITY`, `YAPC_SOUND`
2. `~/config/yapc.yaml`
3. `/etc/default/yapc.yaml`

The structure of the yaml file is as follows:

```yaml
---
app_token: your pushover app token
user_key: your pushover user key
priotity: 0
sound: bike
```
