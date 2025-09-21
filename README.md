# IEX High Frequency Dataset

The Investors Exchange (IEX) provides free access to historical datasets such as **Top of Book (TOPS)** and **Depth of Book (DEEP)** through its web interface.  
Unfortunately, downloading these files manually requires clicking through each link — impractical for large-scale research or backtesting.  

This project provides `iex-download`, a **Rust-based automation tool** for fetching these datasets programmatically.

## Dataset Overview

- **TOPS**: top-of-book aggregated quotes (`bid`/`ask`), along with reported trade events for each transaction.  
- **DEEP**: extended order book information, including all reported trades and available quantities at multiple price levels on both bid and ask sides.  
  *Note: IEX DEEP is not a full Level II feed but provides a richer picture than TOPS alone.*

## Features

- Written in **Rust** with strong typing and reliability in mind  
- Query the official IEX historical data API  
- Date-range selection for downloading multiple days at once  
- Choice of **TOPS** or **DEEP** datasets  
- **Dry-run mode** for listing available files without downloading  
- Progress bar and retry logic for robust large-file transfers  
- Customizable download directory  

## Prerequisites

- A recent Rust toolchain (e.g. via [rustup](https://rustup.rs/))  

Clone the repository:

```bash
git clone git@github.com:vargaconsulting/iex-download.git
cd iex-download
````

Build and run:

```bash
cargo run --release -- --help
```

## Usage

The tool downloads gzip-compressed packet capture (`.pcap.gz`) files of Ethernet frames.
These can be further processed with the [`iex2h5`](https://github.com/vargaconsulting/iex2h5) utility to transform raw packet data into structured **HDF5** format for analytics.

After running `iex-download`, the chosen output directory will contain TOPS or DEEP datasets named according to their trading day.

The data is provided free of charge by IEX.
By accessing or using IEX Historical Data, you agree to their [Terms of Use](https://iextrading.com/iex-historical-data-terms/).

### Command-line Options

```bash
IEX-DOWNLOAD is a web scraping utility built with Puppeteer to retrieve
datasets from IEX. The datasets are gzip-compressed packet capture (pcap)
files of Ethernet frames, which can be further processed using the H5CPP-
based `iex2h5` conversion utility to transform them into the HDF5 format.

After running the script, the `download` directory will be populated with
TOPS or DEEP gzip-compressed datasets, named according to the corresponding
trading day. For additional details on processing the data, see `iex2h5`.

The data is provided free of charge by IEX. By accessing or using IEX
Historical Data, you agree to their Terms of Use. For more information,
visit: https://iextrading.com/iex-historical-data-terms/

Options:
  --version                 Show version number  [boolean]
  --deep                    Download IEX DEEP datasets
  --tops                    Download IEX TOPS datasets
  --directory               Location to save downloaded files  [default: "./"]
  --progress_stall_timeout  Timeout (seconds) to detect stalled downloads  [default: 30]
  --max_retry               Max retry attempts per file  [default: 5]
  --from                    First trading day to download  [default: "2025-03-22"]
  --to                      Last trading day to download   [default: "2025-03-24"]
  --dry-run                 Query API without downloading
  --help                    Show help  [boolean]

Copyright © <2017-2025> Varga Consulting, Toronto, ON. info@vargaconsulting.ca
```


[100]: https://en.wikipedia.org/wiki/Web_scraping
[101]: https://iextrading.com/trading/market-data/
