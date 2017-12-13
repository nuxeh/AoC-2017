#!/usr/bin/awk -f

BEGIN {
	FS=": |\n"
	max_range = 0
}

{
	d[$1] = $2
	p[$1] = 0

	max_depth = $1
	if ($2 > max_range)
		max_range = $2
}

END {
	for (t=0; t<=max_depth; t++)
		tick(t)
}

function tick(t) {
	for (l in d) {
		p[l] = (p[l] + 1) % d[l]
	}
	draw(t)
}

function draw(t, l, d) {
	print t
	for (l=0; l<max_range; l++) {
		string = ""
		for (d=0; d<=max_depth; d++) {
			if (d in p == 0)
				string = string "... "
			else if (p[d] == l)
				string = string "[S] "
			else
				string = string "[ ] "
		}
		print string
	}
	print ""
}
