import pandas as pd
import matplotlib.pyplot as plt
import matplotlib

board="rpi-pico"

def load_md_table():
    df = pd.read_table("data/"+board+".md", engine='python', sep=r" \| ", header=0, skipinitialspace=True).iloc[1:]
    # Update 'main' field to match the pattern '<feat> -n <rev>' of the other fields;
    df.loc[df['source'] == 'main', 'source'] = 'dual-core -s main'
    main2 = df.loc[df['source'] == 'dual-core -s main'].copy()
    main2['source'] = 'single-core -s main'
    df = pd.concat([main2, df ], ignore_index=True)
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

def benchmark(df, name):
    return df.filter(regex="|".join(["Rev", "Feat", name]))


def bar_plot(df, name):
    if len(df.columns) > 1:
        df = df.transpose()
    ax = df.plot.bar(
        rot=15,
        # figsize=(10,5),
        colormap="summer",
        edgecolor = "black",
        legend=len(df.columns) > 1,
    )
    # plt.xlabel(xlabel, fontsize=16)
    # plt.ylabel("ylabel", fontsize=16)
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    plt.subplots_adjust(bottom=0.12, top=1, right=0.98, left=0.1)
    plt.savefig("graphs/" + name + "_" + board + ".png", )
    plt.close()

df = load_md_table()
print(df)

single_core = filter_feature(df, 'single-core')
dual_core = filter_feature(df, 'dual-core')

v2 = filter_revs(df, ['multicore-v2'])

## Sched Yield

bar_plot(benchmark(single_core, "yield"), "sched-yield_single-core")
bar_plot(benchmark(dual_core, "yield"), "sched-yield_dual-core")


v2_dual_core = filter_revs(dual_core, ['multicore-v2'])

### Fib 

fib = benchmark(v2_dual_core, "fib")
bar_plot(fib, "fib_dual-core")


### Matrix Mult


fib = benchmark(dual_core, "matrix mult")
bar_plot(fib, "matrix-mult")
