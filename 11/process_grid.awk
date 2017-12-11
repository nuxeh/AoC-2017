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

	a = 0
	b = 0
	c = 0

	x = 0
	y = 0
}

{
	frequency[$0] += 1

	# cube coordinates
	switch ($0) {
		case "n": a += 1
		break
		case "s": a -= 1
		break
		case "ne": b += 1
		break
		case "sw": b -= 1
		break
		case "se": c += 1
		break
		case "nw": c -= 1
		break
	}

	# axial coordinates
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
	print NR, "moves"

	z = 0 - x - y
	print x, y, z, "=", x+y+z

	sum = 0
	for (f in frequency) {
		sum += frequency[f]
		print f ":\t" frequency[f]
	}
	print "sum", sum
}

function abs(v) {return v < 0 ? -v : v}
