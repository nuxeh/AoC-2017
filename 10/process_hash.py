lengths = [97,167,54,178,2,11,209,174,119,248,254,0,255,1,64,190]
#lengths = [3, 4, 1, 5]
llength = 5
llength = 256

pos = 0
skip = 0

a = [x for x in range(llength)]

print(a)

for length in lengths:
	print length
	b = [0] * length
	for i in range(length):
		b[i] = a[(pos + i) % llength]

	print "b ", b
	b.reverse()

	for i in range(length):
		a[(pos + i) % llength] = b[i]

	print "b ", b
	print "a ", a

	pos = (pos + length + skip) % llength
	skip += 1

	print skip, pos

print "result is", a[0] * a[1]
