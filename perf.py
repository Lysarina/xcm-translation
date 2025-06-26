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

def tolerant_mean(arrs):
    lens = [len(i) for i in arrs]
    arr = np.ma.empty((np.max(lens),len(arrs)))
    arr.mask = True
    for idx, l in enumerate(arrs):
        arr[:len(l),idx] = l
    return arr.mean(axis = -1), arr.std(axis=-1)

subfolder = "paper/"
fig_save_name = ""

print_details = False
plot_all_values = True # plot all test times in same fig as respective conf interval
plot_sigtest_conf_intervals = False # plot confidence intervals of each sig tests (leads to lots of plots)
max_plots = 30 # max conf interval plots (recommmended to not bust the computer)

warmups = 4

test_count = 165

confidence = 0.95
alpha = 0.05 # max p-value for significance

custom_colors = ["#004777","#a30000","#ff7700","#efd28d","#00afb5", "#9DBD9F"]

versions = ["original-c", "full-c2rust-translation", "rustlike"]
prettified_versions = ["Original", "MinMod", "RustLike"]
subversions = [["redo", "redo-2", "2", "3"], ["redo", "redo-2", "redo-3","final-2"], ["redo", "redo-2", "2", "3"]] # all
files = [20, 20, 20]

test_times = re.compile(".*<.*>") # find test times
total_time = re.compile("165 tests run in .*") #catch whole res line

data = {} # versions own tests
data_sv = {}
data_test = {} # tests own versions
data_test["total"] = []

fail = False

# Read all data
for v in range(len(versions)):
    data_sv[v] = {}
    data_sv[v]["total"] = []
    sv = subversions[v]
    for k in range(len(sv)):
        data_sv[v]["total"].append([])
        for i in range(warmups+1, files[v]+1):
            with open(f"perf_results/{versions[v]}-{sv[k]}-res-{i}.txt") as f:
                content = f.read()
                # Find all tests and their respective times
                count = 0
                fails = 0
                for test_match in test_times.findall(content):
                    time = re.sub(r"[\" s>\"]", "", test_match.split("<")[1])
                    test_name = ":".join(test_match.split(":")[:2])
                    if (k == 0 and (i == warmups+1 or test_name not in data_sv[v].keys())): 
                        data_sv[v][test_name] = []
                        data_sv[v][test_name].append([])
                    elif (i == warmups+1 or test_name not in data_sv[v].keys()):
                        data_sv[v][test_name].append([])
                    if "FAILED" in test_match:
                        fails += 1
                        fail = True
                        continue
                    data_sv[v][test_name][k].append(float(time))
                    count += 1
                if fails > 0 or count < test_count:
                    print(f"File {versions[v]}-{sv[k]}-res-{i}.txt FAILED {fails} tests")
                # Find total time
                for res_match in total_time.findall(content):
                    time = re.sub(r"165 tests run in ", "", res_match.split("s;")[0])
                    data_sv[v]["total"][k].append(float(time))
            f.close()

# Take average of runs
for v in range(len(versions)):
    data[v] = {}
    for test_name, res in data_sv[v].items():
        if (v == 0):
            data_test[test_name] = []
        data_test[test_name].append([])
        if len(subversions[v]) == 1:
            avg = res[0]
        else:
            y, error = tolerant_mean(res)
            avg = y.data
        data[v][test_name] = avg
        data_test[test_name][v] = avg

versions = prettified_versions

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
                    sig_pairs.append((i, j, dunn.loc[i, j]))

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
            print(t)
            for a, b, p in pairwise_faster:
                print(f"\t{versions[a]} faster than {versions[b]}, p = {p:.8f}")
    elif print_details:
        print(t)
        print("\tNot stat sig")

win_matrix = np.zeros((3, 3), dtype=int)

# Count wins
for results in performance_comparison.values():
    for faster, slower, _ in results:
        win_matrix[faster, slower] += 1

if print_details: print(win_matrix)

win_matrix_percent = (win_matrix / 166) * 100

# Plot heatmap
plt.figure(figsize=(6, 5))
sns.heatmap(win_matrix_percent, annot=True, fmt=".1f", cmap="Blues",
            xticklabels=versions, yticklabels=versions, cbar_kws={'label': '% of tests'})
plt.xlabel("Slower Version")
plt.ylabel("Faster Version")
plt.tight_layout()
plt.savefig(f"../{subfolder}xcm-perf-comparison-{fig_save_name}.png")


count = 0
if plot_sigtest_conf_intervals:
    for t in sig_tests:
        if print_details: print(t)
        plt.figure(figsize=(10, 6))
        if (plot_all_values): plt.suptitle(f"{t}", fontsize=14)    
        else: 
            plt.title(f"Confidence Intervals for {t}", fontsize=14)
            plt.xlabel('Version', fontsize=12)
            plt.ylabel('Time (s)', fontsize=12)

        # Plot each version's confidence interval
        for i in range(len(versions)):
            median = np.median(data_test[t][i])
            mean = np.mean(data_test[t][i])
            sem = stats.sem(data_test[t][i])  # Standard error of the mean
            margin = sem * stats.t.ppf((1 + confidence) / 2.0, files[i] - 1 - warmups)
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
            plt.plot(i, median, marker='D')
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
        if count > max_plots:
            break

print(f"Total significantly different tests: {len(sig_tests)}")

## PLOT ALL TESTS

data_test_medians = {}
for t in data_test.keys():
    data_test_medians[t] = []
    for i in range(len(versions)):
        median = np.median(data_test[t][i])
        data_test_medians[t].append(median)

# Group by prefix before colon
grouped_labels = defaultdict(list)
grouped_values = defaultdict(list)

# joint = "addr\nattr_map\nattr_path\nattr_tree"
joint = "attr_map\nattr_path\nattr_tree"
group_boundaries = [0, 0, 0]
for label in data_test_medians:
    prefix = label.split(':')[0]
    if prefix == "total": continue
    if prefix == "attr_map" or prefix == "attr_path" or prefix == "attr_tree":
        if prefix == "attr_map": 
            group_boundaries[0] += 1
            group_boundaries[1] += 1
        elif prefix == "attr_path":
            group_boundaries[1] += 1
        group_boundaries[2] += 1 
        grouped_labels[joint].append(label)
        grouped_values[joint].append(data_test_medians[label])
    else:
        grouped_labels[prefix].append(label)
        grouped_values[prefix].append(data_test_medians[label])

colors = ["#004777","#a30000","#ff7700", 'lightgray', 'white']
cmap = ListedColormap(colors)

grid_data = []
row_labels = []
group_row_end_indices = []  # To track where to draw separators
current_row = 0

values_per_row = 40

# Sort groups by size descending
sorted_prefixes = sorted(grouped_labels.keys(), key=lambda k: -len(grouped_labels[k]))

for prefix in sorted_prefixes:
    labels = grouped_labels[prefix]
    values = np.array(grouped_values[prefix])  # shape: (n_items_in_group, 3)

    # Determine min indices
    min_indices = np.argmin(values, axis=1)
    for i, label in enumerate(labels):
        if label not in sig_tests:
            min_indices[i] = 3  # Not statistically significant

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
fig, ax = plt.subplots(figsize=(values_per_row * 0.3, len(grid_data) * 0.45))
im = ax.imshow(grid_data, interpolation='none', aspect='auto', cmap=cmap)

# Draw cell borders (skip padding)
for i, row in enumerate(grid_data):
    for j, val in enumerate(row):
        if val == 4:
            continue
        # if i == 4 and j in group_boundaries:
        #     ax.plot([j - 0.5, j - 0.5], [i - 0.5, i + 0.5],
        #         color='black', linewidth=2)
        rect = mpatches.Rectangle((j - 0.5, i - 0.5), 1, 1,
                                  edgecolor='black', facecolor='none', linewidth=0.4)
        ax.add_patch(rect)

# Draw faint horizontal separators between groups
for row in group_row_end_indices[:-1]:  # Exclude final row
    ax.axhline(row - 0.5, color='black', linestyle='--', linewidth=0.4, alpha=0.3)

# Y-axis labels
ax.set_yticks([i for i, label in enumerate(row_labels) if label != ""])
ax.set_yticklabels([label for label in row_labels if label != ""], fontsize=8)

# Remove x ticks
ax.set_xticks([])

# Legend
legend_patches = [
    mpatches.Patch(color=colors[0], label=versions[0]),
    mpatches.Patch(color=colors[1], label=versions[1]),
    mpatches.Patch(color=colors[2], label=versions[2]),
    mpatches.Patch(color=colors[3], label="No stat. sig.")
]
ax.legend(handles=legend_patches, loc='upper center', bbox_to_anchor=(0.5, -0.05),
          ncol=len(legend_patches), frameon=False)

plt.tight_layout()
plt.savefig(f"../{subfolder}xcm-all-tests-{fig_save_name}.png", dpi=300, bbox_inches='tight')
if count < 30:
    plt.show()