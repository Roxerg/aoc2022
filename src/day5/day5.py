import re
import copy

import time
start_time = time.time()

def read_file(): return open("input.txt", "r").read()

def move_boxes(commands, _stacks, part2=False):
    stacks = copy.deepcopy(_stacks)

    for command in commands.split("\n"):
        cmd = list(map(int, re.findall(r"[0-9]+", command)))
        times, src, dest = cmd[0], cmd[1]-1, cmd[2]-1

        moving, stacks[src] = stacks[src][(-times):], stacks[src][:(-times)]
        stacks[dest] += reversed(moving) if part2 else moving

    return stacks

def main():
    layout, commands = read_file().split("\n\n")
    maxx = int(re.findall(r"[0-9]+", layout)[-1])

    stacks = [[] for _ in range(0, maxx)]

    for i, box in enumerate([x[1] for x in re.findall(r"(\[(.*?)\]| {4})", layout)]):
        if box != '': stacks[i % maxx].insert(0,box)

    print('part1:', ''.join(s[-1] for s in move_boxes(commands, stacks)))
    print('part2:', ''.join(s[-1] for s in move_boxes(commands, stacks, True)))
    print(time.time() - start_time)