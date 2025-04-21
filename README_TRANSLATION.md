## How to run

gen-results.sh runs sudo make check an amount of times and saves the output of each run to a file. Rename the output file in gen-results.sh for each new version.

perf.py reads the output files and performs some statistics; it finds the tests where the mean execution time is statistically significantly different between versions, and plots the confidence intervals of these. For each new version,add the version name, and number of files in the arrays at top.

## Versions

OBS: gen-results.sh needs to be rerun for these versions, as it was run in chunks rather than one continuous test.

- c-code: original c code
- rs-1: attr_path.c has been translated to Rust

