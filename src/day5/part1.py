import re




def read_file():
    with open("input.txt", "r") as file:
        return file.read()




def main():
    input = read_file().split("\n\n")
    layout, commands = input[0], input[1]
    idxes = re.findall(r"[0-9]", layout)
    maxx = int(idxes[-1])


    stacks = {}
    print(layout)
    layout = layout.replace("    ", "x")
    print(layout)

    boxes = re.findall(r"(\[(.*?)\]|x)", layout)

    print(boxes)
    idx = 0
    while idx < len(boxes)-1:
        for row_idx in range(0, maxx+1):
            box = boxes[idx]
            idx += 1 
            if (idx > len(boxes) - 1):
                break
            # print(box)
            if box[1] != '':
                if stacks.get(row_idx+1) == None:
                    stacks[row_idx+1] = []
                stacks[row_idx+1].append(box[1])
                print(box[1], " goes to ", row_idx+1)

    for stack in stacks.values():
        stack.reverse()

    for i in range(1,10):
        print(i, stacks[i])
        

    
    for command in commands.split("\n"):
        print(stacks[9])
        cmd = list(map(int, re.findall(r"[0-9]+", command)))
        # print(cmd[0])
        for n in range(0, cmd[0]):
            # print(n)
            stacks[cmd[2]].append(stacks[cmd[1]].pop(-1))


    res = ""
    for idx in range(1, maxx+1):
        res += stacks[idx][-1]

    print(stacks)
    print(res)



main()