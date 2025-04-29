## How to run

gen-results.sh runs sudo make check an amount of times and saves the output of each run to a file. Rename the output file in gen-results.sh for each new version.

perf.py reads the output files and performs some statistics; it finds the tests where the mean execution time is statistically significantly different between versions, and plots the confidence intervals of these. For each new version,add the version name, and number of files in the arrays at top.

## Versions

OBS: rs-1 was run in chunks rather than one continuous test.

- original-c: original c code
- rs-1: attr_path.c has been translated to Rust
- rs-2: attr_path, xcm_dns and xcm_addr have been translated
- rs-3: rs-2 + attr_node, attr_tree, log, log_attr_tree and util have been translated
- rs-4: rs-3 but util is still in C
- rs-5: rs-3 + xcm_tp, ctl, common_ctl, xpoll, active_fd have been translated (the four last are needed by xcm_tp). CRASHES
- rs-6: rs-3 + ctl, common_ctl, xpoll, active_fd have been translated (i.e. rs-5 without xcm_tp)
- rs-7: rs-5 but no longer uses raw pointers => does not crash
- rs-8: rs-7 + xcm & common_tp have been translated
- full-c2rust-translation: done with files