#!/bin/bash

# Deps: sudo

function config() {
	source "functions.sh"
	: ${INSTALL:=install -Dm766}
	: ${BUILD:=cargo build --release}

	set -o nounset
	if [[ ! -e "target/release/dybuk" || ! -e "target/release/deps" ]]; then
		return 1
	fi

	return 0
}

function build() {
	echo "N: Build command: \`$BUILD\`"
	$BUILD
	return $?
}

function try_to_install() {
	echo "N: Install \`$1\` into \`$2\`"
	if [[ "$EUID" != "0" ]]; then
		$(which sudo) $INSTALL "$1" "$2"
		return
	fi

	$INSTALL "$1" "$2"
}

function _install() {
	cd "target/release"
	try_to_install "dybuk" "/usr/local/bin/dybuk"
	while read file; do
		try_to_install "deps/$file" "/usr/local/lib/rustlib/dybuk/$file"
	done < <(find deps \( -type f -or -type l \) -printf '%P\n')
}

function main() {
	config

	case "$?" in
		0)
			_install
			;;

		1)
			echo 'E: Generated files do not exists.'
			if [[ "$EUID" == "0" ]]; then
				echo "N: Building the project with root privileges is prohibited."
				return 1
			fi

			if ask_yesno "Build project"; then
				build && _install
				return $?
			fi
			;;
	esac	
}

main "$@"

