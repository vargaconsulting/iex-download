
[![CI](https://github.com/vargaconsulting/iex-download/actions/workflows/ci.yml/badge.svg)](https://github.com/vargaconsulting/iex-download/actions/workflows/ci.yml)
[![MIT License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.17188420.svg)](https://doi.org/10.5281/zenodo.17188420)
[![GitHub release](https://img.shields.io/github/v/release/vargaconsulting/iex-download.svg)](https://github.com/vargaconsulting/iex-download/releases)
[![Documentation](https://img.shields.io/badge/docs-stable-blue)](https://vargaconsulting.github.io/iex-download)


# IEX High Frequency Dataset

The Investors Exchange (IEX) provides free access to historical datasets such as **Top of Book (TOPS)** and **Depth of Book (DEEP)** through its web interface.  
Unfortunately, downloading these files manually requires clicking through each link — impractical for large-scale research or backtesting.  

This project provides `iex-download`, a **Rust-based automation tool** for fetching these datasets programmatically.

### Why Bother?

Because it lets you grab over **13 TB of IEX tick data** in one shot as of 2025-09-24. Wait, wasn’t it [6 TB last week][101]? Exactly. Trading data is like an iceberg: TOPS shows you the shiny tip (best bid/ask and last trade), while the real bulk is hidden underneath in DEEP and DEEP+. That’s where the weight lives — and where the fun begins.

<table><tr><td>
Here’s the lay of the land:

| Feed   | Files to Download | Total Size (≈ GB) |
|--------|------------------:|------------------:|
| **TOPS**  | 2,285 | 5,947.68 |
| **DEEP**  | 2,115 | 5,955.02 |
| **DEEP+** |   197 | 1,353.52 |
| **TOTAL** | 4,597 | 13,256.22 |


</td><td>

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/assets/screenshot-dark.png" width="500">
  <source media="(prefers-color-scheme: light)" srcset="docs/assets/screenshot-light.png" width="500">
  <img alt="Demo screenshot" src="docs/assets/screenshot-light.png" width="500">
</picture>

</td></tr></table>

### Key Differences (TOPS vs DEEP vs DEEP+)

| Feature                                     | TOPS (Top-of-book)                                                | DEEP (Aggregated)                                                                 | DEEP+ (Order-by-order)                                                                                |
| ------------------------------------------- | ----------------------------------------------------------------- | --------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| **Order granularity**                       | Only best bid/ask + last trade                                    | Aggregated by price level (size summed)                                           | Individual orders (each displayed order)                                                              |
| **OrderID / update per order**              | Not present                                                       | Not present                                                                       | Present                                                                                               |
| **Hidden / non-display / reserve portions** | Not shown                                                         | Not shown                                                                         | Not shown                                                                                             |
| **Memory / bandwidth load**                 | Lowest (very compact, minimal updates)                            | Lower (fewer messages, coarser updates)                                           | Higher (tracking many individual orders, cancels, modifications)                                      |
| **Use-cases**                               | Quote feeds, NBBO tracking, top-level liquidity, lightweight apps | General depth, price level elasticity, coarse modelling, liquidity at price tiers | Detailed book shape, order flow-level strategy, detailed execution modelling, microstructure research |


## Features at a Glance

- **Progress bar with attitude** → because watching terabytes flow should feel satisfying.  
- **PEG-based date parser** → type `2025-01-??` or `2025-01-02,2025-01-03,2025-01-06,2025-01-2?` and it just works, no regex headaches.  
- **One tiny ELF** → a single 3.5 MB executable (`-rwxrwxr-x 2 steven steven 3.5M Sep 23 11:00 target/release/iex-download`).  
  No Python venvs, no dependency jungles. Drop it anywhere, `chmod +x`, and let it rip.  
- Need details? Just ask my imaginary friend, Manual. He’s got you covered. `man iex-download` of `iex-download --help`

## Prerequisites

- A recent Rust toolchain (e.g. via [rustup](https://rustup.rs/))  

Clone the repository:

```bash
git clone git@github.com:vargaconsulting/iex-download.git
cd iex-download
````

Build and run:

```bash
make && make install
```

[100]: https://en.wikipedia.org/wiki/Web_scraping
[101]: https://iextrading.com/trading/market-data/
[102]: https://steven-varga.ca/site/iex2h5/
[103]: https://steven-varga.ca/iex2h5/

