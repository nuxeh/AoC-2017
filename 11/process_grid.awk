#!/usr/bin/awk -f

BEGIN {
	RS=",|\n"
	n_s = 0
	ne_sw = 0
	se_nw = 0
}

{
	frequency[$0] += 1

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
}

END {

	for (f in frequency)
		print f ":" frequency[f]

#	print ()

	print "n/s", n_s, "ne/sw", ne_sw, "se_nw", se_nw

	if (se_nw > 0) {
		n_s += ne_sw - (ne_sw % se_nw)
		ne_sw = ne_sw % se_nw
		se_nw = 0
	} else if (ne_sw > 0) {
		n_s += se_nw - (ne_sw % ne_sw)
		se_nw = ne_sw % ne_sw
		ne_sw = 0
	}

	print "n/s", n_s, "ne/sw", ne_sw, "se_nw", se_nw

	print abs(n_s) + abs(ne_sw) + abs(se_nw)

}

function abs(v) {return v < 0 ? -v : v}
