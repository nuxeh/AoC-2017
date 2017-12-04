#!/usr/bin/awk -f

BEGIN {
	n_valid = 0
	n_invalid = 0
	n_valid_ana = 0
	n_invalid_ana = 0
	debug = 1
}

{
	valid = 1
	valid_ana = 1

	if(debug) print $0

	for (i=1; i <= NF; i++)
	{
		print i
		for (j=1; j <= NF; j++)
		{
			if (i == j) continue
			if ($i == $j) {
				if(debug) print "invalid: " $i
				valid = 0
				break
			}
		}
		if (valid == 0) break
	}

	if (valid) {
		if(debug) print "valid"
		n_valid += 1
	} else {
		if(debug) print "invalid"
		n_invalid += 1
	}

#	if (valid_ana) {
#		if(debug) print "valid"
#		n_valid_ana += 1
#	} else {
#		if(debug) print "invalid"
#		n_invalid_ana += 1
#	}

	for (k=1; k <= NF; k++)
	{
		print get_word_hash($k)
	}
}

END {
	print "A -> " n_valid " valid, " n_invalid " invalid passphrases in input"
	print "B -> " n_valid_ana " valid, " n_invalid_ana " invalid passphrases in input"
}

function get_word_hash(word, result)
{
	word = toupper(word)

	n = split(word, a, "")
	asort(a)

	for (i=1; i <= n; i++)
		result = result a[i]

	return result
}
