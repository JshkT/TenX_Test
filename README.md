# TenX Technical Exercise

This repo contains my solution to "The Exchange Rate Path Problem", a technical exercise provided by TenX.

The task was to take price updates and exchange rate requests as inputs and return the best possible exchange rate and the path taken to achieve this.

I chose to implement my solution using Rust.

## Usage
Enter the following commands in Terminal to run the program.
```sh
$ cd TenX_Test
$ cargo run --package TenX_Test --bin TenX_Test
```

Price Updates should be of the form:
```
<timestamp> <exchange> <source_currency> <destination_currency> <forward_factor> <backward_factor>
```
and Exchange Rate Requests should be written like so:
```
EXCHANGE_RATE_REQUEST <source_exchange> <source_currency> <destination_exchange> <destination_currency>
```

Here are two example price updates:
```
2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009
2017-11-01T09:43:23+00:00 GDAX BTC USD 1001.0 0.0008
```
and here's an example exchange rate request:
```
EXCHANGE_RATE_REQUEST KRAKEN BTC GDAX USD
```
