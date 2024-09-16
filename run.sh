#! /bin/bash

get_all_benchmarks() {
    mapfile -t BENCHMARKS < <(find ./benchmarks/ -name "*bench_*" -type d |  grep -vE "poll|fib" )

    for i in 1 3 4
    do
        BENCHMARKS+=("benchmarks/bench_sched_yield_t -s t$i")
    done

    for i in none fib loop
    do
        BENCHMARKS+=("benchmarks/bench_fib -s $i")
    done

    for i in none poll await
    do
        BENCHMARKS+=("benchmarks/bench_busy_poll -s $i")
    done
}

print_table_header(){
    benchmark_names=""
    table_line=""

    for benchmark in "${BENCHMARKS[@]}"
    do
        bench=$(echo $benchmark | grep -oP "(?<=bench_).*" | sed "s/_/  /g")
        benchmark_names+=" | $bench"
        table_line+=" | -:"
    done

    echo "source$benchmark_names"
    echo ":-$table_line"
}


subprocess(){
    board=$1
    source=$2
    benchmark_name="${*:3}"
    set -m
    laze build -C $benchmark_name -b $board -s $source run 2>&1 &
    echo "$!"
}

run_benchmark() {

    exec 3< <(subprocess "$@")
    read <&3 subprocess_pid;
    while true; 
    do
        read <&3 line;
        # echo $line
        if ticks=$(echo $line | grep -Po "\d+(?= ticks)"); then
            echo -ne " | $ticks"
            kill -- -$subprocess_pid # Terminate the function
            break
        elif err=$(echo $line | grep "none of the selected packages contains these features"); then
            echo -ne " | - "
            break
        elif panic=$(echo $line | grep -Pio "panic:.*"); then
            echo -ne " | <panic>"
            echo -e "\n$panic" 1>&2
            kill -- -$subprocess_pid # Terminate the function
            break
        elif err=$(echo $line | grep -Pio "Error:.*"); then
            echo -ne " | <error>"
            echo -e "\n$err" 1>&2
            break
        fi
    done
}

run(){

    if [ -z "$BOARD" ]
    then
        echo "Please set the BOARD env."
        exit
    fi

    if [ -z "$SOURCES" ]
    then
        SOURCES="main single-core dual-core"
    fi


    if [ -z "${BENCHMARKS[0]}" ]
    then
        get_all_benchmarks
    else 
        BENCHMARKS=$($BENCHMARKS)
    fi

    print_table_header

    for source in $SOURCES
    do
        echo -n $source
        for benchmark in "${BENCHMARKS[@]}"
        do
            run_benchmark $BOARD $source $benchmark
        done
        echo
    done
}

run