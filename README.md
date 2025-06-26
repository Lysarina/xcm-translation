# XCM Translation

Translation of the [XCM](https://github.com/Ericsson/xcm) project to Rust. Based on commit ec9639d.

## How to run

Some package installations are needed; see the original project. For the first time, run
```
autoreconf -i && ./configure
```

Thereafter, select which variant to run. Execute
- `setup_c.bash` for Original,
- `setup_minmod.bash` for MinMod, and
- `setup_c.bash` for RustLike.

If you want to switch variants, execute the corresponding setup script.

For Rust, you can build/clean/clippy the current variant with `cargo_build_all.sh`, `cargo_clean_all.sh` and `cargo_clippy_all.sh`, respectively.

To generate new data, run
```
sudo bash gen-results.sh
```
to run sudo make check an amount of iterations (default 20). The output of each run is saved to a txt file. Rename the output files in `gen-results.sh` for each new variant or run. The format is `perf_results/<variant-name>-<run-name>-res-$i.txt`. If running several runs, `run-name` needs to be changed, but both the `variant-name` and the amount of iterations needs to stay the same.

For analysing the results, run
```
python3 perf.py
```

It reads the output files and performs some statistics, and plots the results. 

For each new variant, add the variant name in `variants`, a prettified name in `prettified_variants`, the run name in a new array in `run_names`, and the number of iterations in `files`, all at the same index in their respective arrays. If you have several runs with the same variant, add all names into the array in `run_names`.