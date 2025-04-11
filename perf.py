import re
regex = re.compile("<.*>")

results = []

with open("c-code-res.txt") as f:
    cont = f.read()
    for match in regex.findall(cont):
        # print(match)
        match = re.sub(r"[<,\" s>\"]", "", match)
        results.append(float(match))
    print(f"Number of matches: {(len(results))}")
    print(results)
f.close()