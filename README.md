# Signals Registry

This repository contains the signal_ids for BandProtocol feeds module. A `registry.json` contains the aggreagted signals
that makes it easy to start using the signals in Bothan.

Schema files containing the recommended metadata structure can be found in `*.schema.json` files located in
the [`schema/`](/schema) directory.

# Signals

The metadata in these files represents the sources and its corresponding routes used to compute the signal value. For
more details on the structure of the `signal.yaml` and its fields, please refer to the schema file located in
[`schema/signal.schema.json`](/schema/signal.schema.json).

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

The metadata in these files represents the processor used for all signal with the corresponding prefix. For more details
on the structure of the `prefix.yaml` and its fields, please refer to the schema file located in
[`schema/prefix.schema.json`](/schema/prefix.schema.json).

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

# License
Shield: [![CC BY 4.0][cc-by-shield]][cc-by]

This work is licensed under a
[Creative Commons Attribution 4.0 International License][cc-by].

[![CC BY 4.0][cc-by-image]][cc-by]

[cc-by]: http://creativecommons.org/licenses/by/4.0/
[cc-by-image]: https://i.creativecommons.org/l/by/4.0/88x31.png
[cc-by-shield]: https://img.shields.io/badge/License-CC%20BY%204.0-lightgrey.svg
