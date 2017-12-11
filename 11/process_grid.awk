#!/usr/bin/awk -f

#      a            a
#      ^  b         |\ c
#      | /          | \
#      |/           | /
#       \           |/ b
#        \
#         c

BEGIN {
	RS=",|\n"
	x = y = 0
}

{
	frequency[$0] += 1

	switch ($0) {
		case "n": y += 1
		break
		case "s": y -= 1
		break
		case "ne": x += 1
		break
		case "sw": x -= 1
		break
		case "se": x += 1; y -= 1
		break
		case "nw": x -= 1; y += 1
		break
	}
}

END {
	print NR

	z = 0 - x - y
	print x, y, z

	for (f in frequency)
		print f ":" frequency[f]

	n = 0
}

function abs(v) {return v < 0 ? -v : v}
