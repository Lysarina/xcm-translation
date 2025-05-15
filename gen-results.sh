for i in $(seq 1 20);
do
    sudo make check &> perf_results/full-c2rust-translation-redo-2-res-$i.txt
done