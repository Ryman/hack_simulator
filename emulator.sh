#!/usr/bin/env sh

# User's CDPATH can interfere with cd in this script
unset CDPATH
# Get the true name of this script
script="`test -L "$0" && readlink -n "$0" || echo "$0"`"
dir="$PWD"
cd "`dirname "$script"`"
if [ \( $# -gt 1 \) -o \( "$1" = "-h" \) -o \( "$1" = "--help" \) ]
then
    echo "Usage:"
    echo "    `basename "$0"` FILE.tst    Starts the CPU Emulator and runs the File.tst"
    echo "                               test script.  The success/failure message"
    echo "                               is printed to the command console."
elif [ $# -eq 0 ]
then
    echo "Requires a .tst file"
else
    # Convert arg1 to an absolute path and run CPU emulator with arg1
    if [ `echo "$1" | sed -e "s/\(.\).*/\1/"` = / ]
    then
        arg1="$1"
    else
        arg1="${dir}/$1"
    fi
#   echo Running "$arg1"
    ./target/release/hack_simulator --runner "$arg1"
fi
