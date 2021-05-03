#!python3
import subprocess
import getopt
import sys
import os
import platform
import shutil


def usage():
    print("USAGE: ./configure.sh [-d | --debug]")
    print("                      [-h | --help]")

def eval(s):
    return subprocess.run(s.split(' '),stdout=subprocess.PIPE, text=True).stdout.strip()

def run(s):
    print("Run " + s)
    if os.system(s) != 0:
        print("Exec " + s + 'failed.')
        exit(2)

def main(argumentList):
    is_debug = False
    cargo_path = 'cargo' 
    sqlx_path = 'sqlx'
    psql_path = 'psql'
    try:
        options = "dh"
        long_options = ["debug", "help"]
        arguments, values = getopt.getopt(argumentList, options, long_options)
        
        for currentArgument, currentValue in arguments:
            if currentArgument in ("-h", "--help"):
                usage()
                return
            elif currentArgument in ("-d", "--Debug"):
                is_debug = True             
    except getopt.error:
        usage()   

    try:
        cargo_path = eval('which cargo')
        sqlx_path = eval('which sqlx')
        psql_path = eval('which psql')
    except e:
        print(e)

    if cargo_path:
        print(f"CARGO PATH: {cargo_path}")
    else:
        print("cargo NOT FOUND")
        return
    if sqlx_path:
        print(f"SQLX PATH: {sqlx_path}")
    else:
        print("sqlx NOT FOUND")
        return
    if psql_path:
        print(f"PSQL PATH: {psql_path}")
    else:
        print("psql NOT FOUND")
        return
    print(f"DEBUG: {is_debug}")

    run(f"{sqlx_path} migrate run")
    if is_debug:
        run(f"{cargo_path} build -q")
    else:
        run(f"{cargo_path} build -q --release")
    
    target_path = os.path.join("./bin/")
    # print(target_path)
    os.makedirs("bin", exist_ok=True)

    if platform.system() == "Linux":
        tracker_lib = "libretracker.so"
    elif platform.system() == "Darwin":
        tracker_lib = "libretracker.dylib"
    else:
        print("Error: not support windows or other system!")
        return
    
    bins = ['sopt', 'sopt_proxy', tracker_lib]
    if is_debug:
        source_path = os.path.join("./target", "debug")
    else:
        source_path = os.path.join("./target", "release")
    for binary in bins:
        os.symlink(os.path.join(source_path, binary), os.path.join(target_path, binary))

argumentList = sys.argv[1:]
main(argumentList)
# $CARGO build --release
