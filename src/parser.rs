/* This file is part of the IEX2H5 project and is licensed under the MIT License.
 Copyright © 2017–2025 Varga LABS, Toronto, ON, Canada 🇨🇦 Contact: info@vargalabs.com */

use chrono::NaiveDate;
use pest::Parser;
use pest_derive::Parser;

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