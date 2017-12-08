#!/usr/bin/awk -f

{
	if (check_cond($5, $6, $7)) {
		if ($2 == "inc")
			r[$1] += $3
		else
			r[$1] -= $3
	}
}

function check_cond(reg, oper, val) {
	ret = 0
	switch (oper)
	{
		case ">":
			if (r[reg] > val) ret = 1; break
		case "<":
			if (r[reg] < val) ret = 1; break
		case ">=":
			if (r[reg] >= val) ret = 1; break
		case "<=":
			if (r[reg] <= val) ret = 1; break
		case "==":
			if (r[reg] == val) ret = 1; break
		case "!=":
			if (r[reg] != val) ret = 1; break
	}
	print "(" reg " (" r[reg] ") " oper " " val ") => " ret
	return ret
}

END {
	count = 0
	for (reg in r) {
		if (count == 0 || r[reg] > max)
			max = r[reg]
		count += 1
		print reg " => " r[reg]
	}
	print "max: " max}
