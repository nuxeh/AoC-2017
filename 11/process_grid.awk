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
		case "n":
			n_s += 1
			print "add one"
		break
		case "s": n_s -= 1
		break
		case "ne": ne_sw += 1
			print "add one"
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
	print n_s, ne_sw, se_nw

#	if (n_s > ne_sw && n_s > se_nw
}
