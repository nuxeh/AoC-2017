#!/usr/bin/awk -f

BEGIN {
	FS=": |\n"
	max_range = 0
}

{
	r[$1] = $2	# r[d]
	p[$1] = 0	# p[d]

	max_depth = $1
	if ($2 > max_range)
		max_range = $2
}

END {
	draw(0)
	for (t=1; t<=max_depth; t++)
		tick(t)
}

function tick(t) {
	for (d in r) {
		p[d] = (p[d] + 1) % r[d]
	}
	draw(t)
}

function draw(t, l, d) {
	print t
	for (l=0; l<max_range; l++) {
		string = ""
		for (d=0; d<=max_depth; d++) {
			if (d in p == 0)
				if (l == 0)
					string = string "... "
				else
					string = string "    "
			else if (p[d] == l)
				string = string "[S] "
			else if (l < r[d])
				string = string "[ ] "
			else
				string = string "    "
		}
		print string
	}
	print ""
}
