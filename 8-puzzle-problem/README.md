# 8-puzzle

Implemented BFS and A* to solve the n-puzzle problem. The scripts print the results to an output.txt file.

### Instructions to execute

You need to have 2 python files: state.py and the python file with the algorithm you want to run (bfs.py or astar.py). Make sure you have the files saved in the same directory. Then, on the command line execute:
```
python bfs.py bfs 7,2,4,5,0,6,8,3,1
```
or
```
python astar.py bfs 7,2,4,5,0,6,8,3,1
```
### Results with BFS
```
Path to goal: ['Left', 'Up', 'Right', 'Down', 'Down', 'Left', 'Up', 'Right', 'Right', 'Up', 'Left', 'Left', 'Down', 'Right', 'Right', 'Down', 'Left', 'Up', 'Right', 'Up', 'Left', 'Down', 'Down', 'Left', 'Up', 'Up']
Cost to the path: 26
Number of visited nodes at the end: 171711
Running_time: 2.70168495
Used memory: 151.40000000
```
### Results with A*
```
Path to goal: ['Left', 'Up', 'Right', 'Down', 'Right', 'Down', 'Left', 'Left', 'Up', 'Right', 'Right', 'Down', 'Left', 'Left', 'Up', 'Right', 'Right', 'Up', 'Left', 'Left', 'Down', 'Right', 'Right', 'Up', 'Left', 'Left']
Cost to the path: 26
Number of visited nodes at the end: 3088
Running time: 0.14735603
Used memory: 15.91200000
```
