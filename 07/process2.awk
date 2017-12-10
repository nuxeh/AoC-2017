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

	get_child_weights(root)
	print "---"
	get_child_weights("ljaktj")
	print "---"
	get_child_weights("vrgxe")
	print "---"

}

# recursive weight function
# problems with awk global scoping :P
function get_child_weights(name, w, bal)
{
#	print "recursing for " name
	if (name in children) {
		combined_weight[name] = weights[name]

		for (w in children[name]) {
			child = children[name][w]
#			print "child -> " child "(w=" w ")"
			the_weight = get_child_weights(child)
#			print "done child -> " child "(w=" w ")"
			child = children[name][w]
			child_balanced = 1

			combined_weight[name] += the_weight

			child_weights[name][child] = the_weight
#			print "[" name "][" child "]"
		}

		delete frequency
		for (w in child_weights[name]) {
			frequency[child_weights[name][w]] += 1
		}
		bal = 1
		for (w in frequency) {
#			print w " : " frequency[w]
			if (frequency[w] == 1) {
				bal = 0
				print "unbalanced at name " name
				for (j in child_weights[name]) {
					print j " : " child_weights[name][j]
					if (child_weights[name][j] == w) {
						print "unbalanced node is " j
						print "weight should be: " get_corrected_weight(frequency, w, j)
						#get_child_weights(j)
					}
				}
			}
		}
		if (bal) {
			print name " is balanced"
			balanced[name] = 1
		}

#		print "-> " name " --> " combined_weight[name]
		return combined_weight[name]
	} else {
#		print name " -> " weights[name]
		return weights[name]
	}
}

function get_corrected_weight(freq, w, name)
{
	for (i in freq) {
		if (i != w)
			other_weight = i
	}

	print other_weight, weights[name], w

	return weights[name] - (w - other_weight)
}
