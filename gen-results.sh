for i in $(seq 1 20); # change nbr of iterations
do
    sudo make check &> perf_results/variant-runname-res-$i.txt
done