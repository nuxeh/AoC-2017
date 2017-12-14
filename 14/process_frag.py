#!/usr/bin/env python

from process_hash import knot_hash_string

t = knot_hash_string("flqrgnkx")

def hash_to_bit_map(hash_string):
	ret = ""
	for c in hash_string:
		ret += '{0:b}'.format(int(c, 16))
	return ret

print t

u = hash_to_bit_map(t)
print u
print len(u)
