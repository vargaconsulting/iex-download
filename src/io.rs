/* This file is part of the IEX2H5 project and is licensed under the MIT License.
 Copyright © 2017–2025 Varga LABS, Toronto, ON, Canada 🇨🇦 Contact: info@vargalabs.com */

use std::path::PathBuf;
use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde::Deserialize;
use crate::utils::to_u64;

#[derive(Debug, Deserialize)]
pub struct HistEntry {
    pub link: String,
    pub feed: String,
    pub date: String,
    pub version: String,
    #[serde(deserialize_with = "to_u64")]
    pub size: u64,
    #[serde(default)]  
    pub trading_day: usize
}

pub fn download(entry: &HistEntry, dir: &   PathBuf, client: &Client, dry_run: bool, silent: bool) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{Read, Write};
    use std::fs::File;
    use indicatif::{ProgressBar, ProgressStyle};
    let mut resp = client.get(&entry.link).send()?;
    let total_size = if dry_run { entry.size } else {
        client.get(&entry.link).send()?.content_length().unwrap_or(entry.size)
    };

    let date = NaiveDate::parse_from_str(&entry.date, "%Y%m%d").unwrap().format("%Y-%m-%d").to_string();
    let path = dir.join(format!("{}-{}.pcap.gz", entry.feed, date));
    
    let pb = if silent { ProgressBar::hidden() } else {    
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar().template("{msg} {bar:40.white/orange} {bytes_per_sec} | ETA: {eta} | {bytes}/{total_bytes}")
            .unwrap().progress_chars("■□"));
        pb.set_message(
            format!("{i:>5}  {feed:<4} v{ver:<3} {date} ", i = entry.trading_day, feed = entry.feed, ver = entry.version, date = date));
        pb
    };
    if dry_run { // dry_run and !silent 
        pb.set_position(0);
        pb.abandon();
        return Ok(());
    }
    // `dry_run + silent == noop` so we should never have that case here, but skipped a layer up 
    let mut file = File::create(path)?;
    let mut buffer = [0u8; 8192];
    let mut downloaded: u64 = 0;

    loop {
        let n = resp.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
        downloaded += n as u64;
        pb.set_position(downloaded);
    }
    pb.finish();
    Ok(())
}