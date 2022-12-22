# `ezz` [![Tests](https://github.com/cuducos/ezz/actions/workflows/tests.yml/badge.svg)](https://github.com/cuducos/ezz/actions/workflows/tests.yml)

`ezz` (cheesy abbreviation for _easy Zoom_) is a simple CLI tool to schedule meetings on [Zoom](https://zoom.us):

```
$ cargo run -- --help
ezz is a simple CLI tool to schedule meetings on Zoom.

Usage: ezz [OPTIONS] --name <NAME> --password <PASSWORD> --date <DATE> --time <TIME>

Options:
  -n, --name <NAME>          Name of the meeting
  -p, --password <PASSWORD>  Meeting password
  -t, --timezone <TIMEZONE>  Timezone (as in the TZ database) for the meeting, e.g. America/Recife (see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones) [default: Etc/UTC]
  -d, --duration <DURATION>  Duration of the meeting in minutes [default: 60]
  -w, --date <DATE>          Date of the meeting in YYYY-MM-DD format or one of: today, tomorrow, monday, tuesday, wednesday, thursday, friday, saturday, sunday
  -a, --time <TIME>          Time of the meeting in HH:MM format
  -h, --help                 Print help information
```
