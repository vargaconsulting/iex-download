/*
 * ALL RIGHTS RESERVED.
 * ___________________________________________________________________________________
 * NOTICE: All information contained herein is, and remains the property of Varga LABS
 * and its suppliers, if any. The intellectual and technical concepts contained herein 
 * are  proprietary to Varga LABS and its suppliers and may be covered by Canadian and 
 * Foreign Patents, patents in process, and are protected by trade secret or copyright 
 * law. Dissemination of this information or reproduction of this material is strictly
 * forbidden unless prior written permission is obtained from Varga LABS.
 *
 * Copyright © 2017-2025 Varga LABS, Toronto, On                    info@vargalabs.com
 * ___________________________________________________________________________________
 */

use std::time::Duration;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};
use clap::{Arg, Command};
use reqwest::blocking::Client;
use serde::Deserialize;
use chrono::{NaiveDate, Utc};
use indicatif::{ProgressBar, ProgressStyle};
use serde::de::{self, Visitor};
use serde::Deserializer;
use std::fmt;

pub const HIST_URL: &str = "https://iextrading.com/api/1.0/hist";

#[derive(Debug, Deserialize)]
struct HistEntry {
    link: String,
    feed: String,
    date: String,
    version: String,
    #[serde(deserialize_with = "to_u64")]
    size: u64,
}

fn to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error> where D: Deserializer<'de> {
    struct U64Visitor;

    impl<'de> Visitor<'de> for U64Visitor {
        type Value = u64;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("an integer or a string containing an integer")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: de::Error {
            v.parse::<u64>().map_err(E::custom)
        }
    }

    deserializer.deserialize_any(U64Visitor)
}

fn download(i: usize, entry: &HistEntry, dir: &PathBuf, client: &Client, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = client.get(&entry.link).send()?;
    let total_size = if dry_run { entry.size } else {
        client.get(&entry.link).send()?.content_length().unwrap_or(entry.size)
    };
    let pb = ProgressBar::new(total_size);
    let date = NaiveDate::parse_from_str(&entry.date, "%Y%m%d").unwrap().format("%Y-%m-%d").to_string();
    let path = dir.join(format!("{}-{}.pcap.gz", entry.feed, date));
    pb.set_style(ProgressStyle::default_bar().template("{msg} {bar:40.white/orange} {percent}% | ETA: {eta} | {bytes}/{total_bytes}")
        .unwrap().progress_chars("■□"));
    pb.set_message(
        format!("{i:>5}  {feed:<4} v{ver:<3} {date} ", i = i, feed = entry.feed, ver = entry.version, date = date));
    
    if dry_run {
        pb.set_position(0);
        pb.abandon();
        return Ok(());
    }

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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let about = r#"
IEX-DOWNLOAD  is a web  scraping  utility to retrieve  datasets  from
IEX. The  datasets are gzip-compressed packet capture (pcap) files of
Ethernet frames, which can be further processed using the H5CPP-based
`iex2h5` conversion  utility to  transform them into the HDF5 format.
After running the script, the  `download` directory will be populated
with TOPS or DEEP  gzip-compressed datasets,  named  according to the
corresponding trading day.  For details  on  processing the data, see
`iex2h5`. 

The data is provided free of charge by IEX. By accessing or using IEX
Historical  Data, you  agree  to their  Terms of Use. For more infor-
mation, visit: https://iextrading.com/iex-historical-data-terms/
"#;

    let matches = Command::new(clap::crate_name!())
        .version(clap::crate_version!()).about(about).author(clap::crate_authors!("\n"))
        .arg(Arg::new("deep").long("deep").action(clap::ArgAction::SetTrue).help("Download IEX DEEP datasets"))
        .arg(Arg::new("tops").long("tops").action(clap::ArgAction::SetTrue).help("Download IEX TOPS datasets"))
        .arg(Arg::new("dpls").long("dpls").action(clap::ArgAction::SetTrue).help("Download IEX DPLS datasets"))
        .arg(Arg::new("directory").long("directory").default_value("./").help("Location to save downloaded files"))
        .arg(Arg::new("progress-stall-timeout").long("progress-stall-timeout").default_value("30").help("Timeout in seconds for stalled downloads"))
        .arg(Arg::new("max-retry").long("max-retry").default_value("5").help("Max retry attempts per file"))
        .arg(Arg::new("from").long("from").help("First trading day to download (YYYY-MM-DD)"))
        .arg(Arg::new("to").long("to").help("Last trading day to download (YYYY-MM-DD)"))
        .arg(Arg::new("dry_run").long("dry-run").action(clap::ArgAction::SetTrue).help("Skips downloading"))
        .get_matches();
    let today = Utc::now().date_naive();
    let deep = matches.get_flag("deep");
    let tops = matches.get_flag("tops");
    let dpls = matches.get_flag("dpls");
    let directory = PathBuf::from(matches.get_one::<String>("directory").unwrap());
    let from = matches.get_one::<String>("from").map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d")).transpose()?.unwrap_or_else(|| today);
    let to = matches.get_one::<String>("to").map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d")).transpose()?.unwrap_or_else(|| today);
    let dry_run = matches.get_flag("dry_run");
    let client = Client::builder().timeout(Duration::from_secs(60)).build()?;
    
    let resp: BTreeMap<String, Vec<HistEntry>> = client.get(HIST_URL).send()?.json()?;

    for (i, (date, entries)) in resp.iter().enumerate() {
        let entry_date = NaiveDate::parse_from_str(&date, "%Y%m%d")?;
        if entry_date < from || entry_date > to {
            continue;
        }
        for entry in entries {
            let is_deep = entry.feed == "DEEP";
            let is_tops = entry.feed == "TOPS";
            let is_dpls = entry.feed == "DPLS";
            
            if (deep && is_deep) || (dpls && is_dpls) || (tops && is_tops) {
                download(i, entry, &directory, &client, dry_run)?;
            }
        }
    }
    Ok(())
}

