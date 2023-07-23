import re
import matplotlib.pyplot as plt
from scipy.cluster.hierarchy import dendrogram
import pickle

# Load the pickle file
with open('data/dendrogram.pkl', 'rb') as f:
    string = pickle.load(f)

with open('data/names.pkl', 'rb') as f:
    names = pickle.load(f)

# Extract the step information
steps_str = re.search(r'steps: \[(.*?)\]', string).group(1)
steps = re.findall(r'Step { (.*?) }', steps_str)

# Extract the number of observations
observations = int(re.search(r'observations: (\d+)', string).group(1))

data = []
for step in steps:
    cluster1 = int(re.search(r'cluster1: (\d+)', step).group(1))
    cluster2 = int(re.search(r'cluster2: (\d+)', step).group(1))
    dissimilarity = float(re.search(r'dissimilarity: (\d+\.\d+)', step).group(1))
    size = int(re.search(r'size: (\d+)', step).group(1))
    data.append([cluster1, cluster2, dissimilarity, size])

# Prepare the linkage matrix
linkage_matrix = [[i[0], i[1], i[2], i[3]] for i in data]

# Plot the dendrogram
dendrogram(linkage_matrix, labels=names)

# Add labels and title
plt.xlabel('Observations')
plt.ylabel('Distance')
plt.title('Dendrogram')
plt.show()
plt.savefig("data/dendrogram.png")