# aquote - Quote of the Day for your shell

[![crates.io](https://img.shields.io/crates/v/aquote)](https://crates.io/crates/aquote)
[![License](https://img.shields.io/crates/l/aquote)](LICENSE.md)
[![Tests Status](https://github.com/kerlilow/aquote/workflows/tests/badge.svg)](https://github.com/kerlilow/aquote/actions)

aquote fetches a random quote daily from one of the configured quote vendors.
By default, the following vendors are used:

- [They Said So](https://theysaidso.com)
- [ZenQuotes.io](https://zenquotes.io)

## Installation

### From binaries

Download the precompiled binary for your OS from the [release page](https://github.com/kerlilow/aquote/releases).

After downloading the release package for your OS, extract the archive and run `install.sh` as root.
This will install the binary and setup a scheduled job to fetch a new quote daily.

Finally, add `aquote show` to your shell startup script (or `fish_greeting.fish` for fish).

#### Bash

Add `aquote show` to `~/.bashrc`.

#### Zsh

Add `aquote show` to `~/.zshrc`.

#### fish

Add `aquote show` to the `fish_greeting` function in `~/.config/fish/functions/fish_greeting.fish`.

Example:

```fish
function fish_greeting -d "What's up, fish?"
    ...
    set_color normal

    # Add the following line
    aquote show
end
```

## License

This project is licensed under the terms of the MIT license.

See the [LICENSE.md](LICENSE.md) file in this repository for more information.
