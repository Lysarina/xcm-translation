import re
import numpy as np
import matplotlib.pyplot as plt
import os
import math
from scipy import stats
from matplotlib.colors import ListedColormap
import matplotlib.patches as mpatches

test_count = 165

# versions = ["original-c-3", "rustlike-3"]
# files = [20, 20]
versions = ["original-c-3", "full-c2rust-translation-final-2", "rustlike-2"]
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
    for i in range(1, files[v]+1):
        with open(f"perf_results/{versions[v]}-res-{i}.txt") as f:
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
                print(f"File {versions[v]}-res-{i}.txt FAILED {fails} tests")
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

confidence = 0.95

sig_tests = [] # statistically significant tests
# t = test name
# v = array of arrays of test results
for t, v in data_test.items():
    r = stats.kruskal(*v)
    if (r.pvalue < 0.05):
        sig_tests.append(t)
        print(f"{t}: F = {r.statistic}, p = {r.pvalue}\n")

plot_all_values = True # plot all test times in same fig as respective conf interval

count = 0

for t in sig_tests:
    break
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
        margin = sem * stats.t.ppf((1 + confidence) / 2.0, files[i] - 1)
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
        print(f"\t{versions[i]}\n\t\tMean: {mean}\n\t\t{confidence*100:.1f}% confidence interval: ({lower_bound:.5f}, {upper_bound:.5f})")

    
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

colors = ['red', 'green', 'blue', 'gold', 'lightgray']
cmap = ListedColormap(colors)

# Extract labels and values
labels = list(data_test_means.keys())
print(len(labels))
values = np.array(list(data_test_means.values()))  # shape: (n, 3)

# Determine which set has min value per label
min_indices = np.argmin(values, axis=1)

# Override indices with highlight color (index 3) if in highlight_dict
for i, label in enumerate(labels):
    if label not in sig_tests:
        min_indices[i] = 3  # Highlight

# Reshape into grid of 33 values per row
values_per_row = 30
n = len(min_indices)
num_rows = math.ceil(n / values_per_row)
padding = num_rows * values_per_row - n

# Add padding
if padding > 0:
    min_indices = np.append(min_indices, [4] * padding)  # 4 = padding color
    labels += [""] * padding

# Reshape
grid = min_indices.reshape((num_rows, values_per_row))

# Plot
fig, ax = plt.subplots(figsize=(values_per_row * 0.3, num_rows * 0.5))
im = ax.imshow(grid, aspect='auto', cmap=cmap)

# Draw borders
for i in range(num_rows):
    for j in range(values_per_row):
        rect = mpatches.Rectangle((j - 0.5, i - 0.5), 1, 1,
                                  edgecolor='black', facecolor='none', linewidth=0.5)
        ax.add_patch(rect)

# Remove ticks
ax.set_xticks([])
ax.set_yticks([])

# Title
# ax.set_title("Minimum Value Set per Key (highlighted values in gold)")

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
plt.savefig("../all-tests.png")

if count < 30:
    plt.show()