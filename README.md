# subnudger-rs

A simple tool to offset subtitles timings by a given amount of milliseconds built in Rust.

## Building

```sh
cargo build --release
mv target/release/subnudger-rs /usr/local/bin
```

## Usage
```sh
subnudger-rs <input>.srt <output>.srt <offset>
```

## Example

Offsetting subs.srt by 500 milliseconds and saving the result to subs-offset.srt:

```sh
subnudger-rs subs.srt subs-offset.srt 500
```
