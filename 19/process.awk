BEGIN {
	FS=""
}

{
	for (f=1; f<=NF; f++)
		map[NR][f] = $f
}

END {

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
