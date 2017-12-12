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

	walk(0, 0)

}

function walk(n, depth, p) {
	print n
	if (depth > max_depth)
		max_depth = depth

	for (p in pipes[n]) {
		walk(p, depth+1)
	}
}
