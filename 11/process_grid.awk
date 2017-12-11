#!/usr/bin/awk -f

BEGIN {
	RS=",|\n"
	n_s = 0
	ne_sw = 0
	se_nw = 0
}

{
#	print
	switch ($0) {
		case "n": n_s += 1
		break
		case "s": n_s -= 1
		break
		case "ne": ne_sw += 1
		break
		case "sw": ne_sw -= 1
		break
		case "se": se_nw += 1
		break
		case "nw": se_nw -= 1
		break
	}

	last_seen = $1
}

END {
	print n_s, ne_sw, se_nw

#	if (n_s > ne_sw && n_s > se_nw

	if (se_nw > 0) {
		print ne_sw % se_nw
		n_s += ne_sw - (ne_sw % se_nw)
		ne_sw = ne_sw % se_nw
		se_nw = 0
	}

	print abs(n_s) + abs(ne_sw) + abs(se_nw)

}

function abs(v) {return v < 0 ? -v : v}
