#!/bin/bash

function config() {
	source "functions.sh"
	set -o nounset
	return 0
}

function main() {
	if config; then
		if check-privileges; then
			rm -rvf /usr/local/bin/dybuk | perl -pe "s/^r(.+) .(.+).$/N: R\1 '\2 '\./"
			rm -rfv /usr/local/lib/rustlib/dybuk | perl -pe "s/^r(.+) .(.+).$/N: R\1 '\2 '\./"
		else
			echo 'E: This script must be run with root privileges.'
			return 1
		fi
	fi
}

main "$@"

