run_benchmark() {
    board=$1
    source=$2
    benchmark_name="${@:3}"

    laze build -C $benchmark_name -b $board -s $source run &>output.txt &
    # timeout 3s laze build -C $benchmark_name -b $board -s $source run &>output.txt &
    func_pid=$!
    while true; 
    do
        tail=$(tail output.txt)
        if ticks=$(echo $tail | grep -Po "\d+(?= ticks)"); then
            echo -ne " | $ticks"
            kill -- -$func_pid # Terminate the function
            break
        elif err=$(echo $tail | grep "none of the selected packages contains these features"); then
            echo -ne " | - "
            break
        elif err=$(echo $tail | grep -Pio -B2 "Error:.*"); then
            echo -e "\nError in $benchmark_name for $source on $board:\n$err"
            break
        fi
        sleep 0.5
    done
}

set -m

if [ -z "$BOARD" ]
then
    echo "Please set the BOARD env."
    exit
fi

SOURCES="multicore-v1"
BENCHMARKS=($(find ./benchmarks/ -name "*bench_*" -type d |  grep -vE "reallocate|async|poll|fib" ))

for i in 1 3 4
do
    BENCHMARKS+=("benchmarks/bench_sched_yield -s t$i")
done

for i in none fib loop
do
    BENCHMARKS+=("benchmarks/bench_fib -s $i")
done

echo -ne "source"
for benchmark in "${BENCHMARKS[@]}"
do
    bench=$(echo $benchmark | grep -oP "(?<=bench_).*")
    echo -n "| $bench"
done
echo 

echo -n ":- "
for benchmark in "${BENCHMARKS[@]}"
do
    echo -n "| -: "
done
echo 


for source in $SOURCES
do
    echo -n $source
    for benchmark in "${BENCHMARKS[@]}"
    do
        run_benchmark $BOARD $source $benchmark
    done
    echo
done
