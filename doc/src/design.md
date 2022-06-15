# App Design

This is an app for cyclists in NYC. It will aggregate GIS data sources to
provide useful information, like ideal routing, avoidance or warnings of
dangerous intersections, and locations of bike locking locations.

The possible feature set of this app if imagined as the "waze of cycling" is
extremely large and not at all feasible as a personal project. As always, the
first important thing to do is to dial in the scope.

## Features

Initial MVP features will include:

- view a heatmap for dangerous intersections
- trigger phone vibration in the background whenever entering dangerous
  intersections by bike
- view accurate locations of all bike lockup locations in NYC using [this
  dataset](https://www1.nyc.gov/html/dot/downloads/pdf/bike-parking-data.pdf)

## Technical Challenges

Anticipating problems and taking notes on solutions.

### Geospatial Programming in Rust

- [geospatial rust tools overview](https://pka.github.io/rust-for-geo/#/geospatial-rust)

### Sourcing and Rendering Map Data

Not sure where is the right place to start with this but OpenStreetMap seems
like an ideal data source for prototyping.

There are also datasets of geospatial data for NYC that should be useful, like:

- [all bike parking spots](https://www1.nyc.gov/html/dot/html/bicyclists/cityrack-suggest.shtml)
- [bike crash data (NYC)](https://www1.nyc.gov/html/dot/html/bicyclists/bikestats.shtml#crashdata)
