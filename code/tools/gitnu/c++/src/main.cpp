#include "git_cmd.h"
#include <filesystem>
#include <iostream>
#include <memory>
#include <unistd.h>

#define MAX_ARGS 1024
#define ARG(i, str) args[i] = const_cast<char *>(str)

namespace fs = std::filesystem;

fs::path get_git_dir() {
    char buffer[256];
    std::unique_ptr<FILE, decltype(&pclose)> pipe(
        popen("git rev-parse --git-dir", "r"), pclose);
    if (!pipe) throw std::runtime_error("popen() failed");
    if (fgets(buffer, sizeof(buffer), pipe.get()) == nullptr)
        throw std::runtime_error("Error running git rev-parse --git-dir");
    return buffer;
}

std::string exec(const char *cmd) {
    char buffer[256];
    std::string result;
    std::unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
    if (!pipe) throw std::runtime_error("popen() failed");
    while (fgets(buffer, sizeof(buffer), pipe.get()) != nullptr)
        result += buffer;
    return result;
}

int main(const int argc, const char **argv) {
    // Make sure that we're dealing with non-trivial stuff.
    // After this line, we know that argc >= 2.
    if (argc <= 1) execlp("git", "git", NULL);

    // Build the base list of arguments.
    char *args[MAX_ARGS];
    args[0] = const_cast<char *>("git");
    for (short i = 1; i < MAX_ARGS; i++)
        args[i] = NULL;

    GitCommand subcommand = kNone;
    const char *subcommand_ = nullptr;

    for (int i = 0; i < argc; ++i) {
        if ((subcommand = get_git_command(argv[i])) != kNone) {
            subcommand_ = argv[i];
            std::cout << "Subcommand: " << subcommand_ << std::endl;
            break;
        }
    }

    fs::path git_dir(get_git_dir());

    printf("Git Directory: %s\n", git_dir.c_str());

    return 0;

    ARG(0, "git");
    ARG(1, "status");
    for (int i = 0; i < argc; i++) {
        std::cout << "[" << i << "] " << argv[i] << std::endl;
    }
    execvp("git", args);
}
