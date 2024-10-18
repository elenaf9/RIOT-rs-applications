import pandas as pd
import matplotlib.pyplot as plt
import matplotlib

board = "rpi-pico"

colors = {
    "rpi-pico": {
        "single-core": ["#417279", "#9cb3b7", "#f9f9f9"],
        "dual-core": ["#00535c", "#6f9297", "#cad6d8"],
    },
    "espressif-esp32-s3-wroom-1": {
        "single-core": ["#974a5a", "#cba0a6", "#f9f9f9"],
        "dual-core": ["#7b1b38", "#b2757f", "#e3cccf"]
    }
}

map_index_sched = {
    "main": "main",
    "multicore-v1": "Allocation",
    "multicore-v2": "Dynamic",
    "multicore-v2-cs": "Internal CS",
    "multicore-v2-locking": "Fine-grained Locking",
}

map_index_locking = {
    "multicore-v2": "Original",
    "multicore-v2-cs": "Internal CS",
    "multicore-v2-locking": "Fine-grained Locking",
}

rename_fib = {
    'fib -s none': 'None',
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
    'sched yield t -s t3 -s affinity-0': 'Pin on Core 0',
    'sched yield t -s t3 -s affinity-1': 'Pin on Core 1',
}

rename_busy_poll = {
    'busy poll -s poll': 'Polling',
    'busy poll -s await': 'Interrupt',
}

rename_feat = {
    'single-core': 'Single-Core',
    'dual-core': 'Multi-Core'
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
        rot=15,
        # colormap="bone",
        color=colors[board][feat],
        edgecolor="black",
        legend=len(df.columns) > 1,
    )
    plt.xlabel("", fontsize=16)
    plt.ylabel("", fontsize=16)
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    plt.subplots_adjust(bottom=0.15, top=1, right=0.98, left=0.1)
    plt.savefig("graphs/" + name + "_" + feat + "_" + board + ".png", )
    plt.close()


def plot_one(board, df, search,
             feat="dual-core", index_map=map_index_sched, rename_revs=None, exclude=None, name=None):
    df_benchmark = benchmark(df, search, exclude)

    if isinstance(df_benchmark.index, pd.MultiIndex) and len(df_benchmark.columns) == 1:
        df_benchmark = df_benchmark.unstack(level=1)
        df_benchmark.columns = df_benchmark.columns.droplevel(0)[::-1]

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

    # Locking versions
    df_lock_revs = filter_revs(
        df, ['multicore-v2', 'multicore-v2-cs', 'multicore-v2-locking'])
    # Single Core locking-version
    df_lock_revs_single_core = filter_revs(
        df_single_core, ['multicore-v2', 'multicore-v2-cs', 'multicore-v2-locking'])
    # Dual Core locking-version
    df_lock_revs_dual_core = filter_revs(
        df_dual_core, ['multicore-v2', 'multicore-v2-cs', 'multicore-v2-locking'])

    # == Plot benchmark data

    plot_one(board, df_sched_revs_single_core, "sched yield",
             rename_revs=rename_sched, feat="single-core", exclude="affinity")
    plot_one(board, df_sched_revs_dual_core, "sched yield",
             rename_revs=rename_sched,  exclude="affinity")
    # plot_one(board, df_sched_revs_dual_core, "-s t3", name="affinities", rename=rename_affinity)
    plot_one(board, df_v2_dual_core, "fib",
             rename_revs=rename_fib)
    plot_one(board, df_sched_revs_dual_core, "matrix mult")
    plot_one(board, df_sched_revs_dual_core, "counter")
    plot_one(board, df_sched_revs_dual_core, "leibnitz pi")
    plot_one(board, df_sched_revs_dual_core, "async")
    plot_one(board, df_sched_revs_dual_core, "busy poll",
             rename_revs=rename_busy_poll)
    plot_one(board, df_lock_revs_single_core, "-s t3",
             feat="single-core", index_map=map_index_locking, name="locking")
    plot_one(board, df_lock_revs_dual_core, "-s t3",
             index_map=map_index_locking, name="sched_locking")
    plot_one(board, df_lock_revs_dual_core, "flags",
             index_map=map_index_locking, name="flags_locking")
    plot_one(board, df_lock_revs, "threads access",
             index_map=map_index_locking, rename_revs=rename_feat)


# Plot all charts
for board in ["rpi-pico", "espressif-esp32-s3-wroom-1"]:
    df = load_md_table(board)
    print(df)
    plot_all(board, df)
