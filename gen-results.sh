for i in $(seq 1 10);
do
    sudo make check &> perf_results/rs-2-res-$i.txt
done