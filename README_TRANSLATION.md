# XCM Translation

Translation of the [XCM](https://github.com/Ericsson/xcm) project. Based on commit ec9639d.

## How to run

Select which version to run. Rename `Makefile_c.am` or `Makefile_rust.am` to `Makefile.am` to run C or Rust, respectively. 

Run

```
autoreconf -i && ./configure
```
If running Rust, run
```
bash cargo_build_all.sh
```

to rebuild the Rust projects.

To generate new data, run
```
sudo bash gen-results.sh
```
to run sudo make check an amount of times. The output of each run is saved to a txt file. Rename the output file in gen-results.sh for each new version. The format is perf_results/\<version-name\>-res-$i.txt

For analysing the results, run
```
python3 perf.py
```

It reads the output files and performs some statistics, and plots the name. For each new version, add the version name, and number of files in the arrays at top.