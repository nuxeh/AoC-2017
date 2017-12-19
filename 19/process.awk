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

	#      0
	#      |
	#  3 --+-- 1
	#      |
	#      2

	dir = 2;

	while (1) {
		step()
		draw()
	}
}

function step() {
	switch (dir) {
	case 0:
			nx = x;		ny = y - 1
		break;
	case 1:
			nx = x + 1;	ny = y
		break;
	case 2:
			nx = x;		ny = y + 1
		break;
	case 3:
			nx = x - 1;	ny = y
		break;
	}

	map[y][x] = "x"

	if (map[ny][nx] != " ") {
		x = nx
		y = ny
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
