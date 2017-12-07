#!/usr/bin/awk -f

BEGIN {
	FS=","
}

{
	all_seen[NR] = $1

	for (i=3; i<=NF; i++)
	{
		has_parent[$i] = 1
		children[$1][i] = $i
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
		if (has_parent[all_seen[v]] == 0) {
			print "root -> " all_seen[v]
			root = all_seen[v]
		}
	}

	for (v in children) {

		print v

		print "weight: " get_child_weights(v)
	}

	pos = root

	get_child_weights(pos)

}

# recursive weight function
# problems with awk global scoping :P
function get_child_weights(name)
{
	if (name in children) {
		combined_weight[name] = weights[name]
		for (w in children[name]) {
			child = children[name][w]
#			print "child -> " child
			the_weight = get_child_weights(child)
			combined_weight[name] += the_weight

			child_weights[name][child] = the_weight

			# check for non-matching weights
			for (x in child_weights[name]) {
				if (child_weights[name][x] != the_weight) {
					print "unbalanced!"
					string = "["
					for (y in child_weights[name]) {
						string = string " " child_weights[name][y]
					}
					print string " ] -> " child_weights[name][y] - the_weight
				}
			}
		}


		return combined_weight[name]
	} else {
		return weights[name]
	}
}
