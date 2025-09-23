// This file is part of the IEX2H5 project and is licensed under the MIT License.
//
// Copyright © 2017–2025 Varga LABS, Toronto, ON, Canada 🇨🇦
// Contact: info@vargalabs.com 

use std::time::Duration;
use std::path::PathBuf;
use reqwest::blocking::Client;
use serde::Deserialize;
use chrono::NaiveDate;
use serde::de::{self, Visitor};
use serde::Deserializer;
use std::fmt;
use pest::Parser;
use pest_derive::Parser;

pub const HIST_URL: &str = "https://iextrading.com/api/1.0/hist";

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DateSpecParser;

#[derive(Debug)]
pub enum DateSpec {
    Range(String, String),
    Sequence(Vec<String>),
    Single(String),
}
fn parse_date(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
        .or_else(|| NaiveDate::parse_from_str(s, "%Y%m%d").ok())
}
fn expand_pattern(pattern: &str) -> Vec<NaiveDate> {
    let mut results = vec![pattern.to_string()];

    for i in 0..pattern.len() {
        if &pattern[i..i+1] == "?" {
            let mut next = Vec::new();
            for s in &results {
                for d in '0'..='9' {
                    let mut chars: Vec<char> = s.chars().collect();
                    chars[i] = d;
                    next.push(chars.iter().collect::<String>());
                }
            }
            results = next;
        }
    }

    results.into_iter().filter_map(|s| parse_date(&s)).collect()
}


pub fn expand_datespec(ds: DateSpec) -> Vec<NaiveDate> {
    match ds {
        DateSpec::Single(s) => expand_pattern(&s),

        DateSpec::Sequence(v) => v.into_iter()
            .flat_map(|s| expand_pattern(&s))
            .collect(),

        DateSpec::Range(s1, s2) => {
            let start_candidates = expand_pattern(&s1);
            let end_candidates   = expand_pattern(&s2);

            if start_candidates.is_empty() || end_candidates.is_empty() {
                return Vec::new();
            }

            let start = *start_candidates.iter().min().unwrap();
            let end   = *end_candidates.iter().max().unwrap();

            let mut dates = Vec::new();
            let mut cur = start;
            while cur <= end {
                dates.push(cur);
                cur = cur.succ_opt().unwrap();
            }
            dates
        }
    }
}

pub fn parse_datespec(input: &str) -> Result<DateSpec, Box<dyn std::error::Error>> {
    let mut pairs = DateSpecParser::parse(Rule::spec, input)?;
    let pair = pairs.next().unwrap();
    let pair = if pair.as_rule() == Rule::spec {
        pair.into_inner().next().unwrap()
    } else { pair };

    match pair.as_rule() {
        Rule::date => Ok(DateSpec::Single(pair.as_str().to_string())),
        Rule::range => {
            let mut inner = pair.into_inner();
            let d1 = inner.next().unwrap().as_str().to_string();
            let d2 = inner.next().unwrap().as_str().to_string(); 
            Ok(DateSpec::Range(d1, d2))
        }
        Rule::sequence => {
            let dates = pair.into_inner()
                .map(|p| p.as_str().to_string())
                .collect();
            Ok(DateSpec::Sequence(dates))
        }
        _ => unreachable!(),
    }
}


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
    use std::io::{Read, Write};
    use std::fs::File;
    use indicatif::{ProgressBar, ProgressStyle};
    let mut resp = client.get(&entry.link).send()?;
    let total_size = if dry_run { entry.size } else {
        client.get(&entry.link).send()?.content_length().unwrap_or(entry.size)
    };
    let pb = ProgressBar::new(total_size);
    let date = NaiveDate::parse_from_str(&entry.date, "%Y%m%d").unwrap().format("%Y-%m-%d").to_string();
    let path = dir.join(format!("{}-{}.pcap.gz", entry.feed, date));
    pb.set_style(ProgressStyle::default_bar().template("{msg} {bar:40.white/orange} {bytes_per_sec} | ETA: {eta} | {bytes}/{total_bytes}")
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
    use clap::{Command, Arg, arg, ArgAction::SetTrue, value_parser};
    use std::collections::BTreeMap;
    use std::collections::HashSet;

    let prog = Command::new(clap::crate_name!())
        .version(clap::crate_version!()).author(clap::crate_authors!("\n"))
    .args([
        Arg::new("dry-run").long("dry-run").action(SetTrue),
        arg!(--tops "Enable TOPS").action(SetTrue),
        arg!(--deep "Enable DEEP").action(SetTrue),
        arg!(--dpls "Enable DPLS").action(SetTrue),
        arg!(--directory <DIR> "Output directory").default_value("./"),
        arg!(--progress-stall-timeout <SECS>).default_value("30").value_parser(value_parser!(u64)),
        Arg::new("").num_args(0..).trailing_var_arg(true)
    ]).override_help(r#"
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

Options:

--tops --deep --dpls Selects dataset type, probably you need `tops` only
--dry-run        Skips downloading but prints out  what would take place
--directory      Location to save the downloaded files
--from           First trading day to download (YYYY-MM-DD)
--to             Last trading day to download (YYYY-MM-DD)

-h, --help       Display this message
-v, --version    Display version info

example:
    iex-download --tops --from 2016-01-01 --to 2025-01-01 --directory /tmp

Copyright © 2017–2025 Varga LABS, Toronto, ON, Canada info@vargalabs.com
"#).trailing_var_arg(true).get_matches();

    let dry_run = prog.get_flag("dry-run");
    let deep = prog.get_flag("deep");
    let tops = prog.get_flag("tops");
    let dpls = prog.get_flag("dpls");
    let directory = PathBuf::from(prog.get_one::<String>("directory").unwrap());
    let rest: Vec<String> = prog.get_many::<String>("").unwrap_or_default().cloned().collect();
    let specs = rest.join(" ");
    let parsed = parse_datespec(&specs)?;
    let expanded = expand_datespec(parsed);
    let wanted: HashSet<NaiveDate> = expanded.into_iter().collect();

    let client = Client::builder().timeout(Duration::from_secs(60)).build()?;
    let resp: BTreeMap<String, Vec<HistEntry>> = client.get(HIST_URL).send()?.json()?;

    for (i, (date, entries)) in resp.iter().enumerate() {
        let entry_date = NaiveDate::parse_from_str(&date, "%Y%m%d")?;
        if !wanted.contains(&entry_date) {
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

