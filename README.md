
# IEX High Frequency Dataset
<p align='justify'>
IEX exchange provides [free access to its historical data-sets][101] such as top of book: TOPS and depth of book: DEEP  through web interface. Less fortunate is the requirement of clicking on the links to start downloading the files. This inconvenience is mitigated with `iex-download` written in nodejs, based on headless chrome browser, and a few additional packages.
</p><p align='justify'>
TOPS datasets are top of the book of aggregated `ask` and `bid` requests, and reported trade events for each transaction. Similarly DEEP contains all reported `trade`-s in addition to available quantities for different price levels for `ask` and `bid` sides. For details see: [IEX2H5](conversion.md).
</p>
<p align='justify'>
**NOTE**: This dataset simulates direct access / co-located data collection framework, but is significantly sparser then an aggregated national level II. dataset.
DEEP and TOPS are roughly 700GB each, totaling to **1.4TB** in gzip compressed format. Uncompressed is roughly **6TB** data. This real world dataset
is used to demonstrate the fitness of H5CLUSTER storage cabability, as well as  H5CPP seamless compiler assisted persistence framework.
</p>


## Features
* robust [web scraping with][100] headless chromium and node js
* date based range selection of downloadable files
* choice of TOPS and DEEP datasets
* dry run for testing/querying 
* visual feedback of download
* ability to specify download directory

## Prerequisites  
* a recent version of `node js` installed
```bash
apt get install npm
npm i puppeteer url fs filesize-parser filesize cli-progress sprintf-js dateformat yargs 
```

## Usage
```bash
IEX-DOWNLOAD  is a puppeteer based web scraping utility for IEX datasets, (pcap) files  of ethernet  frames
which may be further processed with H5CPP based `iex2h5` conversion utility into HDF5 format. After execut-
ing the script the `download` directory is populated with TOPS or DEEP gzip compressed  datasets with names 
representing the given trading day. See `iex2h5` for further details

Data provided for  free  by IEX. By accessing or using IEX Historical Data, you agree to the IEX Historical 
Data Terms of Use. See: https://iextrading.com/iex-historical-data-terms/

INSTALL:
   npm i puppeteer url fs filesize-parser filesize cli-progress sprintf-js dateformat yargs

Options:
  --version    Show version number                                     
  --deep       download IEX DEEP datasets
  --tops       Download IEX TOPS datasets
  --directory  location to save downloaded files
                                         [default: "current working directory"]
  --from       date you start downloading from           [default: "first day"]
  --to         last day                                      [default: "today"]
  --dry-run    skips downloading
  --help       Show help                                               

                                 Copyright © <2019> Varga Consulting, Toronto, ON, info@vargaconsulting.ca
```

## DEMO
<asciinema-player src="../cast/iex.cast" 
	cols=180 rows=40 autoplay=true speed=4 idle-time-limit=.1
	font-size=small theme=solarized-light></asciinema-player>
<script src="../js/asciinema-player.js"></script>



[100]: https://en.wikipedia.org/wiki/Web_scraping
[101]: https://iextrading.com/trading/market-data/
