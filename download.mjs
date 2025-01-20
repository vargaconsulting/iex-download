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

function summary(feed, start, stop, download_path, progress_stall_timeout, max_retry, dry_run){
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
			const format_str = sprintf('%3i %5s %5s %8s %14s', total_count, feed, ver, trading_day, filesize(size)); // set progress bar format
			var filename = download_path +'/'+ get_filename(link);
			fs.access(filename, fs.constants.F_OK, (err) => {
				not_exists = err ? true : false; });
				// do actual download with feedback
			if (!dry_run) for (var attempt = 0; attempt < max_retry; attempt++) {
				const bar = new progress_bar.SingleBar({
					barCompleteChar: '\u25A0',
					barIncompleteChar: '\u25A1',
					format: format_str + ' {bar} {percentage}% | ETA: {eta}s | {value}/{total}'
				}, progress_bar.Presets.rect);
				bar.start(size, 0);
				const start_time = Date.now();
				let last_progress_time = Date.now(), last_size = 0, is_timed_out = false, is_no_progress = false;
				await page.evaluate(element => element.children[0].click(), td[i + 1]); // download filename is encoded in the link
				while (true) { // progress bar update
					const elapsed_time = Date.now() - start_time;
					const since_last_progress = Date.now() - last_progress_time;
					is_no_progress = since_last_progress > progress_stall_timeout;
					if (is_timed_out || is_no_progress || fs.existsSync(filename)) break;
					try {                               // update progress bar if `.crdownload` file exists
						const temp_file = filename + '.crdownload';
						if (fs.existsSync(temp_file)) {
							const stats = fs.statSync(temp_file);
							bar.update(stats.size);
							if (stats.size > last_size) { 		// detect progress
								last_size = stats.size;
								last_progress_time = Date.now(); // reset no-progress timer
							}
						}
					} catch (e) {
						console.warn(`Error reading temporary file: ${e.message}`);
					}
					await delay(1000); // wait before checking again
				}
				bar.update(size); // ensure the bar completes
				bar.stop();
				if(is_no_progress) continue; // we try it in next iteration
				try {// rename file to final name
					if (fs.existsSync(filename)) {
						fs.renameSync(filename, download_path + '/' + feed + '-' + trading_day + '.pcap.gz');
					} else {
						console.error(`Download failed for file: ${filename}`);
						continue; // give another chance... 
					}
					break; // at this point we do have the goods, so we can break out of the loop...
				} catch (e) {
					console.error(`Error renaming file: ${e.message}`);
				}
			} else { // simulate download progress for dry run
				const bar = new progress_bar.SingleBar({
					barCompleteChar: '\u25A0',
					barIncompleteChar: '\u25A1',
					format: format_str + ' {bar} {percentage}% | ETA: {eta}s | {value}/{total}'
				}, progress_bar.Presets.rect);				
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

const usage_txt = `
IEX-DOWNLOAD is a web scraping utility built with Puppeteer to retrieve
datasets from IEX. The datasets are gzip-compressed packet capture (pcap)
files of Ethernet frames, which can be further processed using the H5CPP-
based \`iex2h5\` conversion utility to transform them into the HDF5 format.

After running the script, the \`download\` directory will be populated with
TOPS or DEEP gzip-compressed datasets, named according to the corresponding
trading day. For additional details on processing the data, see \`iex2h5\`.

The data is provided free of charge by IEX. By accessing or using IEX
Historical Data, you agree to their Terms of Use. For more information,
visit: https://iextrading.com/iex-historical-data-terms/

INSTALLATION:
   npm install puppeteer url fs filesize-parser filesize cli-progress
   sprintf-js dateformat yargs
`;

const yargs = _yargs(hideBin(process.argv));
(async () => {
    var today = new Date();
    var yesterday = today.setDate(today.getDate() - 2);
    const argv = await yargs
        .usage(usage_txt)
        .option('deep', {
            conflicts: 'tops',
            describe: 'Download IEX DEEP datasets'
        })
        .option('tops', {
            conflicts: 'deep',
            describe: 'Download IEX TOPS datasets'
        })
        .option('directory', {
            describe: 'Location to save downloaded files',
            default: './'
        })
        .option('progress_stall_timeout', {
            describe: 'Timeout duration (in seconds) to detect and handle stalled downloads',
            default: 30
        })
        .option('max_retry', {
            describe: 'Number of times to retry downloading the same file before giving up',
            default: 5
        })
        .option('from', {
            describe: 'Date you start downloading from',
            default: df(yesterday, "isoDate")
        })
        .option('to', {
            describe: 'Last day to download',
            default: df(new Date(), "isoDate")
        })
        .option('dry-run', {
            describe: 'Skips downloading',
        })
        .help()
		.wrap(null)
        .epilogue('Copyright © <2017-2022> Varga Consulting, Toronto, ON. info@vargaconsulting.ca')
        .argv;

    const dataset = argv.deep ? "DEEP" : "TOPS";
    const dryrun = argv.dryRun ? true : false;
    summary(dataset, new Date(argv.from), new Date(argv.to), argv.directory,
        argv.progress_stall_timeout * 1000, argv.max_retry, dryrun);
})();
