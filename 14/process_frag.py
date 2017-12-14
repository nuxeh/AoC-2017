#!/usr/bin/env python

from process_hash import knot_hash_string

print knot_hash_string("flqrgnkx")

print ord("e")
print int("e", 16)
print '{0:b}'.format(int("e", 16))

for c in "aoc2017":
	print ord(c)
#	print int(c)
#	print '{0:b}'.format(c)
