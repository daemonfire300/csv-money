# Dummy Ledger implementation

I do not like [Actual](https://www.actualbudget.com/) [Budget](https://github.com/actualbudget/actual). We used it for almost a year now and I have come to the conclusion that it does not suit my needs.
So I am going to write a minimalistic double-entry bookkeeping solution instead that I can script/tweak as I wish and properly automate using a minimalistic UI.

This is currently not a real [double-entry bookkeeping](https://quickbooks.intuit.com/r/bookkeeping/complete-guide-to-double-entry-bookkeeping/) solution but a first exploration into how to process and also import bank statements (simplified).

# Notes

## Implementation Details

The initial draft was built with the assumption that you cannot chargeback/dispute withdrawals. Later I realized this should probably also be able.
This is when the code got a bit more convoluted in my opinion, so I would probably do another refactor with that in my mind when I do the real double-entry bookkeeping implementation (where I have to do individual/multiple transactions for cross-account movements).

## Misc

  1. `pub(crate)` is not used 100% consistently (in some places I forgot to correct it)
  2. Decimals are not handled consistently, sometimes 0 is 0 and sometimes it's 0.0, going to change this some other day (there is an option in the library to fixate this)

## git

The commit messages are a bit loose, excuse me.

## nix

If you should be ever so curious and want to run this using nix (flakes):

```
nix develop -c zsh # or the shell of your choice, this should put you into a devShell with everything this project needs
```

I did not add security scanning yet, but I intend to in the future because [https://github.com/ipetkov/crane](https://github.com/ipetkov/crane) as very nice examples on how to set it up.

## Performance


### HashMap(s) and buffers

Based on the data ingest/egress one could apply heuristics like file-size before creating the `Processor` and pre-allocating a bigger `HashMap` to avoid too many re-allocations it due to growth.
Similar for the buffer size, if reads/writes are costly or a bottleneck, a bigger buffer might be more advantages to avoid starving due to too many frequent syscalls.


### Decimal

The decimal lib is a bit overkill for regular bookkeeping, there is "too much" precision and we could probably use less bytes, but it's convenient.

### UTF-8 vs "raw"

Assuming all statements are in ASCII we could maybe optimize the deserialization here, e.g., go over chunks of `&[u8]` instead of contructing strings, but that's just a hunch. I have not looked into this yet and it might very well end up not improving things depending on the volume of transactions.

### In General

I did not benchmark this dummy project. I went with two plain `HashMap`s. We could have a more distrubed approach where the input get's fanned out into N worker streams, where workers are sharded by accounts and only receive the respective transactions. They then keep their local state and submit an aggregated account update every M ticks to an aggregator/final/"true" append only log. E.g., in a CQRS ([3 years old CQRS golang demo](https://github.com/daemonfire300/cqrs-demo-go)) style manner, since every transaction (type) is a kind of command.


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


# Disclaimer

I wrote this while moving between cities.
