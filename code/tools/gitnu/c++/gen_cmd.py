import subprocess, sys


class Switcher:
    def __init__(self, idx: int, subcommands: list[str]):
        self.idx = idx
        self.subcommands = subcommands

    def __enter__(self):
        print("switch(arg[", self.idx, "]) {", sep="")
        for subcommand in filter(lambda v: len(v) == self.idx, self.subcommands):
            print(case_return("0", subcommand))

    def __exit__(self, a, b, c):
        del a, b, c
        print("default: return kNone;")
        print("}")


output = subprocess.check_output(["git", "help", "--all"], encoding="ascii")
output = output.split("External commands", maxsplit=1)[0]
lines = filter(lambda v: v.startswith("   "), output.splitlines())
lines = map(lambda v: v.removeprefix("   ").split(" ", maxsplit=1)[0], lines)
git_subcommands = sorted(lines)

git_subcommands.remove("status")
git_subcommands = ["status"] + git_subcommands

# All possible characters in the subcommands list
CHARS = "".join(map(chr, range(ord("a"), ord("z") + 1))) + "-"
CHARS += "".join(map(str, range(0, 10)))

for subcommand in git_subcommands:
    for char in subcommand:
        assert char in CHARS, f"{char} not found"


def cap1(x: str) -> str:
    return x[0].upper() + x[1:]


def enumize(x: str) -> str:
    j = x.find("-")
    while j != -1:
        x = x[:j] + cap1(x[j + 1 :])
        j = x.find("-")
    return "k" + cap1(x)


def case_return(char: str, subcommand: str) -> str:
    return "case %s: return %s;" % (char, enumize(subcommand))


def switch2(idx: int, subcommands: list[str]):
    for c in CHARS:
        sublist = [x for x in subcommands if idx < len(x) and x[idx] == c]
        if len(sublist) == 0:
            continue
        elif len(sublist) == 1:
            print(case_return("'" + c + "'", sublist[0]))
        else:
            print("case '%s': {" % c)
            with Switcher(idx + 1, sublist):
                switch2(idx + 1, sublist)
            print("}")


with open("src/git_cmd.h", "w") as sys.stdout:
    print("enum GitCommand {")
    print("kNone = 0,")
    for subcommand in git_subcommands:
        print(enumize(subcommand) + ",")
    print("};")
    print()
    print("GitCommand get_git_command(const char*);")


with open("src/git_cmd.cpp", "w") as sys.stdout:
    print('#include "git_cmd.h"')
    print()
    print("GitCommand get_git_command(const char *arg) {")
    with Switcher(0, git_subcommands):
        switch2(0, git_subcommands)
    print("return kNone;")
    print("}")
