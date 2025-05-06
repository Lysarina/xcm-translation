for i in $(seq 1 20);
do
    # mv perf_results/rustlike-$i.txt perf_results/rustlike-res-$i.txt
    sudo make check &> perf_results/original-c-2-res-$i.txt
done