BEGIN {
	sum = 0;
}

{
	max = -1;
	min = -1;

	for (i = 1; i <= NF; i++)
	{
#		print($i);
		if ($i > max || max == -1)
			max = $i;
		if ($i < min || min == -1)
			min = $i;
	}
#	print(max);
#	print(min);

	print $0
	print "difference " max - min " max " max " min " min;

	sum += max - min;
}

END {
	print "checksum is " sum ;
}
