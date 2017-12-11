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

	print a, b, c

	if (a > 0) {
		print "1:", c % a
		n = b + (c % a)
	} else if (b > 0) {
		print "2:", a % b
		n = c + (a % b)
	} else if (c > 0) {
		print "3:", b % c
		n = a + (b % c)
	}

	print (c % b)
	print ((c + b) - (c % b))

	print a + (b % c) + ((c+b) - (b%c))
	r = b % c
	n = ((b + c) - (b % c)) / 2
	print a + r + n

	

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
