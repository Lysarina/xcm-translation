for i in $(seq 1 10);
do
    sudo make check &> rs-1-res-$i.txt
done