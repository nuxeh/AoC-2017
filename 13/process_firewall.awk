#!/usr/bin/awk -f

BEGIN {
	FS=": |\n"
}

{
	print $1, $2
	levels[$1] = $2
}

END {

}
