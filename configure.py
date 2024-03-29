#!python3
import subprocess
import getopt
import sys
import os
import platform
import shutil

system = platform.system()


def usage():
    print("USAGE: ./configure.py [-d | --debug]")
    print("                      [-h | --help]")


def run_command(s):
    if system == 'Windows':
        return subprocess.run(s.split(' '), stdout=subprocess.PIPE, text=True, shell=True).stdout.strip()
    else:
        return subprocess.run(s.split(' '), stdout=subprocess.PIPE, text=True).stdout.strip()


def find(s):
    if system == 'Windows':
        return run_command('where ' + s)
    else:
        return run_command('which ' + s)


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
        sqlx_path = find('sqlx')
        redis_path = find('redis-server')
    except Exception as e:
        print(e)

    if 'cargo_path' in locals() and cargo_path:
        print(f"CARGO PATH: {cargo_path}")
    else:
        raise Exception("Error: cargo NOT FOUND")
    if 'sqlx_path' in locals() and sqlx_path:
        print(f"SQLX PATH: {sqlx_path}")
    else:
        run(f"{cargo_path} install sqlx-cli")
        sqlx_path = find('sqlx')
    if 'redis_path' in locals() and redis_path:
        print(f"REDIS PATH: {redis_path}")
    else:
        raise Exception("Error: redis-server NOT FOUND")
    if system == "Windows":
        if 'minio.exe' not in run_command("dir minio").split():
            run("Invoke-WebRequest -Uri \"https://dl.min.io/server/minio/release/windows-amd64/minio.exe\" -OutFile minio.exe")
            run("setx MINIO_ROOT_USER admin")
            run("setx MINIO_ROOT_PASSWORD password")
    else:
        if 'minio' not in run_command("ls minio").split():
            run("wget https://dl.min.io/server/minio/release/linux-amd64/minio")
            run("chmod +x minio")
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

    if system == "Linux":
        tracker_lib = "libretracker.so"
    elif system == "Darwin":
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
    if system == "Windows":
        run("./minio server D:\minio_data --console-address \":9001\"")
    else:
        run("MINIO_ROOT_USER=admin MINIO_ROOT_PASSWORD=password ./minio server /mnt/data --console-address ':9001'")
    try:
        run("redis-server ./config/redis.conf &")
        run("./sopt &")
        run("./sopt_proxy &")
    except KeyboardInterrupt:
        os.kill("kill $(pgrep sopt)")


if __name__ == "__main__":
    main()
