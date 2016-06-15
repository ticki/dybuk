#!/bin/bash

function config() {
	set -e
	cd "$(dirname "$BASH_SOURCE")"

	source "functions.sh"
	return "$?"
}

function main() {
	config "$@" || return "$?"
	while shift; do :; done

	if check-privileges; then
		./uninstall.sh
		./install.sh
	else
		"$(which sudo)" ./uninstall.sh
		"$(which sudo)" ./install.sh
	fi
}

main "$@"
exit "$?"
