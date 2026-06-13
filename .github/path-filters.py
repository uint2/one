from os import path

__rfile__ = path.realpath(__file__)
__cwd__ = path.dirname(__rfile__)
__root__ = path.dirname(__cwd__)

print("file:", __rfile__)
print("cwd:", __cwd__)
print("root:", __root__)

__file_rel__ = path.relpath(__rfile__, __root__)

OUTPUT_FILE = path.join(__cwd__, "path-filters.yml")

pairs = [
    ("aquarium", "code/games/aquarium"),
    ("canvas-sync-cli", "code/tools/canvas-sync-cli"),
    ("debian13", "code/docs/debian13-setup"),
    ("diff-rs", "code/tools/diff-rs"),
    ("draw-rs", "code/tools/draw-rs"),
    ("git-checkout2", "code/tools/git-checkout2"),
    ("git-ln", "code/tools/ln"),
    ("gitlab-api", "code/tools/gitlab-api"),
    ("gitnu-c", "code/tools/gitnu/c"),
    ("gitnu-rs", "code/tools/gitnu/rust"),
    ("helium", "code/tools/wacom-macos-precision-mode-gui"),
    ("heliumd", "code/tools/wacom-macos-precision-mode-daemon"),
    ("kopiwm", "code/tools/kopiwm"),
    ("loan-payoff-strategy", "code/tools/loan-payoff-strategy"),
    ("make-rs", "code/tools/make-rs"),
    ("numerical-methods", "code/tools/numerical-methods"),
    ("personal-site", "code/web/site"),
    ("rofi-pdf-search", "code/tools/rofi-pdf-search"),
    ("solid-rect", "code/tools/solid-rect"),
    ("stats-calc", "code/tools/stats-calc"),
    ("sudoku", "code/games/sudoku"),
    ("t-runner", "code/tools/t-runner"),
    ("tailwind-rs", "code/tools/tailwind-rs"),
    ("tmux-fzf", "code/tools/tmux-fzf"),
    ("wordle", "code/games/wordle"),
]


def print2(*v, file):
    print(*v, file=file)
    print(*v)


with open(OUTPUT_FILE, "w") as f:
    for key, subpath in pairs:
        print2(f"{key}:", file=f)
        print2(f'  - "{subpath}/**/*"', file=f)
        print2(f"  - {__file_rel__}", file=f)
        abs_subpath = path.join(__root__, subpath)
        assert path.isdir(abs_subpath), "Subpath does not exist!\n" + abs_subpath
