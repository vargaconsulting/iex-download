/* This file is part of the IEX2H5 project and is licensed under the MIT License.
 Copyright © 2017–2025 Varga LABS, Toronto, ON, Canada 🇨🇦 Contact: info@vargalabs.com */

mod utils;
mod parser;
mod io;

use std::time::Duration;
use std::path::PathBuf;
use reqwest::blocking::Client;
use chrono::NaiveDate;
use utils::to_human;
use io::{HistEntry, download};
use parser::{expand_datespec, parse_datespec};

pub const HIST_URL: &str = "https://iextrading.com/api/1.0/hist";

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
        arg!(--silent "Disable progress bar").action(SetTrue),
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

-h, --help       Display this message
-v, --version    Display version info

iex-download --tops | --deep | dpls [--directory DIR] [--dry-run] <date specification>

example:
    iex-download --tops --directory /tmp --dry-run 2024-04-01..2025-01-01
    iex-download --tops 2024-04-01:2025-01-01
    iex-download --deep 20240401..20250101
    iex-download --deep --tops 2025-01-1?,2025-01-?3
    iex-download --tops 2025-01-11,2025-01-12,2025-03-01

BNF grammar specification for dates:
    <spec>       ::= <range> | <sequence> | <date>
    <range>      ::= <date> <range-sep> <date>
    <range-sep>  ::= ".." | ":"
    <sequence>   ::= <date> { "," <date> }
    <date>       ::= <year> "-" <month> "-" <day>
                | <year> <month> <day>          ; compact form YYYYMMDD
    <year>       ::= <digit><digit><digit><digit>
    <month>      ::= <digit-or-wild><digit-or-wild>
    <day>        ::= <digit-or-wild><digit-or-wild>
    <digit>      ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
    <digit-or-wild> ::= <digit> | "?"

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

