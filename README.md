# `ezz` [![Tests](https://github.com/cuducos/ezz/actions/workflows/tests.yml/badge.svg)](https://github.com/cuducos/ezz/actions/workflows/tests.yml)

`ezz` (cheesy abbreviation for _easy Zoom_) is a simple CLI tool to schedule [Zoom](https://zoom.us) meetings.

## Install

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

Usage: ezz [OPTIONS] --name <NAME> --password <PASSWORD> --date <DATE> --time <TIME>

Options:
  -n, --name <NAME>          Name of the meeting
  -p, --password <PASSWORD>  Meeting password
  -t, --timezone <TIMEZONE>  Timezone (as in the TZ database) for the meeting, e.g. America/Sao_Paulo (see https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists/#timezones) [default: UTC]
  -d, --duration <DURATION>  Duration of the meeting in minutes [default: 60]
  -w, --date <DATE>          Date of the meeting in YYYY-MM-DD format or one of: today, tomorrow, monday, tuesday, wednesday, thursday, friday, saturday, sunday
  -a, --time <TIME>          Time of the meeting in HH:MM format
  -h, --help                 Print help information
```

Note that the **default timezone is UTC** and the default duration is 1h.

### Example

Creating a Zoom meeting called _Aloha_ for next Friday, 5 pm, with password `12345678`:

```console
$ ezz --name Aloha --password 12345678 --date friday --time 17:00
https://us02web.zoom.us/j/00000000000?pwd=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```
