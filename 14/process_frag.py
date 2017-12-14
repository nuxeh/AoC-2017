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

def get_groups(a):
	e = generate_empty_map()

	for y,row in enumerate(a):
		for x,col in enumerate(a[y]):
			print str(x) + ", " + str(y)


#        0,-1
#         |
# -1,0 - 0,0 - 1,0
#         |
#        0,1

def walk(x, y, i, a, e, depth):
	neighbours = [[0,-1], [0,1], [-1, 0], [1,0]]

	if e[y][x] != 0:
		return
	else:
		e[y][x] = 1

	for n in neighbours:
		if a[y+n[1]][x+n[0]] == 1:
			walk(x+n[0], y+n[1], i, a, e, depth+1)

#b = generate_bitmap(testinput)
e = generate_empty_map()
get_groups(e)

#print knot_hash_string("flqrgnkx-127")
