#!/usr/bin/env node
/*
 *   ALL RIGHTS RESERVED.
 *   _________________________________________________________________________________
 *   NOTICE: All information contained  herein is, and remains the property  of  Varga
 *   Consulting and  its suppliers, if  any. The intellectual and  technical  concepts
 *   contained herein are proprietary to Varga Consulting and its suppliers and may be
 *   covered  by  Canadian and  Foreign Patents, patents in process, and are protected
 *   by  trade secret or copyright law. Dissemination of this information or reproduc-
 *   tion  of  this  material is strictly forbidden unless prior written permission is
 *   obtained from Varga Consulting.
 *
 *   Copyright © <2017-2022> Varga Consulting, Toronto, On     info@vargaconsulting.ca
 *   _________________________________________________________________________________
 */
//You then need to import like this import * as fetch from 'node-fetch'; – 

//import * as df from 'dateformat';
import dateFormat, { masks } from "dateformat";
import * as process from 'process';
import _yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import * as puppeteer from 'puppeteer';
import * as url from 'url';
import * as fs from 'fs';
import {filesize} from "filesize";
import filesizeParser from 'filesize-parser';
import * as progress_bar from 'cli-progress';
import  {sprintf} from 'sprintf-js';
import * as colors from 'colors';

const df = dateFormat;
const fp = filesizeParser;
const userAgent = 'Mozilla/5.0 (X11; Linux x86_64)' +
						'AppleWebKit/537.36 (KHTML, like Gecko) Chrome/64.0.3282.39 Safari/537.36';
const p_date = 0; const p_feed = 1;	const p_size = 4;
function delay(timeout) {
  return new Promise((resolve) => {
    setTimeout(resolve, timeout);
  });
}
function get_filename(url_name) {
	var iex = url.parse(url_name, true);
	var tmp = decodeURIComponent(iex.pathname).split('iex/o/');
	var filename = tmp[tmp.length - 1];
	return filename.replace(/[\/]/g,'_');
}

function summary(feed, start, stop, download_path, dry_run){
	(async () => {

		const browser = await puppeteer.launch();
		const page = await browser.newPage();
		await page.setUserAgent(userAgent);
		await page.setViewport({
			width: 1200, height: 800, deviceScaleFactor: 1});

        const client = await page.target().createCDPSession();
        await client.send('Page.setDownloadBehavior', {behavior: 'allow', downloadPath: download_path});
        await page.goto('https://iextrading.com/trading/market-data/', {waitUntil: 'load', timeout: 0});
		const td = await page.$$('tbody#hist-rows tr td');
		var total_size = 0; var total_count = 0;
		var previous_day='';
		// iterate through all table rows, row by row
		for (var i = 0; i < td.length; i+=5) { // FIXME: make this more robust by getting columns size
			const trading_day = await page.evaluate(element => element.innerHTML, td[i]);
			const today = new Date(trading_day);
			const feed_ = await page.evaluate(element => element.children[0].innerHTML, td[i+1]);
			if( feed != feed_ || today <= start || today >= stop || previous_day == trading_day ) continue; // consider only specified interval 

			var not_exists = true;
			const size = fp(await page.evaluate(element => element.innerHTML, td[i+4]));
			const link = await page.evaluate(element => element.children[0].href, td[i+1]);
			const ver  = await page.evaluate(element => element.innerHTML, td[i+2]);
			// maintain invariants
			total_size += size; total_count ++;
			// set progress bar format
			const format_str = sprintf('%3i %5s %5s %8s %14s', total_count, feed, ver, trading_day, filesize(size));
			const bar = new progress_bar.SingleBar({
				barCompleteChar: '\u25A0',
				barIncompleteChar: '\u25A1',
				format: format_str + ' {bar} {percentage}% | ETA: {eta}s | {value}/{total}'
			}, progress_bar.Presets.rect);
			// increment counters
			// download filename is encoded in the link
			var filename = download_path +'/'+ get_filename(link);
			fs.access(filename, fs.constants.F_OK, (err) => {
					not_exists = err ? true : false; });
			// do actual download with feedback
			if( !dry_run ){
				bar.start(size, 0);
				await page.evaluate(element => element.children[0].click(), td[i+1]); // do click in page context
				while(not_exists){
					fs.access(filename, fs.constants.F_OK, (err) => not_exists = err ? true : false );
					try {
						const stats = fs.statSync(filename + '.crdownload');
						bar.update(stats.size);
					}catch(e) {}
					await delay(1000);
				}
				bar.update(size);
				bar.stop();
				// once completed, rename file
				fs.renameSync(filename, download_path + '/' + feed + '-' + trading_day + '.pcap.gz');
			} else { // fake downloaded file
				bar.start(size, 0);
				bar.stop();
			}
			previous_day = trading_day;
		}
		if (dry_run){
			console.log(
				'IEX download utility, you are about to download '
				+ total_count + ' count ' + feed + ' dataset '
				+ filesize(total_size) + ' of cumulative size.');
		}
		console.log('Copyright © <2017-2022> Varga Consulting, Toronto, ON, info@vargaconsulting.ca');
	await browser.close();
	})();
}


const yargs = _yargs(hideBin(process.argv));
(async () => {
    var today = new Date();
    var yesterday = today.setDate(today.getDate() - 2);
    const argv = await yargs
    .usage(
    'IEX-DOWNLOAD is a puppeteer based web scraping utility for IEX datasets. The datasets are gzip compressed packet capure (pcap) files of ethernet frames which may be further processed with H5CPP based `iex2h5` conversion utility into HDF5 format. After exexuting the script the `download` directory is populated with TOPS or DEEP gzip compressed datasets with names representing the given trading day. See `iex2h5` for further details. \n'
    + '\nData provided for free by IEX. By accessing or using IEX Historical Data, you agree to the IEX Historical Data Terms of Use.\nSee: https://iextrading.com/iex-historical-data-terms/ \n\n'
    + 'INSTALL:\n   npm i puppeteer url fs filesize-parser filesize cli-progress sprintf-js dateformat yargs\n'
    )
        .option('deep', {
            conflicts: 'tops',
            describe: 'download IEX DEEP datasets'
        })
        .option('tops', {
            conflicts: 'deep',
            describe: 'Download IEX TOPS datasets'
        })
        .option('directory', {
            describe: 'location to save downloaded files',
            default: '/lake/iex/queue'
        })
        .option('from', {
            describe: 'date you start downloading from',
            default: df(yesterday, "isoDate")
        })
        .option('to', {
            describe: 'last day',
            default: df(new Date(),"isoDate")
        })
        .option('dry-run', {
            describe: 'skips downloading',
        })
        .help()
        .epilogue('Copyright © <2017-2022> Varga Consulting, Toronto, ON, info@vargaconsulting.ca')
        .argv

        const dataset = argv.deep ? "DEEP" : "TOPS";
        const dryrun = argv.dryRun ? true : false;
        // console.log(argv.from, argv.to)
        // execute the actual download script
        summary(dataset, new Date(argv.from), new Date(argv.to), argv.directory, dryrun);
})();
