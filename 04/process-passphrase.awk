#!/usr/bin/awk -f

BEGIN {
	n_valid = 0
	n_invalid = 0
	debug = 1
}

{
	if(debug) print $0

	for (i=1; i <= NF; i++)
	{
		for (j=1; j <= NF; j++)
		{
			if (i == j) {
				if(debug) print "invalid"
				n_invalid += 1
			} else {
				if(debug) print "valid"
				n_valid += 1
			}
		}
	}
}


END {
	print n_valid " valid, " n_invalid " invalid passphrases in input"


}
