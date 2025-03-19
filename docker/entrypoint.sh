#!/bin/sh

# Allows Ctrl-C, by letting this sh process act as PID 1
exit_func() {
    exit 1
}
trap exit_func TERM INT

ulimit -n 65535

echo "Running: ${*}"
"$@"
