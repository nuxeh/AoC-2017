BEGIN {
	FS=""
}

{
	for (f=1; f<=NF; f++) {
		map[NR][f] = $f
		w = f;
	}

	h = NR;
}

END {
#	draw()

	for (s in map[1])
		if (map[1][s] == "|")
			break

	print "width", w, "height", h
	print "starting x coordinate ", s

	x = s;
	y = 1;
	path = "";

	#      0             0,-1
	#      |              |
	#  3 --+-- 1  -1,0 --0,0-- 1,0
	#      |              |
	#      2             0,1

	dir = 2;

	opts[0][0] =  0
	opts[0][1] = -1
	opts[1][0] =  1
	opts[1][1] =  0
	opts[2][0] =  0
	opts[2][1] =  1
	opts[3][0] = -1
	opts[3][1] =  0

	while (step()) {
#		draw()
#		print x,y,dir
	}

	print "path:", path
}

function step() {

	ov = map[y][x]	# store old value
	map[y][x] = "x"	# mark as visited

	nx = x + opts[dir][0]
	ny = y + opts[dir][1]

	if (ov == "+") {
		for (nd in opts) {
#			print nd
			nx = x + opts[nd][0]
			ny = y + opts[nd][1]
			fv = map[ny][nx]
#			print fv

			if (fv != " " && fv != "x") {
				dir = nd
				break
			}
		}
#		print "+"
	}

	if (ov != "+" && ov != "|" && ov != "-" && ov != "x")
		path = path ov

	if (ny in map && nx in map[ny]) {
		x = nx
		y = ny
		return 1
	}

	return 0;
}

function draw(x, y) {
	ORS = ""
	for (y in map) {
		printf "%-5d ", y
		for (x in map[y]) {
			print map[y][x]
		}
		print "\n"
	}
	ORS="\n"
}

function count() {

	for (y in map) {
		for (x in map[y]) {
			freq[map[y][x]] += 1;
		}
	}

	return freq["x"]

}
