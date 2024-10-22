#! /bin/bash

OUT=data/$BOARD.md

get_all_benchmarks() {
    mapfile -t BENCHMARKS < <(find ./benchmarks/ -name "*bench_*" -type d |  grep -vE "yield|poll|fib|matrix|spinlocks" )
    for i in $(seq 4)
    do
        BENCHMARKS+=("benchmarks/bench_sched_yield_t -s t$i")
    done

    for i in "" "-0" "-1"  
    do
        BENCHMARKS+=("benchmarks/bench_sched_yield_t -s t3 -s affinity$i")
    done

    for i in none fib loop
    do
        BENCHMARKS+=("benchmarks/bench_fib -s $i")
    done

    for i in $(seq 10 10 40)
    do
        BENCHMARKS+=("benchmarks/bench_matrix_mult -s n$i")
    done

    for i in poll await
    do
        BENCHMARKS+=("benchmarks/bench_busy_poll -s $i")
    done

    for i in noop cs atomic atomic-rw hardware
    do
        BENCHMARKS+=("benchmarks/bench_spinlocks -s $i")
    done
}

print_table_header(){
    benchmark_names=""
    table_line=""

    for benchmark in "${BENCHMARKS[@]}"
    do
        bench=$(echo $benchmark | grep -oP "(?<=bench_).*" | sed "s/_/ /g")
        benchmark_names+=" | $bench"
        table_line+=" | -:"
    done

    echo "source$benchmark_names" > $OUT
    echo ":-$table_line" >> $OUT
}


subprocess(){
    board=$1
    source=$2
    benchmark_name="${*:3}"
    set -m
    # Build once first so that timeout doesn't cancel slow builds.
    laze build -C $benchmark_name -b $board -s $source &> /dev/null
    timeout -v 10s laze build -C $benchmark_name -b $board -s $source run 2>&1 &
    echo "$!"
}

run_benchmark() {

    exec 3< <(subprocess "$@")
    read <&3 subprocess_pid;
    while true; 
    do
        read <&3 line;
        echo "$line"

        if ticks=$(echo $line | grep -Po "\d+(?= ticks)"); then
            echo -ne " | $ticks" >> $OUT
            kill -- -$subprocess_pid # Terminate the function
            break
        elif echo $line | grep "none of the selected packages contains these features"; then
            echo -ne " | -" >> $OUT
            break
        elif echo $line | grep "no matching target for task"; then
            echo -ne " | -" >> $OUT
            break
        elif echo $line | grep "is not an ancestor of"; then
            echo -ne " | -" >> $OUT
            break
        elif bench_err=$(echo $line | grep -Po "(?<=benchmark error: )\w+"); then
            echo -ne " | benchmark $bench_err" >> $OUT
            kill -- -$subprocess_pid # Terminate the function
            break
        elif echo $line | grep -Pio "panic:.*"; then
            echo -ne " | panic" >> $OUT
            kill -- -$subprocess_pid # Terminate the function
            break
        elif echo $line | grep -Pio "Error:.*"; then
            echo -ne " | error" >> $OUT
            break
        elif echo $line | grep -Pio "timeout:"; then
            echo -ne " | timeout" >> $OUT
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
        if [ -z "$REVS" ]
        then
            REVS="main multicore-v1 multicore-v2 multicore-v2-cs multicore-v2-locking"
        fi

        if [ -z "$FEAT" ]
        then
            FEAT="single-core dual-core"
        fi

        for rev in $REVS
        do
            if [ "$rev" = "main" ];
            then

                SOURCES+=("$rev")
                continue
            fi
            for feat in $FEAT
            do

                SOURCES+=("$feat -s $rev")

            done
        done
    fi

    for source in "${SOURCES[@]}"
    do
        echo $source
    done

    if [ -z "${BENCHMARKS[0]}" ]
    then
        get_all_benchmarks
    else 
        BENCHMARKS=$($BENCHMARKS)
    fi

    print_table_header

    for source in "${SOURCES[@]}"
    do
        echo -n $source >> $OUT
        for benchmark in "${BENCHMARKS[@]}"
        do
            run_benchmark "$BOARD" "$source" "$benchmark"
        done
        echo "" >> $OUT
    done

    { date; echo ""; cat $OUT; echo ""; } >> data/archive/$BOARD.md
}

run