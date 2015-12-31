#!/bin/bash

function config() {
	source "functions.sh"
	set -o nounset
	return 0
}

function main() {
	if config; then
		if check-privileges; then
			./uninstall.sh && ./install.sh
		else
			$(which sudo) ./uninstall.sh && ./install.sh
		fi
	fi
}

main "$@"

