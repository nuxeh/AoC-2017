#!/usr/bin/awk -f

BEGIN {
	FS = ""
	OFS = "\t"
	print "", "ro", "go", "gd", "rd"
}

{

	score = 0
	group_depth = 0
	rubbish_depth = 0

	for (i=1; i<=NF; i++)
	{
		c = $i

		parse(c)
		print c

		print "", rubbish_open, group_open, group_depth, rubbish_depth
	}

#	print "--------"
	print
	print group_depth ", " rubbish_depth
	print "score: " score
	print ""
	print "", "ro", "go", "gd", "rd"
#	print "--------"
}

END {
}

function parse(char)
{
	switch (c) {
		case "{":
			if (rubbish_open == 0) {
				group_depth += 1
				group_open = 1
				score += group_depth
			}
		break
		case "}":
			if (group_open == 1 && rubbish_open == 0)
				group_depth -= 1
		break
		case "<":
			if (rubbish_open == 0) {
				rubbish_depth += 1
				rubbish_open = 1
			}
		break
		case ">":
			if (rubbish_open == 1) {
				rubbish_depth -= 1
				rubbish_open = 0
			}
		break
		case "!":
			i += 1
		break
	}
}
