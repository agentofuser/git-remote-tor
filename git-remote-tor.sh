#!/bin/sh -e

# Originally found at:
# https://trac.torproject.org/projects/tor/attachment/ticket/21227/git-remote-tor
# Author: nicoo

die() {
    echo "$0: fatal: " "$@" >&2
    exit 1
}

[ -n "$GIT_DIR" ] || die 'GIT_DIR not set'

TRANSPORT="$(echo "$2" | cut -d: -f1)"

exec torsocks git-remote-"${TRANSPORT}" "$@"
