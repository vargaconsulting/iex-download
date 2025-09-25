---
hide:
  - navigation
  - toc
---

The Investors Exchange (IEX) provides free access to historical datasets such as **Top of Book (TOPS)** and **Depth of Book (DEEP)** through its web interface.  Unfortunately, downloading these files manually requires clicking through each link — impractical for large-scale research or backtesting. This project provides `iex-download`, a **Rust-based automation tool** for fetching these datasets programmatically.

## :material-arrow-expand-horizontal:{.icon} Key Differences (TOPS vs DEEP vs DEEP+)

| Feature                                     | TOPS (Top-of-book)                                                | DEEP (Aggregated)                                                                 | DEEP+ (Order-by-order)                                                                                |
| ------------------------------------------- | ----------------------------------------------------------------- | --------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| **Order granularity**                       | Only best bid/ask + last trade                                    | Aggregated by price level (size summed)                                           | Individual orders (each displayed order)                                                              |
| **OrderID / update per order**              | Not present                                                       | Not present                                                                       | Present                                                                                               |
| **Hidden / non-display / reserve portions** | Not shown                                                         | Not shown                                                                         | Not shown                                                                                             |
| **Memory / bandwidth load**                 | Lowest (very compact, minimal updates)                            | Lower (fewer messages, coarser updates)                                           | Higher (tracking many individual orders, cancels, modifications)                                      |
| **Use-cases**                               | Quote feeds, NBBO tracking, top-level liquidity, lightweight apps | General depth, price level elasticity, coarse modelling, liquidity at price tiers | Detailed book shape, order flow-level strategy, detailed execution modelling, microstructure research |


## :material-progress-question:{.icon} Features at a Glance

- **Progress bar with attitude** → because watching terabytes flow should feel satisfying.  
- **PEG-based date parser** → type `2025-01-??` or `2025-01-02,2025-01-03,2025-01-06,2025-01-2?` and it just works, no regex headaches.  
- **One tiny ELF** → a single 3.5 MB executable (`-rwxrwxr-x 2 steven steven 3.5M Sep 23 11:00 target/release/iex-download`).  
  No Python venvs, no dependency jungles. Drop it anywhere, `chmod +x`, and let it rip.  
- Need details? Just ask my imaginary friend, Manual. He’s got you covered. `man iex-download` of `iex-download --help`

## :octicons-workflow-24:{.icon} Workflow

```bash
# Download a batch of gzip-compressed PCAP files
iex-download --tops --directory ./data 2016-12-01..2016-12-31  

# Convert the IRTS stream into an HDF5 container
iex2h5 -c irts -o iex-archive.h5 ./data/*.pcap.gz

# Convert IRTS streams into daily price matrices sampled every 10 seconds for the month of September 2025
iex2h5 -c rts --time-interval 00:00:10 --date-range 2025-09-01:2025-09-30 -o experiment-001.h5 iex-archive.h5
```

**Tip:** Don’t try to pull the entire dataset in one go (we’re talking 13+ TB and counting). Instead, download in manageable batches and incrementally add the IrRegular Time Series (IRTS) stream into your HDF5 container. Once caught up to the current trading day, you only need to maintain it with daily updates. That way the commands are tight, the warning about data size is still there, but placed below as advice.  

## :material-clock-fast:{.icon} Demo
<div id="asciicast-iex-download-demo" class="asciicast-player [&_.ap-terminal]:text-[0.55rem] w-full">
</div>
<script>
  document.addEventListener("DOMContentLoaded", function () {
    function waitForAsciinemaPlayer(attempts = 10) {
      if (typeof AsciinemaPlayer !== "undefined") {
        AsciinemaPlayer.create('casts/iex-download-demo.cast', document.getElementById('asciicast-iex-download-demo'), {
          cols: 148, rows: 39, autoPlay: true,  loop: true, speed: 1.0,  idleTimeLimit: .3, controls: true
        });
      } else if (attempts > 0) {
        setTimeout(() => waitForAsciinemaPlayer(attempts - 1), 200);
      } else {
        console.error("AsciinemaPlayer failed to load.");
      }
    }
    waitForAsciinemaPlayer();
});
</script>

## :fontawesome-solid-terminal:{.icon} Related Projects

* **[IEX2H5][203]** — A blazing-fast tick data archival and analytics system for IEX market data.
* **[H5CPP][204]** — A reflection-powered C++17/23 library for working with HDF5 in scientific computing and trading.

### Notice:
“[Data provided][100] for free by IEX. By accessing or using IEX Historical Data, you agree to the [IEX Historical Data Terms of Use][101].”


[email]: mailto:steven@vargaconsulting.ca
[whatsup]: https://wa.me/16475611829
[twitter]: https://x.com/vargaconsulting
[linkedin]: https://www.linkedin.com/in/steven-varga-04224a19/
[discord]: https://discord.gg/rZHgg2HpsD

[100]: https://iextrading.com/trading/market-data/
[101]: https://www.iexexchange.io/legal/hist-data-terms
[201]: https://steven-varga.ca/blog/longest-active-stocks-from-iex-pcap/
[202]: https://steven-varga.ca/site/iex2h5/
[203]: https://steven-varga.ca/iex2h5/
[204]: https://steven-varga.ca/site/h5cpp/

