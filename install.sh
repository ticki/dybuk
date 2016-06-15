#!/bin/bash

function config() {
	cd "$(dirname "$BASH_SOURCE")"

	[ "$DEBUG" ] && set -x
	source "functions.sh"

	: ${INSTALL:=install -Dm u=rwx,g=rx,o=rx}
	: ${BUILD:=cargo build --release}
}

function build() {
	log_n "Build command: \`$BUILD\`"

	$BUILD
	return "$?"
}

function try_to_install() {
	log_l "Install \`$1\` into \`$2\`"
	if ! [ "$EUID" = "0" ]; then
		"$(which sudo)" $INSTALL "$1" "$2"
		return "$?"
	fi

	$INSTALL "$1" "$2"
	return "$?"
}

function _install() {
	log_n "Install command: \`$INSTALL\`"

	cd "target/release"
	try_to_install "dybuk" "/usr/local/bin/dybuk"
	while read file; do
		try_to_install "deps/$file" "/usr/local/lib/rustlib/dybuk/$file"
	done < <(find deps \( -type f -or -type l \) -printf '%P\n')
}

function main() {
	config "$@" || return "$?"
	while shift; do :; done

	[[ ! -e "target/release/dybuk" || ! -e "target/release/deps" ]]
	case "$?" in
		0)
			_install
			;;

		1)
			log_e "Generated files doesn't exists."
			if [ "$EUID" = "0" ]; then
				log_e "Building the project with root privileges is prohibited."
				return 1
			fi

			if ask_yesno "Build project"; then
				build && _install
				return "$?"
			fi
			;;
	esac
}

main "$@"
exit "$?"

