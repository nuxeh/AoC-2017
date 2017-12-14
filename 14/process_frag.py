#!/usr/bin/env python

from process_hash import knot_hash_string

t = knot_hash_string("flqrgnkx")

def hash_to_bit_map(hash_string):
	ret = ""
	for c in hash_string:
		ret += '{0:04b}'.format(int(c, 16))
	return ret

def count_ones(string):
	count = 0
	for c in string:
		if c == "1":
			count += 1
	return count

theinput = "nbysizxe"

used = 0
for i in range(128):
	istr = theinput + "-" + str(i)
	khash = knot_hash_string(istr)
	bitmap = hash_to_bit_map(khash)
	used += count_ones(bitmap)

print used
