#!/usr/bin/awk -f

BEGIN {
	FS = ""
	OFS = "\t"
}

{
	print "", "ro", "go", "gd", "rd"

	score = 0
	group_depth = 0
	rubbish_depth = 0

	rubbish_count = 0

	for (i=1; i<=NF; i++)
	{
		c = $i

		parse(c)

		print c, rubbish_open, group_open, group_depth, rubbish_depth
	}

	print "group depth:" group_depth
	print "rubbish depth: " rubbish_depth
	print "score: " score
	print rubbish_count " rubbish."
	print ""
}

END {
}

function parse(char)
{
	r1 = rubbish_open

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

	if (rubbish_open == 1 && c != "!" && r1 == 1)
		rubbish_count += 1
}
