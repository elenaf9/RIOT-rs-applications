import pandas as pd
import matplotlib.pyplot as plt
import matplotlib

board = "rpi-pico"

colors = {
    "rpi-pico": {
        "single-core": ["#336971", "#95aeb2", "#f9f9f9"],
        "dual-core": ["#00535c", "#75979c", "#d7e0e1"],
    },
    "espressif-esp32-s3-wroom-1": {
        "single-core": ["#91452f", "#ca9c8e", "#f9f9f9"],
        "dual-core": ["#7b2712", "#b87e6d", "#ebd9d4"]
    }
}

map_index_sched = {
    "main": "No SMP",
    "multicore-v1": "Allocation",
    "multicore-v2": "Dynamic",
    "multicore-v2-cs": "Internal CS",
    "multicore-v2-locking": "Fine-grained",
}

map_index_locking = {
    "multicore-v2": "Original",
    "multicore-v2-cs": "Internal CS",
    "multicore-v2-locking": "Fine-grained",
}

rename_fib = {
    'fib -s none': 'Idle',
    'fib -s fib': 'Fib',
    'fib -s loop': 'Loop',
}

rename_sched = {
    'sched yield t -s t1': '1',
    'sched yield t -s t2': '2',
    'sched yield t -s t3': '3',
    'sched yield t -s t4': '4',
}

rename_affinity = {
    'sched yield t -s t3': 'disabled',
    'sched yield t -s t3 -s affinity': 'enabled',
    'sched yield t -s t3 -s affinity-0': 'Bind Core 0',
    'sched yield t -s t3 -s affinity-1': 'Bind Core 1',
}

rename_matrix_mult = {
    'matrix mult -s n10': '10',
    'matrix mult -s n20': '20',
    'matrix mult -s n30': '30',
    'matrix mult -s n40': '40',
}

rename_busy_poll = {
    'busy poll -s poll': 'Polling',
    'busy poll -s await': 'Interrupt',
}

rename_feat = {
    'single-core': 'Single-Core',
    'dual-core': 'Multi-Core'
}

rename_spinlocks = {
    'spinlocks -s noop': 'None',
    'spinlocks -s cs': 'Critical-Section',
    'spinlocks -s atomic-rw': 'Atomics RW',
    'spinlocks -s atomic': 'Atomics',
    'spinlocks -s hardware': 'Hardware',
}

rename_runqueue = {
    'runqueue add': 'Add',
    'runqueue get next': 'Next',
    'runqueue reallocate': 'Realloc',
    'runqueue pop head': 'Pop\nNext',
    'runqueue del': 'Del',
    'runqueue advance': 'Advance',
}


def load_md_table(board):
    df = pd.read_table("data/"+board+".md", engine='python',
                       sep=r" \| ", header=0, skipinitialspace=True).iloc[1:]

    # Update 'main' field to match the pattern '<feat> -n <rev>' of the other fields;
    df.loc[df['source'] == 'main', 'source'] = 'dual-core -s main'
    main2 = df.loc[df['source'] == 'dual-core -s main'].copy()
    main2['source'] = 'single-core -s main'
    df = pd.concat([main2, df], ignore_index=True)

    # Split source into rev and feat
    df[['Feat', 'Rev']] = df['source'].str.split(' -s ', n=1, expand=True)
    df = df.drop('source', axis=1)
    df.set_index(['Rev', 'Feat'], inplace=True)

    df = df.apply(pd.to_numeric, errors='coerce')
    return df


def filter_feature(df, value):
    df = df.loc[df.index.get_level_values('Feat') == value]
    return df.reset_index(level='Feat', drop=True)


def filter_revs(df, revs):
    df = df.loc[df.index.get_level_values('Rev').isin(revs)]
    if len(revs) == 1:
        df = df.reset_index(level='Rev', drop=True)
    return df


def benchmark(df, find, exclude):
    df = df.filter(regex="Rev|Feat|"+find)
    if exclude:
        df = df[df.columns[~df.columns.str.contains(exclude)]]
    return df


def bar_plot(board, df, name, feat, index):
    if index:
        df.index = df.index.map(index)
    if len(df.columns) > 1:
        df = df.transpose()
    ax = df.plot.bar(
        rot=0,
        # colormap="bone",
        color=colors[board][feat],
        edgecolor="black",
        legend=len(df.columns) > 1,
        fontsize=16

    )
    ax.xaxis.label.set_visible(False)
    ax.yaxis.label.set_visible(False)
    ax.spines.top.set_visible(False)
    ax.spines.right.set_visible(False)
    if len(df.columns) > 1:
        plt.legend(prop={'size': 14})
    plt.subplots_adjust(bottom=0.07, top=0.97, right=0.98, left=0.15)
    plt.savefig("graphs/" + name + "_" + feat + "_" + board + ".png", )
    plt.close()


def plot_one(board, df, search,
             feat="dual-core", index_map=map_index_sched, rename_revs=None, exclude=None, name=None):
    df_benchmark = benchmark(df, search, exclude).dropna(axis=1, how='all')
    if df_benchmark.empty:
        return

    if isinstance(df_benchmark.index, pd.MultiIndex) and len(df_benchmark.columns) == 1:
        df_benchmark = df_benchmark.unstack(level=1)
        df_benchmark.columns = df_benchmark.columns.droplevel(0)
        df_benchmark = df_benchmark.iloc[:, ::-1]

    if rename_revs:
        df_benchmark = df_benchmark.rename(columns=rename_revs)
    chart_name = name if name else search.replace(" ", "-")
    bar_plot(board, df_benchmark, chart_name, feat, index_map)


def plot_all(board, df):
    # == Subsets of benchmarks

    # All single-core
    df_single_core = filter_feature(df, 'single-core')
    # All dual-core
    df_dual_core = filter_feature(df, 'dual-core')
    # All dual core of multicore-v2
    df_v2_dual_core = filter_revs(df_dual_core, ['multicore-v2'])

    # Single Core scheduler-version
    df_sched_revs_single_core = filter_revs(
        df_single_core, ['main', 'multicore-v1', 'multicore-v2'])
    # Dual Core scheduler-version
    df_sched_revs_dual_core = filter_revs(
        df_dual_core, ['main', 'multicore-v1', 'multicore-v2'])
    # Like above, but without main
    df_sched_revs_dual_core_only = filter_revs(
        df_dual_core, ['multicore-v1', 'multicore-v2'])
    # Sched revs single- and dual-core combined
    df_sched_revs_both = filter_revs(
        df, ['main', 'multicore-v1', 'multicore-v2'])

    # Locking versions
    df_lock_revs = filter_revs(
        df, ['multicore-v2', 'multicore-v2-cs', 'multicore-v2-locking'])
    # Single Core locking-version
    df_lock_revs_single_core = filter_revs(
        df_single_core, ['multicore-v2', 'multicore-v2-cs', 'multicore-v2-locking'])
    # Dual Core locking-version
    df_lock_revs_dual_core = filter_revs(
        df_dual_core, ['multicore-v2', 'multicore-v2-cs', 'multicore-v2-locking'])
    # Dual Core v2-locking
    df_locking_dual_core = filter_revs(df_dual_core, ['multicore-v2-locking'])

    # == Plot benchmark data

    # Plot fibonacci benchmark
    plot_one(board, df_v2_dual_core, "fib",
             rename_revs=rename_fib)

    # RunQueue operations

    # Runqueue single-core
    plot_one(board, df_sched_revs_single_core, "runqueue",
             rename_revs=rename_runqueue, feat="single-core")
    plot_one(board, df_sched_revs_dual_core, "runqueue",
             rename_revs=rename_runqueue)

    # = Scheduler Benchmarks

    # Plot pure scheduler invocation
    plot_one(board, df_sched_revs_both, "sched", exclude="yield")

    # Plot scheduler performance for different versions on single-core
    plot_one(board, df_sched_revs_single_core, "sched yield",
             rename_revs=rename_sched, feat="single-core", exclude="affinity")
    # Plot scheduler performance for different versions on dual-core
    plot_one(board, df_sched_revs_dual_core, "sched yield",
             rename_revs=rename_sched,  exclude="affinity")
    # Plot core-affinity overhead
    plot_one(board, df_v2_dual_core, "-s t3",
             name="affinities", rename_revs=rename_affinity)
    # Plot matrix-mult benchmark
    plot_one(board, df_sched_revs_dual_core, "matrix mult -s",
             rename_revs=rename_matrix_mult, name="matrix-mult")
    # Plot counter benchmark
    plot_one(board, df_sched_revs_dual_core, "counter")
    # Plot leibnitz pi benchmark
    plot_one(board, df_sched_revs_dual_core, "leibnitz pi")
    # Plot async benchmark
    plot_one(board, df_sched_revs_dual_core, "async")
    # Plot busy-poll benchmark
    plot_one(board, df_sched_revs_dual_core, "busy poll",
             rename_revs=rename_busy_poll)
    # Plot busy-poll benchmark
    plot_one(board, df_sched_revs_dual_core, "busy poll",
             rename_revs=rename_busy_poll)
    # Thread-flags
    plot_one(board, df_sched_revs_dual_core, "thread flags")

    # = Locking benchmarks

    # Plot spinlocks
    plot_one(board, df_locking_dual_core, "spinlocks",
             rename_revs=rename_spinlocks)

    # Just compare internal cs vs main
    plot_one(board, filter_revs(df_dual_core, ['multicore-v2', 'multicore-v2-cs']), "sched yield",
             rename_revs=rename_sched, index_map=map_index_locking,
             exclude="affinity", name="sched_locking_cs")

    # Plot sched benchmark for different locking granularities on dual-core
    plot_one(board, df_lock_revs_dual_core, "sched",
             exclude="yield", name="sched_locking")
    # Plot sched-yield benchmark for different locking granularities on single-core
    plot_one(board, df_lock_revs_single_core, "sched yield",
             rename_revs=rename_sched, feat="single-core", index_map=map_index_locking,
             exclude="affinity", name="sched_yield_locking")
    # Plot sched-yield benchmark for different locking granularities on dual-core
    plot_one(board, df_lock_revs_dual_core, "sched yield",
             rename_revs=rename_sched, index_map=map_index_locking,
             exclude="affinity", name="sched_yield_locking")
    # Plot thread-flags for different locking granularities
    plot_one(board, df_lock_revs_dual_core, "flags",
             index_map=map_index_locking, name="flags_locking")
    # Plot thread-access time for different locking granularities
    plot_one(board, df_lock_revs, "threads access",
             index_map=map_index_locking, rename_revs=rename_feat)


# Plot all charts
for board in ["rpi-pico", "espressif-esp32-s3-wroom-1"]:
    df = load_md_table(board)
    print(df)
    plot_all(board, df)
