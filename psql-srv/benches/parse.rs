use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone};
use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};

const TIMESTAMP_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.f";
const TIMESTAMP_FORMAT_PGJDBC: &str = "%Y-%m-%d %H:%M:%S%.f %#z";

fn timestamp_text_parse_reparse(c: &mut Criterion) {
    let run_benchmark = |b: &mut Bencher| {
        b.iter(|| {
            {
                let input = "2024-02-13 18:35:23.176-08";
                let res = NaiveDateTime::parse_from_str(input, TIMESTAMP_FORMAT);
                if res.is_err() {
                    let _ = DateTime::<FixedOffset>::parse_from_str(input, TIMESTAMP_FORMAT_PGJDBC)
                        .unwrap();
                }
            };
            black_box(())
        })
    };

    c.benchmark_group("timestamp_text_reparse")
        .bench_function("reparse", run_benchmark);
}

fn timestamp_text_parse_offsets(c: &mut Criterion) {
    let run_benchmark = |b: &mut Bencher| {
        b.iter(|| {
            {
                let input = "2024-02-13 18:35:23.176-08";
                let (datetime, timezone_tag) =
                    NaiveDateTime::parse_and_remainder(input, TIMESTAMP_FORMAT).unwrap();

                let mut pgjdbc_tz_tag = timezone_tag.to_owned();
                pgjdbc_tz_tag.push_str(String::from("00").as_str());
                let offset = FixedOffset::from_str(&pgjdbc_tz_tag).unwrap();
                let _ = offset.from_utc_datetime(&datetime);
            };
            black_box(())
        })
    };

    c.benchmark_group("timestamp_text_parse")
        .bench_function("offsets", run_benchmark);
}

criterion_group!(
    benches,
    timestamp_text_parse_offsets,
    timestamp_text_parse_reparse
);
criterion_main!(benches);
