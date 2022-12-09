


def update_curr_dir(dir_chain, cmd):
    if cmd[2] == '..':
        dir_chain.pop()
    else:
        dir_chain.append(cmd[2])

def process_ls(f, d, curr_dir):
    line = f.readline().strip()
    d[curr_dir] = {}
    print(">", line)
    while len(line) > 1 and (line[0] != '$'):
        l = line.split(' ')
        
        
        if l[0] != 'dir':
            if 'size' not in d[curr_dir].keys():
                d[curr_dir]['size'] = int(l[0])
            else:
                d[curr_dir]['size'] += int(l[0])
        else:
            if 'dirs' not in d[curr_dir].keys():
                d[curr_dir]['dirs'] = [l[1]]
            else:
                d[curr_dir]['dirs'].append(l[1])

        line = f.readline().strip()


    


def main():
    d = {}
    dir_chain = []
    f = open("input.txt", "r")
    l = f.readline().strip()
    skip = False
    while l:
        cmd = l.split(' ')
        print(dir_chain, cmd)
        if cmd[0] == '$':
            if cmd[1] == 'cd':
                update_curr_dir(dir_chain, cmd)
            if cmd[1] == 'ls':
                process_ls(f,d,dir_chain[-1])
                skip = True


        if not skip:        
            l = f.readline().strip()
        skip = False

    print(d)




main()
