#!/usr/bin/awk -f

BEGIN {
	FS=","
}

{
	all_seen[NR] = $1

	for (i=3; i<=NF; i++)
	{
		has_parent[$i] = 1
	}

	weights[$1] = $2
}

END {
	print "weights:"
	for (v in weights) {
		print "     " v " => " weights[v]
	}
	print "have parents:"
	for (v in has_parent) {
		print "     " v " => " has_parent[v]
	}
	for (v in all_seen) {
		if (has_parent[all_seen[v]] == 0)
			print all_seen[v]
	}
}
