import re
import numpy as np
import matplotlib.pyplot as plt
import os
from scipy import stats

test_count = 165

versions = ["original-c", "full-c2rust-translation"]
# files = (10, 5)
# versions = ["rs-1", "c-code"]
files = [10, 10]

warmups = 1

test_times = re.compile(".*<.*>") # find test times
total_time = re.compile("165 tests run in .*") #catch whole res line

data = {} # versions own tests
data_test = {} # tests own versions
data_test["total"] = []

for v in range(len(versions)):
    data[v] = {}
    data[v]["total"] = []
    data_test["total"].append([])
    for i in range(warmups+1, files[v]+1):
        with open(f"perf_results/{versions[v]}-res-{i}.txt") as f:
            content = f.read()
            # Find all tests and their respective times
            count = 0
            for test_match in test_times.findall(content):
                time = re.sub(r"[\" s>\"]", "", test_match.split("<")[1])
                test_name = ":".join(test_match.split(":")[:2])
                if (i == warmups+1): 
                    data[v][test_name] = []
                    if (v == 0): data_test[test_name] = []
                    data_test[test_name].append([])
                data[v][test_name].append(float(time))
                data_test[test_name][v].append(float(time))
                count += 1
            if count < test_count:
                print(f"File {versions[v]}-res-{i}.txt FAILED {test_count-count} tests")
                exit(0)
            # print(f"Number of matches: {(len(data[v]))}")
            # Find total time
            for res_match in total_time.findall(content):
                time = re.sub(r"165 tests run in ", "", res_match.split("s;")[0])
                data[v]["total"].append(float(time))
                data_test["total"][v].append(float(time))
        f.close()

# variance = {}
# std = {}

# for t, v in results.items():
#     variance[t] = np.var(v)/np.mean(v)
#     std[t] = np.std(v)/np.mean(v)
# sorted1 = {k: v for k, v in sorted(variance.items(), key=lambda item: item[1])}
# sorted2 = {a: b for a, b in sorted(std.items(), key=lambda item: item[1])}
# # sorted3 = {k: v for k, v in sorted(results.items(), key=lambda item: np.var(item[1]))}

# print(sorted1)
# print()
# print(sorted2)
# print(results["xcm:tls_net_hiccup"])
# print(results["attr_path:parse_unparse"])
# print(results["xcm:backpressure_with_slow_server"])
# print(data_test)
confidence = 0.95

sig_tests = [] # statistically significant tests
# t = test name
# v = array of arrays of test results
for t, v in data_test.items():
    r = stats.kruskal(*v)
    if (r.pvalue < 0.05):
        sig_tests.append(t)
        print(f"{t}: F = {r.statistic}, p = {r.pvalue}\n")


# for t in sig_tests:
#     # for each version time array
#     print(t)
#     for i in range(len(data_test[t])):
#         print(f"\t{versions[i]}")
#         v = data_test[t][i]
#         mean = np.mean(v) # mean over all times of test
#         print(f"\t\tMean: {mean}\n")
#         sem = stats.sem(v)  # Standard error of the mean

#         margin = sem * stats.t.ppf((1 + confidence) / 2.0, files[i] - 1)
#         lower_bound = mean - margin
#         upper_bound = mean + margin
#         print(f"\t\t{confidence*100:.1f}% confidence interval: ({lower_bound:.5f}, {upper_bound:.5f})")

#         plt.figure()
#         plt.plot(v, marker='o')
#         plt.title(f'{t}: {versions[i]}')
#         plt.xlabel('Run')
#         plt.ylabel('Time (s)')

# variance = {}
# std = {}

# for t, v in means.items():
#     # variance[t] = np.var(v)/np.mean(v)
#     std[t] = np.std(v)/np.mean(v)
# # var_sorted = dict(sorted(variance.items(), key=lambda item: item[1], reverse=True))
# std_sorted = dict(sorted(std.items(), key=lambda item: item[1], reverse=True))
# # print(std_sorted)

plot_all_values = True

count = 0

for t in sig_tests:
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
        # Plotting the confidence interval as a shaded area
        plt.fill_between([i - 0.1, i + 0.1], lower_bound, upper_bound, color="lightgray", alpha=0.5)
        
        # Plot the mean as a point
        plt.plot(i, mean, marker='o', markersize=8, label=versions[i] if r == 0 else "")
        print(f"\t{versions[i]}\n\t\tMean: {mean}\n\t\t{confidence*100:.1f}% confidence interval: ({lower_bound:.5f}, {upper_bound:.5f})")

    # Add legend only for the first plot
    # if r == 0:
    
    if (plot_all_values):
        plt.subplot(2, 2, 4)
        plt.title(f"Confidence Intervals")
        plt.xlabel('Version')
        plt.ylabel('Time (s)')
    # Set xticks to be the version indices
    plt.xticks(range(len(versions)), versions, rotation=45)

    count += 1
    if count > 50:
        break

print(f"Total significantly different tests: {len(sig_tests)}")

plt.tight_layout()

plt.show()