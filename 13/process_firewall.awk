#!/usr/bin/awk -f

BEGIN {
	FS=": |\n"
	max_range = 0
	pos = -1

	v = 1

	bail = 0
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
	if (v) draw(-1)

	dly = 0
	for (t=0; t<=max_depth; t++)
		tick(t)
	print "total severity: ", total_severity
}

function tick(t, d) {
	# move the position
	if (--dly < 0)
		pos += 1

	if (v) draw(t)

	ret = check_collisions()

	if (bail && ret)
		return

	for (d in r) {
		update_position(d)
	}
}

function check_collisions(collided) {
	if (pos in p && p[pos] == 0)
	{
		print "collision at depth", pos "\n"
		total_severity += pos * r[pos]
		collided = 1
	}

	return collided
}

function update_position(d) {
	if		(dir[d] > 0 && p[d] == r[d] - 1)	dir[d] *= -1
	else if		(dir[d] < 0 && p[d] == 0)		dir[d] *= -1
	p[d] += dir[d]
}

function draw(t, l, d) {
	print t
	for (l=0; l<max_range; l++) {
		string = ""
		for (d=0; d<=max_depth; d++) {
			if (d == pos && l == 0) {
				s = "("
				t = ")"
				u = "(.)"
			} else {
				s = "["
				t = "]"
				u = "..."
			}
			if (d in p == 0)
				if (l == 0)
					string = string u " "
				else
					string = string "    "
			else if (p[d] == l)
				string = string s"S"t" "
			else if (l < r[d])
				string = string s" "t" "
			else
				string = string "    "
		}
		print string
	}

	p_string = " "
	for (d=0; d<=max_depth; d++) {
		if (d in p)
			p_string = p_string p[d] "   "
		else
			p_string = p_string "-   "
	}
	print p_string

	print ""
}
