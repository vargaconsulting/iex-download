
# IEX High Frequency Dataset
IEX Exchange provides free access to historical datasets such as Top of Book (TOPS) and Depth of Book (DEEP) through its web interface.
Unfortunately, downloading the files requires manually clicking each link — an inconvenience for large-scale use.
This limitation is addressed by `iex-download`, a Node.js-based automation tool built with headless Chrome (Puppeteer) and supporting libraries.

**TOPS** datasets provide top-of-book aggregated `ask` and `bid` quotes, along with reported trade events for each transaction.
**DEEP** datasets extend this by including all reported trades and the available quantities at various price levels on both the ask and bid sides. In my understanding IEX DEEP does not quite qualify for a level II dataset.

## Features
* robust [web scraping with][100] headless chromium and node js
* date based range selection of downloadable files
* choice of TOPS and DEEP datasets
* dry run for testing/querying 
* visual feedback of download
* ability to specify download directory

## Prerequisites  
* a  version of `node js` installed
```bash
git clone git@github.com:vargaconsulting/iex-download.git
https://github.com/vargaconsulting/iex-download.git
apt install npm
npm ci    # install dependencier locally 
npm start # builds and outputs `iex-download-2.1.8.tgz` distribution package
npm install -g iex-download-2.1.24.tgz # install package globally on a host
```

## Usage
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

INSTALLATION:
	npm install ./iex-download-x.y.z.tgz


Options:
  --version                 Show version number  [boolean]
  --deep                    Download IEX DEEP datasets
  --tops                    Download IEX TOPS datasets
  --directory               Location to save downloaded files  [default: "./"]
  --progress_stall_timeout  Timeout duration (in seconds) to detect and handle stalled downloads  [default: 30]
  --max_retry               Number of times to retry downloading the same file before giving up  [default: 5]
  --from                    Date you start downloading from  [default: "2025-03-22"]
  --to                      Last day to download  [default: "2025-03-24"]
  --dry-run                 Skips downloading
  --help                    Show help  [boolean]

Copyright © <2017-2025> Varga Consulting, Toronto, ON. info@vargaconsulting.ca
```


[100]: https://en.wikipedia.org/wiki/Web_scraping
[101]: https://iextrading.com/trading/market-data/
