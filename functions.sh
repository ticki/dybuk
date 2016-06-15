#!/bin/bash

inline() { echo -en "$*"; }
log_l() { inline ":: $*\n"; }
log_n() { inline "N: $*\n"; }
log_w() { inline "W: $*\n"; }
log_e() { inline "E: $*\n"; } 1>&2

function check-privileges() {
	[ "$EUID" = "0" ]
	return "$?"
}

function ask_yesno() {
	local READ_RESULT=""
	while [[ "$READ_RESULT" != "y" && "$READ_RESULT" != "n" ]]; do
		echo -n "$1 [yn]: "
		read READ_RESULT
		[ "$READ_RESULT" ] || READ_RESULT="yn"
	done

	[ "$READ_RESULT" == "y" ] && return "$?"
	! [ "$READ_RESULT" == "n" ] || return "$?"
}

