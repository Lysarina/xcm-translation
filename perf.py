import re
import numpy as np
import matplotlib.pyplot as plt
import os
import math
from scipy import stats
from matplotlib.colors import ListedColormap
import matplotlib.patches as mpatches
from collections import defaultdict
import pandas as pd
import scikit_posthocs as sp
import seaborn as sns
# from sklearn.datasets import load_iris

print_details = False
plot_all_values = True # plot all test times in same fig as respective conf interval
plot_sigtest_conf_intervals = False # plot confidence intervals of sig tests (leads to lots of plots)

test_count = 165

confidence = 0.95
alpha = 0.05 # max p-value for significance

custom_colors = ["#004777","#a30000","#ff7700","#efd28d","#00afb5"]

# versions = ["original-c-3", "rustlike-3"]
# files = [20, 20]
# versions = ["original-c-3", "full-c2rust-translation-final-2", "rustlike-2"] # good res lol
versions = ["original-c", "full-c2rust-translation", "rustlike"]
# subversions = [["3"], ["final-2"], ["2"]]
subversions = [["redo"], ["redo"], ["redo"]]
files = [20, 20, 20]

test_times = re.compile(".*<.*>") # find test times
total_time = re.compile("165 tests run in .*") #catch whole res line

data = {} # versions own tests
data_test = {} # tests own versions
data_test["total"] = []

fail = False

for v in range(len(versions)):
    data[v] = {}
    data[v]["total"] = []
    data_test["total"].append([])
    sv = subversions[v]
    for k in range(len(sv)):
        for i in range(1, files[v]+1):
            with open(f"perf_results/{versions[v]}-{sv[k]}-res-{i}.txt") as f:
                content = f.read()
                # Find all tests and their respective times
                count = 0
                fails = 0
                for test_match in test_times.findall(content):
                    time = re.sub(r"[\" s>\"]", "", test_match.split("<")[1])
                    test_name = ":".join(test_match.split(":")[:2])
                    if (i == 1): 
                        data[v][test_name] = []
                        if (v == 0): data_test[test_name] = []
                        data_test[test_name].append([])
                    if "FAILED" in test_match:
                        fails += 1
                        fail = True
                        continue
                    data[v][test_name].append(float(time))
                    data_test[test_name][v].append(float(time))
                    count += 1
                if fails > 0 or count < test_count:
                    print(f"File {versions[v]}-{sv[k]}-res-{i}.txt FAILED {fails} tests")
                # print(f"Number of matches: {(len(data[v]))}")
                # Find total time
                for res_match in total_time.findall(content):
                    time = re.sub(r"165 tests run in ", "", res_match.split("s;")[0])
                    data[v]["total"].append(float(time))
                    data_test["total"][v].append(float(time))
            f.close()


# if fail:
#     exit(0)

versions = ["Original C", "C2Rust Translation", "Rustlike"]

sig_tests = [] # statistically significant tests
data_test_sig = {}
dunn_results = {}  # store Dunn test results here
significant_pairs = {}  # stores significant group pairs per test
performance_comparison = {}  # Store which version was faster per significant pair

# t = test name
# v = array of arrays of test results
for t, v in data_test.items():
    r = stats.kruskal(*v)
    if (r.pvalue < alpha):
        sig_tests.append(t)
        data_test_sig[t] = v
        if print_details: print(f"{t}: F = {r.statistic}, p = {r.pvalue}")

        # Flatten and prepare for Dunn test
        data = np.concatenate(v)
        groups = [i for i, arr in enumerate(v) for _ in arr]
        df = pd.DataFrame({'score': data, 'group': groups})

        # Run Dunn's test with long-form input
        dunn = sp.posthoc_dunn(df, val_col='score', group_col='group', p_adjust='bonferroni')
        dunn_results[t] = dunn

        # Extract significant pairs
        sig_pairs = []
        pairwise_faster = []  # (v_low, v_high, pval)
        for i in dunn.index:
            for j in dunn.columns:
                if i < j and dunn.loc[i, j] < alpha:
                    sig_pairs.append((i, j, dunn.loc[i, j]))  # optionally include p-value
                    # idx_i = int(i[1]) - 1  # convert 'v1' to 0
                    # idx_j = int(j[1]) - 1

                    median_i = np.median(v[i])
                    median_j = np.median(v[j])

                    if median_i < median_j:
                        faster = (i, j, dunn.loc[i, j])  # i faster than j
                    else:
                        faster = (j, i, dunn.loc[i, j])  # j faster than i

                    pairwise_faster.append(faster)
        significant_pairs[t] = sig_pairs
        performance_comparison[t] = pairwise_faster

        if print_details:
            print(f"{t}: Dunn")
            # print("Significant pairwise differences (p < 0.05):")
            # for pair in sig_pairs:
            #     print(f"\t{versions[pair[0]]} vs {versions[pair[1]]}: p = {pair[2]:.8f}")
            for a, b, p in pairwise_faster:
                    print(f"\t{versions[a]} faster than {versions[b]}, p = {p:.8f}")

# for t, v in data_test_sig.items():

win_matrix = np.zeros((3, 3), dtype=int)

# Count wins
for results in performance_comparison.values():
    for faster, slower, _ in results:
        win_matrix[faster, slower] += 1

if print_details: print(win_matrix)

win_matrix_percent = (win_matrix / 166) * 100

# Plot heatmap
plt.figure(figsize=(6, 5))
# sns.heatmap(win_matrix, annot=True, fmt="d", cmap="Blues",
#             xticklabels=versions, yticklabels=versions)
sns.heatmap(win_matrix_percent, annot=True, fmt=".1f", cmap="Blues",
            xticklabels=versions, yticklabels=versions)

# plt.title("Number of Tests Where Version A Was Faster Than B")
plt.xlabel("Slower Version")
plt.ylabel("Faster Version")
plt.tight_layout()
plt.savefig("../xcm-perf-comparison.png")
# plt.show()


count = 0

for t in sig_tests:
    if not plot_sigtest_conf_intervals: break
    if (np.mean(data_test[t][0]) < np.mean(data_test[t][2])): continue
    print(t)
    plt.figure(figsize=(10, 6))
    if (plot_all_values): plt.suptitle(f"{t}", fontsize=14)    
    else: 
        plt.title(f"Confidence Intervals for {t}", fontsize=14)
        plt.xlabel('Version', fontsize=12)
        plt.ylabel('Time (s)', fontsize=12)
    # plt.legend(versions, title="Versions")

    # Plot each version's confidence interval
    for i in range(len(versions)):
        mean = np.mean(data_test[t][i])
        sem = stats.sem(data_test[t][i])  # Standard error of the mean
        margin = sem * stats.t.ppf((1 + confidence) / 2.0, (files[i]*len(subversions[i])) - 1)
        lower_bound = mean - margin
        upper_bound = mean + margin
        
        if (plot_all_values): 
            plt.subplot(2, 2, i+1)
            plt.plot(data_test[t][i], marker='o')
            plt.title(f'Run times for {versions[i]}')
            plt.xlabel('Run')
            plt.ylabel('Time (s)')

            plt.subplot(2, 2, 4)
        
        plt.errorbar(i, mean, yerr=margin, fmt='o', capsize=5)
        if print_details: print(f"\t{versions[i]}\n\t\tMean: {mean}\n\t\t{confidence*100:.1f}% confidence interval: ({lower_bound:.5f}, {upper_bound:.5f})")

    
    if (plot_all_values):
        plt.subplot(2, 2, 4)
        plt.title(f"Confidence Intervals")
        plt.xlabel('Version')
        plt.ylabel('Time (s)')
    # Set xticks to be the version indices
    plt.xticks(range(len(versions)), versions)
    plt.tight_layout()
    count += 1
    if count > 10:
        break

print(f"Total significantly different tests: {len(sig_tests)}")

data_test_means = {}
for t in data_test.keys():
    data_test_means[t] = []
    for i in range(len(versions)):
        mean = np.mean(data_test[t][i])
        data_test_means[t].append(mean)

# Group by prefix before colon
grouped_labels = defaultdict(list)
grouped_values = defaultdict(list)

for label in data_test_means:
    prefix = label.split(':')[0]
    grouped_labels[prefix].append(label)
    grouped_values[prefix].append(data_test_means[label])

colors = ["#004777","#a30000","#ff7700", 'lightgray', 'white']
cmap = ListedColormap(colors)

grid_data = []
row_labels = []
group_row_end_indices = []  # To track where to draw separators
current_row = 0

values_per_row = 30

# Sort groups by size descending
sorted_prefixes = sorted(grouped_labels.keys(), key=lambda k: -len(grouped_labels[k]))

for prefix in sorted_prefixes:
    labels = grouped_labels[prefix]
    values = np.array(grouped_values[prefix])  # shape: (n_items_in_group, 3)

    # Determine min indices
    min_indices = np.argmin(values, axis=1)
    for i, label in enumerate(labels):
        if label not in sig_tests:
            min_indices[i] = 3  # Highlight

    # Pad to full rows of 30
    n = len(min_indices)
    num_rows = math.ceil(n / values_per_row)
    padding = num_rows * values_per_row - n

    if padding > 0:
        min_indices = np.append(min_indices, [4] * padding)  # Padding

    # Reshape and add to grid
    grid_rows = min_indices.reshape((num_rows, values_per_row))
    grid_data.extend(grid_rows)

    # Label only the first row
    row_labels.append(prefix)
    row_labels.extend([""] * (num_rows - 1))

    # Track end of this group for separators
    current_row += num_rows
    group_row_end_indices.append(current_row)

# Plot
fig, ax = plt.subplots(figsize=(values_per_row * 0.3, len(grid_data) * 0.5))
im = ax.imshow(grid_data, aspect='auto', cmap=cmap)

# Draw cell borders (skip padding)
for i, row in enumerate(grid_data):
    for j, val in enumerate(row):
        if val == 4:
            continue
        rect = mpatches.Rectangle((j - 0.5, i - 0.5), 1, 1,
                                  edgecolor='black', facecolor='none', linewidth=0.5)
        ax.add_patch(rect)

# Draw faint horizontal separators between groups
for row in group_row_end_indices[:-1]:  # Exclude final row
    ax.axhline(row - 0.5, color='black', linestyle='--', linewidth=0.5, alpha=0.3)

# Y-axis labels
ax.set_yticks(np.arange(len(row_labels)))
ax.set_yticklabels(row_labels, fontsize=8)

# Remove x ticks
ax.set_xticks([])

# Legend
legend_patches = [
    mpatches.Patch(color=colors[0], label=versions[0]),
    mpatches.Patch(color=colors[1], label=versions[1]),
    mpatches.Patch(color=colors[2], label=versions[2]),
    mpatches.Patch(color=colors[3], label="Not stat. sig.")
    # mpatches.Patch(color=colors[4], label="Padding")
]
ax.legend(handles=legend_patches, loc='upper center', bbox_to_anchor=(0.5, -0.05),
          ncol=len(legend_patches), frameon=False)

plt.tight_layout()
plt.savefig("../xcm-all-tests.png")
if count < 30:
    plt.show()