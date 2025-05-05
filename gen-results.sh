for i in $(seq 1 10);
do
    sudo make check &> perf_results/full-c2rust-translation-final-res-$i.txt
done