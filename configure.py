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
    return subprocess.run(s.split(' '),stdout=subprocess.PIPE, text=True).stdout.strip()

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
        cargo_path = eval('which cargo')
        sqlx_path = eval('which sqlx')
        redis_path = eval("which redis-server")
    except e:
        print(e)

    if cargo_path:
        print(f"CARGO PATH: {cargo_path}")
    else:
        print("Error: cargo NOT FOUND")
        return
    if sqlx_path:
        print(f"SQLX PATH: {sqlx_path}")
    else:
        run(f"{cargo_path} install sqlx-cli")
        sqlx_path = eval("which sqlx")
    if redis_path:
        print(f"REDIS PATH: {redis_path}")
    else:
        print("Error: redis-server NOT FOUND")
        return
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
        print("Error: not support windows or other system!")
        return
    
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
