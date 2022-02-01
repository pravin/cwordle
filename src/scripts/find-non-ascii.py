with open("words-list.txt") as fp:
    data = fp.read()
    for i in range(len(data)):
        c = data[i]
        if 0 <= ord(c) <= 127:
            continue
        else:
            print(c, data[i : i + 12])
