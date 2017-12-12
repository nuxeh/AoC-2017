#!/usr/bin/awk -f

BEGIN {
	FS=" |, | <-> "
}

{
	for (i=2; i<=NF; i++)
	{
		print $1, $i
	}
	print "----"
}

END {

}
