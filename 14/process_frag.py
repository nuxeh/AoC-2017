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
#		print [int(c) for c in list(bitstr)[0:8]]
		used += count_ones(bitstr)

		bitmap.append([int(c) for c in list(bitstr)])

	print str(used) + " used bits"

	return bitmap

# TODO: do this more fancily with list comprehensions
def generate_empty_map(w, h):
	m = []
	for i in range(h):
		m.append([0 for j in range(w)])
	return m


def get_groups(a):
	h = len(a)
	w = len(a[0])
	e = generate_empty_map(w, h)
	i = 1

	for y,row in enumerate(a):
		for x,col in enumerate(a[y]):
			if a[y][x] == 1:
				if not walk(x, y, i, a, e, w, h, 0):
					i += 1

	print_map(e)

	return i-1

# Recursive flood fill
#
#        0,-1
#         |
# -1,0 - 0,0 - 1,0
#         |
#        0,1
#
def walk(x, y, i, a, e, w, h, depth):
	neighbours = [[0,-1], [0,1], [-1, 0], [1,0]]

	if e[y][x] != 0:
		return 1
	else:
		e[y][x] = i

	for n in neighbours:
		next_x = x+n[0]
		next_y = y+n[1]

		if next_y >= 0 and next_y < h and next_x >=0 and next_x < w:
			if a[next_y][next_x] == 1:
				walk(next_x, next_y, i, a, e, w, h, depth+1)

	return 0

def print_map(a):
	print "map:"
	for y,row in enumerate(a):
		print a[y]

#e = generate_empty_map(10, 10)
#get_groups(e)

#print knot_hash_string("flqrgnkx-127")

test_map = [[1, 1, 0, 1, 0, 1, 0, 0],
            [0, 1, 0, 1, 0, 1, 0, 1],
            [0, 0, 0, 0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1, 1, 0, 1],
            [0, 1, 1, 0, 1, 0, 0, 0],
            [1, 1, 0, 0, 1, 0, 0, 1],
            [0, 1, 0, 0, 0, 1, 0, 0],
            [1, 1, 0, 1, 0, 1, 1, 0]]

print_map(test_map)
get_groups(test_map)

#b = generate_bitmap(testinput)
b = generate_bitmap(theinput)
print str(get_groups(b)) + " groups"



