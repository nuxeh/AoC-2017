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
#	walk(pos)

}

function walk(name) {
	if (name in children) {
		for (w in children[name]) {
			child = children[name][w]

			if (length(child_weights[name]) > 0) {

				string = name
				for (wgt in child_weights[name]) {
					string = string " " wgt
				}
				print string
			}

			if (children[name][w] in unbalanced)
			{

			} else {

			}

			walk(children[name])
		}
	}
}


# recursive weight function
# problems with awk global scoping :P
function get_child_weights(name)
{
	print "recursing for " name
	if (name in children) {
		combined_weight[name] = weights[name]
		for (w in children[name]) {
			child = children[name][w]
			print "child -> " child
			the_weight = get_child_weights(child)
			child_balanced = 1

			combined_weight[name] += the_weight

			child_weights[name][child] = the_weight

			print the_weight;

		}

		val = -1
		for (cw in child_weights[name]) {
			print cw " " child_weights[name][cw]
			if (val == -1)
				val = child_weights[name][cw]
			else if (child_weights[name][cw] != val)
				print "    unbalanced! " name
		}

#		return unbalance * combined_weight[name]
		return combined_weight[name]
	} else {
		return weights[name]
	}
}
