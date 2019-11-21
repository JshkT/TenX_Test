# TenX Technical Exercise

This repo contains my solution to "The Exchange Rate Path Problem", a technical exercise provided by TenX.

The task was to take price updates and exchange rate requests as inputs and return the best possible exchange rate and the path taken to achieve this.

I chose to implement my solution using Rust.

At the core of this solution is the Floyd-Warshall algorithm used to solve the all pairs shortest path problem in O(V3) time.
The upside of this is that once the graph is created and the algorithm has done its work, when we want to lookup the best rate and return the path taken, we can do that relatively quickly. 


## Usage

You will need Rust installed to run this. You can find instructions for that at this link.

https://doc.rust-lang.org/1.30.0/book/second-edition/ch01-01-installation.html

After that, enter the following commands in Terminal to run the program.
```sh
$ cd TenX_Test-1.1
$ cargo build
$ cargo run
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
If all goes well, you can expect the output in the form:
```
BEST_RATES_BEGIN <source_exchange> <source_currency> <destination_exchange> <destination_currency> <rate>
<source_exchange, source_currency>
<exchange, currency>
<exchange, currency>
...
<destination_exchange, destination_currency>
BEST_RATES_END
```

### TODO
- ~~Plenty of room to refactor and clean up~~
- ~~Write helper functions to help move the clutter from main.rs~~
- Write tests
- More error handling

### Limitations
Under the time constraints of a week and this being my first time using Rust, the program is less stable and neat than I would have prefered. It can still be broken by being creative with the user inputs. However, it does perform its function through my own testing.

### Thanks
Thanks for taking the time to look over my work. 
