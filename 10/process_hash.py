lengths = [97,167,54,178,2,11,209,174,119,248,254,0,255,1,64,190]
lengths = [3, 4, 1, 5]
llength = 4

pos = 0
skip = 0

a = [x for x in range(llength)]

print(a)

#b = [x for x in 
for length in lengths:
	b = [0] * length
	for i in range(length):
		b[i] = a[(pos + i) % llength]

	b.reverse()

	for i in range(length):
		a[(pos + i) % llength] = b[i]

	print length
	print b

	pos += (pos + length + skip) % llength
	skip += 1
