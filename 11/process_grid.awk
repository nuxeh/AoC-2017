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
}

{
	frequency[$0] += 1

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
}

END {

	for (f in frequency)
		print f ":" frequency[f]

	n = 0

	a = abs(a)
	b = abs(b)
	c = abs(c)

#	print a, b, c

#	r = b % c
#	n = ((b + c) - (b % c)) / 2
#	print a + r + n
#
#	x = a
#	y = b
#	z = c
#	r = y % z
#	n = ((y + z) - (y % z)) / 2
#	print x + r + n

	x = (c + b) - (2 * c)
	y = (a + c) - (2 * a)
	z = (b + a) - (2 * b)
	
#	print "x = " x

#	print x + (a - (2*x))

#	print a + (b + c) - (2 * c) + (2 * ((b + c) - (2 * c)))


#	a += x
#	b -= x

#	print a + b

	print a, b, c
	print x, y, z
#	print a+x, b+y, c+z
#	print a-x, b-y, c-z
#	print x-a, y-b, z-c


#	print "n/s", a, "ne/sw", b, "c", c
#
#	if (c > 0) {
#		a += b - (b % c)
#		b = b % c
#		c = 0
#	} else if (b > 0) {
#		a += c - (b % b)
#		c = b % b
#		b = 0
#	}
#
#	print "n/s", a, "ne/sw", b, "c", c
#
#	print abs(a) + abs(b) + abs(c)
#
}

function abs(v) {return v < 0 ? -v : v}
