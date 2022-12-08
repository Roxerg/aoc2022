

def step_through(grid, w, h, increment_h, increment_w):
    thresh = 0 + grid[h][w]
    max_h = len(grid)-1
    max_w = len(grid[0])-1
    reached_edge = True

    while w < max_w and h < max_h and w > 0 and h > 0:
        w, h = w+increment_w, h+increment_h
        if w > max_w or h > max_h or w < 0 or h < 0:
            break
        if grid[h][w] >= thresh:
            reached_edge = False
            break
    return reached_edge

def step_through_p2(grid, w, h, increment_h, increment_w):
    thresh = 0 + grid[h][w]
    max_h = len(grid)-1
    max_w = len(grid[0])-1

    score = 0

    while w < max_w and h < max_h and w > 0 and h > 0:
        w, h = w+increment_w, h+increment_h
        if w > max_w or h > max_h or w < 0 or h < 0:
            print("stopped because edge")
            break
        if grid[h][w] < thresh:
            print("met tree ", grid[h][w])
            score += 1
        elif grid[h][w] >= thresh:
            score += 1
            print("stopped because tall")
            break

    print(score)

    return score




with open("input.txt") as file:
    input = file.read()
    input = input.split('\n')
    grid = []
    for i, v in enumerate(input):
        grid.append([])
        for vv in v.rstrip():
            grid[i].append(int(vv))

    height, width = len(grid[0]), len(grid)

    counter = 0



    for w in range(0, width):
        for h in range(0, height): 
            if step_through(grid, w, h, 0, 1) or \
                step_through(grid, w, h, 1, 0) or \
                step_through(grid, w, h, -1, 0) or \
                step_through(grid, w, h, 0, -1):
                counter += 1

    maxx = 0
    for w in range(0, width):
        for h in range(0, height): 
            curr_score = step_through_p2(grid, w, h, 0, 1) * step_through_p2(grid, w, h, 1, 0) * step_through_p2(grid, w, h, -1, 0) * step_through_p2(grid, w, h, 0, -1)
            if (maxx < curr_score):
                maxx = curr_score
    
    
    
    print(counter)
    print("----")   
    print(maxx)


