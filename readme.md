# start here

clone our repo

    git clone --recurse-submodules git@github.com:Vaultemore/api.git

or

    git clone git@github.com:Vaultemore/api.git
    git submodule update --init --recursive

Then clone adjacent repos (this is because they're relatively referenced for ease)

    git clone git@github.com:Vaultemore/foundry.git



# Install Rust

    https://www.rust-lang.org/tools/install

Add rust-analyser to vscode

    rustup component add rustfmt


# run

    cd api

    cargo build

    cargo run

# notes on submodules

Foundry likes submodules with `forge install ...` so we submoduled it in `contracts` dir to keep this feature, so you need to be aware of some quirks https://git-scm.com/book/en/v2/Git-Tools-Submodules

    git submodule add https://github.com/Vaultemore/contracts.git
    git submodule add git@github.com:Vaultemore/vaultemore.git
