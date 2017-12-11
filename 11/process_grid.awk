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
}

/^n$/	{ b += 1; c -= 1;	y += 1 }
/^s$/	{ b -= 1; c += 1;	y -= 1 }
/^ne$/	{ a += 1; c -= 1;	x += 1 }
/^sw$/	{ a -= 1; c += 1;	x -= 1 }
/^se$/	{ a += 1; b -= 1;	x += 1; y -= 1 }
/^nw$/	{ a -= 1; b += 1;	x -= 1; y += 1 }

{
	frequency[$0] += 1
	print $0, "", a, b, c, "", x, y
}

END {
	OFS = oofs

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
