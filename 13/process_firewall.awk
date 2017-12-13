#!/usr/bin/awk -f

BEGIN {
	FS=": |\n"
	max_range = 0
}

{
	r[$1] = $2	# r[d]
	p[$1] = 0	# p[d]
	dir[$1] = 1	# dir[d]

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
		update_position(d)
	}
	draw(t)
}

function update_position(d) {
	if (dir[d] > 0 && p[d] == r[d])
		dir[d] *= -1
	else if (dir[d] < 0 && p[d] == 0)
		dir[d] *= -1
	p[d] += dir[d]
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
