# Dummy Ledger implementation

I do not like Actual Budget. We used it for almost a year now and I have come to the conclusion that it does not suite my needs.
So I am going to write a minimalistic double-entry bookkeeping solution instead that I can script/tweak as I wish and properly automate using a minimalistic UI.

This is currently not a real double-entry bookkeeping solution but a first exploration into how to process and also import bank statements (simplified).

# Notes

## nix

If you should be ever so curious and want to run this using nix (flakes):

```
nix develop -c zsh # or the shell of your choice, this should put you into a devShell with everything this project needs
```

I did not add security scanning yet, but I intend to in the future because [https://github.com/ipetkov/crane](https://github.com/ipetkov/crane) as very nice examples on how to set it up.

## Performance

I did not benchmark this dummy project. I went with two plain `HashMap`s. We could have a more distrubed approach where the input get's fanned out into N worker streams, where workers are sharded by accounts and only receive the respective transactions. They then keep their local state and submit an aggregated account update every M ticks to an aggregator/final/"true" append only log. E.g., in a CQRS style manner, since every transaction (type) is a kind of command.


## Dependencies

To keep it simple I use 
```toml
tempfile = "3.23.0"
```
as dev-dependency.

It has an amount of dependencies that is almost too much for the basic functionality of a tempdir in my opinion.

```
dev-dependencies]
└── tempfile v3.23.0
    ├── fastrand v2.3.0
    ├── getrandom v0.3.4
    │   ├── cfg-if v1.0.4
    │   └── libc v0.2.177
    ├── once_cell v1.21.3
    └── rustix v1.1.2
        ├── bitflags v2.10.0
        └── linux-raw-sys v0.11.0
```

For my work in progress tiny systemd-creds helper lib I went and copied nightly (not sure if it's still nighlty now) rust std code instead and build only the required parts I need for a tempdir. Also since it's linux only I do not need to copy all that many OS specific files. [https://github.com/daemonfire300/systemd-creds-rs/blob/master/src/lib.rs#L87-L136](https://github.com/daemonfire300/systemd-creds-rs/blob/master/src/lib.rs#L87-L136)
At the end it's a trade-off. The linked solution does not have the neat feature of `tempfile` with the `Drop` trait for example.

