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

	oofs = OFS
	OFS="\t"
	print $0, "", a, b, c, "", x, y
	OFS = oofs
}

/^n$/ { a += 1; y += 1 }
/^s$/ { a -= 1; y -= 1 }
/^ne$/ { b += 1; x += 1 }
/^sw$/ { b -= 1; x -= 1 }
/^se$/ { c += 1; x += 1; y -= 1 }
/^nw$/ { c -= 1; x -= 1; y += 1 }

{
	frequency[$0] += 1

	oofs = OFS
	OFS="\t"
	print $0, "", a, b, c, "", x, y
	OFS = oofs
	

#	# cube coordinates
#	switch ($0) {
#		case "n": a += 1
#		case "s": a -= 1
#		case "ne": b += 1
#		case "sw": b -= 1
#		case "se": c += 1
#		case "nw": c -= 1
#	}
#
#	# axial coordinates
#	switch ($0) {
#		case "n": y += 1
#		case "s": y -= 1
#		case "ne": x += 1
#		case "sw": x -= 1
#		case "se": x += 1; y -= 1
#		case "nw": x -= 1; y += 1
#	}
}

END {
	print NR, "moves"

	sum = 0
	for (f in frequency) {
		sum += frequency[f]
		print f ":\t" frequency[f]
	}
	print "sum", sum

	# --- #

	z = 0 - x - y
	print x, y, z, "=", x+y+z

	# --- #

	print abs(a), abs(b), abs(c)
	print (abs(a) + abs(b) + abs(c)) / 2
}

function abs(v) {return v < 0 ? -v : v}
