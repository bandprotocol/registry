# Signals Registry

This repository contains the signal_ids for BandProtocol feeds module.
A `registry.json` contains the aggregated signals that makes it easy to start using the signals in Bothan.

Schema files containing the recommended metadata structure can be found in `*.schema.yaml` files located in
the [`schema/`](/schema) directory.

# Signals

The metadata in these files represents the sources and its corresponding routes used to compute the signal value. For
more details on the structure of the `signal.yaml` and its fields, please refer to the schema file located in
[`schema/signal.schema.yaml`](/schema/signal.schema.yaml).

An example `signal.yaml` can also be seen below:

```yaml
sources:
- source_id: binance
  id: btcusdc
  routes:
  - signal_id: CS:USDC-USD
    operation: '*'
- source_id: binance
  id: btcusdt
  routes:
  - signal_id: CS:USDT-USD
    operation: '*'
- source_id: coinbase
  id: BTC-USD
- source_id: coinbase
  id: BTC-USDT
  routes:
  - signal_id: CS:USDT-USD
    operation: '*'
- source_id: coingecko
  id: bitcoin
- source_id: coinmarketcap
  id: '1'
- source_id: kraken
  id: XBT/USD
- source_id: kraken
  id: XBT/USDC
  routes:
  - signal_id: CS:USDC-USD
    operation: '*'
- source_id: kraken
  id: XBT/USDT
  routes:
  - signal_id: CS:USDT-USD
    operation: '*'
- source_id: okx
  id: BTC-USDC
  routes:
  - signal_id: CS:USDC-USD
    operation: '*'
- source_id: okx
  id: BTC-USDT
  routes:
  - signal_id: CS:USDT-USD
    operation: '*'
```

# Prefix

The metadata in these files represents the processor used for all signals with the corresponding prefix. For more
details
on the structure of the `prefix.yaml` and its fields, please refer to the schema file located in
[`schema/prefix.schema.yaml`](/schema/prefix.schema.yaml).

An example `prefix.yaml` can also be seen below:

```yaml
processor:
  function: median
  params:
    min_source_count: 3
```

# Contributing

We accept pull requests for new signals or changes to existing signals. Please give the pull request a descriptive title
and explain the changes you are making in the description.

For example:

```
# Title 
"Added New Signal CS:BTC-USD"

# Description 
"Added new signal CS:BTC-USD with sources from binance, coinbase, coingecko, coinmarketcap, kraken and okx"
```

## Adding/Modifying Signals

- Navigate to the appropriate prefix directory under `./signals/`.
- If a new signal is being created, create a new file with the signal suffix. For instance, to add signal `CS:BTC-USD`,
  we create a file
  called `BTC-USD.yaml`. If a signal already exists, and you want to modify it, edit the existing file.
- In the file, define the sources and their corresponding routes (if any) to compute the signal.
- Assure that any referenced prerequisite signals already exist in the registry. If they do not, please add them as
  well. Also, make sure that the dependencies used do not form a circular dependency. For example, if `CS:BTC-USD`
  references `CS:USDT-USD`, `CS:USDT-USD` cannot reference `CS:BTC-USD`

### Example: Adding a New Signal `CS:BTC-USD`

Let's say you want to add a new signal called `CS:BTC-USD` where its sources are binance and coinbase, the file would
look like this:

```yaml
sources:
- source_id: binance
  id: btcusdc
  routes:
  - signal_id: CS:USDC-USD
    operation: '*'

- source_id: binance
  id: btcusdt
  routes:
  - signal_id: CS:USDT-USD
  operation: '*'

- source_id: coinbase
  id: BTC-USD
```

In this case, the signal `CS:BTC-USD` is derived from Binance's `btcusdc` and `btcusdt` pairs and Coinbase's
direct `BTC-USD` pair. The Binance `btcusdc` route indicate that the value for `BTC-USD` will be computed by
multiplying `btcusdc` from Binance and the signal value for`CS:USDC-USD`.

When adding a signal, ensure any referenced prerequisite signals (like `CS:USDC-USD` and `CS:USDT-USD` in this example)
already exist in the registry. If they do not, please add them as well.

## Adding/Modifying Prefixes

- To add a new signal prefix, create a YAML file in the `./prefix/` directory that defines the processing logic for all
  signals with the corresponding prefix.

### Example: Adding a New Prefix

To add a new prefix called `CS`, create a file called `CS.yaml` in the `./prefix/` directory.
The file would look like this:

```yaml
processor:
  function: median
  params:
    min_source_count: 3
```

This prefix file defines that all signals with the prefix `CS` will be processed using the `median` function using at
least three sources.

# License

Shield: [![CC BY 4.0][cc-by-shield]][cc-by]

This work is licensed under a
[Creative Commons Attribution 4.0 International License][cc-by].

[![CC BY 4.0][cc-by-image]][cc-by]

[cc-by]: http://creativecommons.org/licenses/by/4.0/

[cc-by-image]: https://i.creativecommons.org/l/by/4.0/88x31.png

[cc-by-shield]: https://img.shields.io/badge/License-CC%20BY%204.0-lightgrey.svg
