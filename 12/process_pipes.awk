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

	walk(0, 1)

	for (p in seen)
		print p, ":", seen[p]

	print "max depth:", max_depth

	print length(seen), "members in 0's group"

}

function walk(n, depth, p) {
	seen[n] = 1

	if (depth > max_depth)
		max_depth = depth

	for (p in pipes[n]) {
		if (seen[pipes[n][p]] != 1)
			walk(pipes[n][p], depth+1)
	}
}
