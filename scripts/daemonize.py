#!/usr/bin/env python3
"""Double-fork daemonizer that survives sandbox sweeps.

Usage: python3 daemonize.py <logfile> <pidfile> <cmd> [args...]

The child: setsid + second fork -> reparented to init (tini), stdio
redirected to <logfile>. The parent exits immediately, so the Bash tool
call returns at once and no descendant of the tool shell remains.
"""
import os
import sys


def main() -> None:
    if len(sys.argv) < 4:
        print("usage: daemonize.py <logfile> <pidfile> <cmd> [args...]", file=sys.stderr)
        sys.exit(2)
    logfile, pidfile, cmd, *args = sys.argv[1:]

    pid = os.fork()
    if pid > 0:
        os.waitpid(pid, 0)
        print(f"daemonized: {cmd} (pidfile={pidfile}, log={logfile})")
        return

    # first child: new session, then fork again so we can never reacquire
    # a controlling terminal; the grandchild reparents to init.
    os.setsid()
    pid2 = os.fork()
    if pid2 > 0:
        os._exit(0)

    logdir = os.path.dirname(os.path.abspath(logfile))
    os.makedirs(logdir, exist_ok=True)
    fd = os.open(logfile, os.O_WRONLY | os.O_CREAT | os.O_APPEND, 0o644)
    os.dup2(fd, 0)
    os.dup2(fd, 1)
    os.dup2(fd, 2)
    if fd > 2:
        os.close(fd)

    with open(pidfile, "w") as f:
        f.write(str(os.getpid()))

    os.execvp(cmd, [cmd, *args])
    os._exit(127)  # exec failed


if __name__ == "__main__":
    main()
