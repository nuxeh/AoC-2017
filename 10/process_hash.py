lengths = [97,167,54,178,2,11,209,174,119,248,254,0,255,1,64,190]
lengths_t = [3, 4, 1, 5]

def get_ascii(string):
	a = [ord(c) for c in string]
	return a + [17, 31, 73, 47, 23]

def do_hash(arr, llength=256, runs=1):
	pos = 0
	skip = 0
	a = [x for x in range(llength)]
	print a
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

do_hash(lengths_t, 5)
do_hash(lengths)


print get_ascii("1,2,3")
