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

	get_child_weights(root)

	search()

}

function search() {

	for (v in all_seen) {
		name = all_seen[v]
		print name
		if (name in children) {
			print "has children"

			delete f
			n = 0
			for (n_ch in child_weights[name]) {
				ch_w = child_weights[name][n_ch]
				print n_ch " " ch_w
				f[ch_w] += 1
				n += 1
			}

			for (freq in f) {
				print freq ": " f[freq]

				if (f[freq] == 1) {
					for (c in children[name]) {
						child_name = children[name][c]
						print " " child_name " = " child_weights[name][child_name]
						if (weights[children[name][c]] == freq)
							print "-> "

					}
				}
			}

		}

		
	}

}

# recursive weight function
# problems with awk global scoping :P
function get_child_weights(name)
{
#	print "recursing for " name
	if (name in children) {
		combined_weight[name] = weights[name]
		for (w in children[name]) {
			child = children[name][w]
#			print "child -> " child
			the_weight = get_child_weights(child)
			child_balanced = 1

			combined_weight[name] += the_weight

			child_weights[name][child] = the_weight
		}

		return combined_weight[name]
	} else {
		return weights[name]
	}
}
