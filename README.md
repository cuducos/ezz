# `ezz` [![Tests](https://github.com/cuducos/ezz/actions/workflows/tests.yml/badge.svg)](https://github.com/cuducos/ezz/actions/workflows/tests.yml)

`ezz` (cheesy abbreviation for _easy Zoom_) is a simple CLI tool to schedule [Zoom](https://zoom.us) meetings.

## Install

You can download the binary for your platform from the [releases page](https://github.com/cuducos/ezz/releases), for example:

```console
$ curl -LO https://github.com/cuducos/ezz/releases/download/v0.1.0/ezz-x86_64-unknown-linux-gnu.tar.gz
$ tar -xzvf ezz-x86_64-unknown-linux-gnu.tar.gz
$ rm ezz-x86_64-unknown-linux-gnu.tar.gz
$ chmod a+x ezz
$ mv ezz /usr/local/bin/
```

### Compile from source

With [`cargo`](https://www.rust-lang.org/) installed:

```console
$ cargo install --path .
```

## Authentication

`ezz` requires three environment variables to authenticate using your Zoom account:

* `ZOOM_ACCOUNT_ID`
* `ZOOM_CLIENT_ID`
* `ZOOM_CLIENT_SECRET`

You can obtain yours in [Zoom's marketplace](https://marketplace.zoom.us/), under _Develop_.

## Usage

See all the options:

```console
$ ezz --help
ezz is a simple CLI tool to schedule Zoom meetings.

Usage: ezz [OPTIONS] --name <NAME> --on <DATE> --at <TIME>

Options:
  -n, --name <NAME>          Name of the meeting
  -o, --on <DATE>            Date of the meeting in YYYY-MM-DD format or one of: today, tomorrow, monday, tuesday, wednesday, thursday, friday, saturday, sunday
  -a, --at <TIME>            Time of the meeting in HH:MM format
  -d, --duration <DURATION>  Duration of the meeting in minutes [default: 60]
  -p, --password <PASSWORD>  Meeting password, max. 10 characters [default: auto-generated random password]
  -t, --timezone <TIMEZONE>  Timezone (as in https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists/#timezones) for the meeting, e.g. America/Recife [default: your account's timezone]
  -h, --help                 Print help information
```

### Example

Creating a Zoom meeting called _Aloha_ for next Friday at 5 pm:

```console
$ ezz --name Aloha --on friday --at 17:00
https://us02web.zoom.us/j/00000000000?pwd=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```
