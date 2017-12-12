#!/usr/bin/awk -f

BEGIN {
	FS=" |, | <-> "
	max_depth = 0
}

{
	for (i=2; i<=NF; i++)
	{
		pipes[$1][i-1] = $i
	}
}

END {

	for (p in pipes)
		walk(p, 1, p)

	for (r in seen) {
		print r
		for (p in seen[r]) {
			print r, ":", p, ":", seen[r][p]
		}
		print length(seen[r]), "members in " r "'s group"
	}

	print "max depth:", max_depth


}

function walk(n, depth, root,	p) {
	seen[root][n] = 1

	if (depth > max_depth)
		max_depth = depth

	for (p in pipes[n]) {
		if (seen[root][pipes[n][p]] != 1)
			walk(pipes[n][p], depth+1, root)
	}
}
