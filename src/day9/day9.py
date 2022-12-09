#!/usr/bin/env python3.10

LOG = False
PART = 2


def coord_add(c1, c2):
    return ((c1[0]+c2[0], c1[1]+c2[1]))

def touching(c1, c2):
    return abs(c1[0]-c2[0]) < 2 and abs(c1[1]-c2[1]) < 2

def move_same_row(c1, c2):
    dir = 1 if c1[0] < c2[0] else -1
    return (c1[0]+dir,  c1[1])

def move_same_col(c1, c2):
    dir = 1 if c1[1] < c2[1] else -1
    return (c1[0],  c1[1]+dir)

def move_diagonal(c1, c2):
    dir1 = 1 if c1[0] < c2[0] else -1
    dir2 = 1 if c1[1] < c2[1] else -1
    return (c1[0]+dir1, c1[1]+dir2)

moves_h = []

pos_t_hist = set()
with open("input.txt") as file:
    for line in file:
        command = line.rstrip().split(' ')
        dir, val = command[0], int(command[1])
        cmd = (0,0)
        match dir:
            case 'L': cmd = ((-1,0),val)
            case 'R': cmd = (( 1,0),val)
            case 'U': cmd = ((0, 1),val)
            case 'D': cmd = ((0,-1),val)
            case _: print("YOU BROKE IT")
        moves_h.append(cmd)

overall_idx = 0
pos_t_hist.add((0,0))


node_cnt = 2 if PART == 1 else 10
nodes = [(0,0) for _ in range(0,node_cnt)]

for move in moves_h:
    for times in range(0, move[1]):
        if LOG: overall_idx += 1
        # 0th node is treated as the head
        nodes[0] = coord_add(nodes[0], move[0])
        for idx_h,idx_t in zip(range(0, len(nodes)), range(1,len(nodes))):
            pos_h, pos_t = nodes[idx_h], nodes[idx_t]
            if touching(pos_h, pos_t):
                if LOG: print(overall_idx, "touching", pos_t, pos_h)
                continue
            elif (pos_h[1] == pos_t[1]):
                pos_t = move_same_row(pos_t, pos_h)
                if LOG: print(overall_idx, "move towards same row", pos_t, pos_h)
            elif (pos_h[0] == pos_t[0]):
                pos_t = move_same_col(pos_t, pos_h)
                if LOG: print(overall_idx, "move towards same col", pos_t, pos_h)
            else:
                pos_t = move_diagonal(pos_t, pos_h)
                if LOG: print(overall_idx, "move towards same diagonal", pos_t, pos_h)
            if (idx_t == len(nodes)-1):
                pos_t_hist.add(pos_t)
            nodes[idx_h], nodes[idx_t] = pos_h, pos_t
print(len(pos_t_hist))
