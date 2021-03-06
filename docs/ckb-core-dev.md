# CKB Core Development

## Well-known Hashes

The command `ckb cli hashes` prints the well-known hashes for current
effective chain spec.

The file `docs/hashes.toml` contains the well-known hashes for all the bundled
chain specs. The file is generated by:

```
cargo run cli hashes -b > docs/hashes.toml
```

## Running Test

Install dependencies

```
rustup component add rustfmt
rustup component add clippy
```

Run tests

```
make ci
```

Run acceptance integration tests

```
make integration
```

## Chain Spec

The chain spec in `specs/dev.toml` can switch between different PoW engines.

CKB now supports following PoW Engines.

### Cuckoo

```
[pow]
func = "Cuckoo"

[pow.params]
# the 2-log of the graph size, which is the size in bits of the node
# identifiers
edge_bits = 15

# length of the cycle to be found, must be an even number, a minimum of 12 is
# recommended
cycle_length = 12
```

### Dummy

```
[pow]
func = "Dummy"

# Delay offset (in milliseconds)
[pow.params.delay]
type = "constant"
value = 5000
```
