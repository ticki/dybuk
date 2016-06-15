#!/bin/bash

function config() {
	cd "$(dirname "$BASH_SOURCE")"
	source "functions.sh"
	return "$?"
}

function rm_custom_format() {
	perl -pe "s/^r(.+) .(.+).$/:: R\1 '\2 '\./"
}

function main() {
	config "$@" || return "$?"
	while shift; do :; done

	if check-privileges; then
		rm -rvf /usr/local/bin/dybuk | rm_custom_format
		rm -rfv /usr/local/lib/rustlib/dybuk | rm_custom_format
	else
		exec "$(which "sudo")" "./$(basename "$BASH_SOURCE")"
	fi
}

main "$@"
exit "$?"

