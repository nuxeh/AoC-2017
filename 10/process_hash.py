#!/usr/bin/env python

lengths = [97,167,54,178,2,11,209,174,119,248,254,0,255,1,64,190]
lengths_t = [3, 4, 1, 5]

def get_ascii(string):
	a = [ord(c) for c in string]
	return a + [17, 31, 73, 47, 23]

def do_hash(arr, llength=256, runs=1):
	pos = 0
	skip = 0
	a = [x for x in range(llength)]

	for run in range(runs):
		for length in arr:
			b = [0] * length
			for i in range(length):
				b[i] = a[(pos + i) % llength]

			b.reverse()

			for i in range(length):
				a[(pos + i) % llength] = b[i]

			pos = (pos + length + skip) % llength
			skip += 1

	print "result is", a[0] * a[1]

	x = [[x*16, x*16+15] for x in range(llength/16)]
	c = []
	for r in x:
		c.append(reduce(lambda x, y: x ^ y, a[r[0]:r[1]+1]))

	return c

def get_string(vals):
	string = ""
	for v in vals:
		string += '{:02x}'.format(v)
	return string

def go(string):
	return get_string(do_hash(get_ascii(string), 256, 64))

if __name__ == "__main__":
    do_hash(lengths_t, 5)
    do_hash(lengths)

    #print get_ascii("1,2,3")
    #print do_hash(get_ascii("1,2,3"), 256, 64)
    #print len(do_hash(get_ascii("1,2,3"), 256, 64))

    print go("")
    print go("AoC 2017")
    print go("1,2,3")
    print go("1,2,4")
    print go("97,167,54,178,2,11,209,174,119,248,254,0,255,1,64,190")
