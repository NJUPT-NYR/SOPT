#!python3
import subprocess
import getopt
import sys
import os
import platform
import shutil


def usage():
    print("USAGE: ./configure.py [-d | --debug]")
    print("                      [-h | --help]")


def eval(s):
    if platform.system().lower() == 'windows':
        return subprocess.run(s.split(' '), stdout=subprocess.PIPE, text=True, shell=True).stdout.strip()
    else:
        return subprocess.run(s.split(' '), stdout=subprocess.PIPT, text=True).stdout.strip()

def find(s):
    if platform.system().lower() == 'windows':
        return eval('where ' + s)
    else:
        return eval('which ' + s)


def run(s):
    print("Run " + s)
    if os.system(s) != 0:
        print("Exec " + s + 'failed.')
        exit(2)


def copyTree(src, dst, symlinks=False, ignore=None):
    for item in os.listdir(src):
        s = os.path.join(src, item)
        d = os.path.join(dst, item)
        if os.path.isdir(s):
            shutil.copytree(s, d, symlinks, ignore)
        else:
            shutil.copy2(s, d)


def main():
    argumentList = sys.argv[1:]
    is_debug = False
    try:
        options = "dh"
        long_options = ["debug", "help"]
        arguments, values = getopt.getopt(argumentList, options, long_options)

        for currentArgument, currentValue in arguments:
            if currentArgument in ("-h", "--help"):
                usage()
                return
            elif currentArgument in ("-d", "--debug"):
                is_debug = True
    except getopt.error:
        usage()

    try:
        cargo_path = find('cargo')
        # if cargo_path == '':
        #    exit(-1)
        sqlx_path = find('sqlx')
        redis_path = find('redis-server')
    except Exception as e:
        print(e)

    if 'cargo_path' in locals():
        print(f"CARGO PATH: {cargo_path}")
    else:
        raise Exception("Error: cargo NOT FOUND")
    if 'sqlx_path' in locals():
        print(f"SQLX PATH: {sqlx_path}")
    else:
        run(f"{cargo_path} install sqlx-cli")
        sqlx_path = find('sqlx')
    if 'redis_path' in locals():
        print(f"REDIS PATH: {redis_path}")
    else:
        raise Exception("Error: redis-server NOT FOUND")
    print(f"DEBUG: {is_debug}\n")

    run(f"{sqlx_path} migrate run")

    if is_debug:
        run(f"{cargo_path} build -q")
    else:
        run(f"{cargo_path} build -q --release")
    target_path = os.path.join("./bin/")
    os.makedirs("bin", exist_ok=True)

    if is_debug:
        source_path = os.path.join("./target", "debug")
    else:
        source_path = os.path.join("./target", "release")

    if platform.system() == "Linux":
        tracker_lib = "libretracker.so"
    elif platform.system() == "Darwin":
        tracker_lib = "libretracker.dylib"
    else:
        raise Exception("Error: not support windows or other system!")

    bins = ['sopt', 'sopt_proxy', tracker_lib]
    for binary in bins:
        shutil.copy(os.path.join(source_path, binary), target_path)

    config_path = os.path.join(target_path, "config/")
    shutil.copytree(os.path.join("./config/"), config_path, dirs_exist_ok=True)
    shutil.copy(".env", target_path)

    os.chdir(target_path)
    run("redis-server ./config/redis.conf &")
    run("./sopt &")
    run("./sopt_proxy &")


if __name__ == "__main__":
    main()
