s = input()
if "+" in s:
    it = s.split("+")
    a = int(it[0], 8)
    b = int(it[1], 8)
    print("{0:o}".format(a+b))
elif "/" in s:
    it = s.split("/")
    a = int(it[0], 8)
    b = int(it[1], 8)
    if b == 0:
        print("invalid")
    else:
        print("{0:o}".format(a//b))
elif "*" in s:
    it = s.split("*")
    a = int(it[0], 8)
    b = int(it[1], 8)
    print("{0:o}".format(a*b))
else:
    it = s.split("-")
    if it[0] == "":
        it.pop(0)
        it[0] = "-" + it[0]
    a = int(it[0], 8)
    b = int(it[1], 8)
    print("{0:o}".format(a-b))


