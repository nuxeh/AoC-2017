#!/usr/bin/env python

from process_hash import knot_hash_string

t = knot_hash_string("flqrgnkx")

def hash_to_bit_map(hash_string):
	ret = ""
	for c in hash_string:
		ret += '{0:04b}'.format(int(c, 16))
	return ret

# TODO optimise sum ones
def count_ones(string):
	count = 0
	for c in string:
		if c == "1":
			count += 1
	return count

theinput = "nbysizxe"
testinput = "flqrgnkx"

def generate_bitmap(seed):
	used = 0
	bitmap = []
	for i in range(128):
		istr = seed + "-" + str(i)
		khash = knot_hash_string(istr)
		bitstr = hash_to_bit_map(khash)
		used += count_ones(bitstr)

		bitmap.append([int(c) for c in list(bitstr)])

	print str(used) + " used bits"

	return bitmap

def generate_empty_map():
	m = []
	for i in range(128):
		m.append([0 for j in range(128)])
	return m

b = generate_bitmap(testinput)

def get_groups(a):
	e = generate_empty_map()

	for row in a:
		print row


get_groups(b)

#print knot_hash_string("flqrgnkx-127")
