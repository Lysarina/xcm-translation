import re
import numpy as np
import matplotlib.pyplot as plt
import os
from scipy import stats

test_count = 165

versions = ["original-c-2", "rustlike-2"]
files = [20, 20]
# versions = ["original-c-2", "full-c2rust-translation-final-2", "rustlike-2"]
# files = [20, 20, 20]

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
    if (np.mean(data_test[t][0]) < np.mean(data_test[t][1])): continue
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
    if count > 30:
        break
    

print(f"Total significantly different tests: {len(sig_tests)}")

plt.show()