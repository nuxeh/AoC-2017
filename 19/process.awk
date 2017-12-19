BEGIN {
	FS=""
}

{
	for (f=1; f<=NF; f++) {
		map[NR][f] = $f
		been[NR][f] = 0;
		w = f;
	}

	h = NR;
}

END {
	draw()

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

	while (1) {
		step()
		draw()
	}
}

function step() {

	nx = x + opts[dir][0]
	ny = y + opts[dir][1]

	map[y][x] = "x"

	if (ny in map && nx in map[ny] && map[ny][nx] != " ") {
		x = nx
		y = ny
	} else {
	}

}

function draw(x, y) {
	ORS = ""
	for (y in map) {
		print y " "
		for (x in map[y]) {
			print map[y][x]
		}
		print "\n"
	}
	ORS="\n"
}
