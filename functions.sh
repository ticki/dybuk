#!/bin/bash

function check-privileges() {
	local ERR_CODE=""
	[[ "$EUID" == "0" ]] && ERR_CODE=0 || ERR_CODE=1
	return $ERR_CODE
}

function ask_yesno() {
	local READ_RESULT=""
	while [[ "$READ_RESULT" != "y" && "$READ_RESULT" != "n" ]]; do
		echo -n "$1 [yn]: "
		read READ_RESULT
		[[ "$READ_RESULT" ]] || READ_RESULT="yn"
	done

	if [[ "$READ_RESULT" == "y" ]]; then
		return 0
	elif [[ "$READ_RESULT" == "n" ]]; then
		return 1
	fi
}

