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

		case "ne": if (last == "nw") { n_s += 1 } else { ne_sw += 1 }
		break
		case "sw": if (last == "sw") { n_s -= 1 } else { ne_sw -= 1 }
		break
		case "se": if (last == "sw") { n_s -= 1 } else { se_nw += 1 }
		break
		case "nw": if (last == "ne") { n_s += 1 } else { se_nw -= 1 }
		break
	}

	last = $1
}

END {
	print n_s, ne_sw, se_nw

#	if (n_s > ne_sw && n_s > se_nw
}
