#!/usr/bin/awk -f

BEGIN {
	FS=",|<->"
}

{
	for (i=3; i<=NF; i++)
	{
		print $0, $i
	}
}

END {

}
