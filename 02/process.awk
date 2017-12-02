BEGIN {
	sum = 0;
	sum_b = 0;
}

{
	max = -1;
	min = -1;

	for (i = 1; i <= NF; i++)
	{
		if ($i > max || max == -1)
			max = $i;
		if ($i < min || min == -1)
			min = $i;

		for (j = 1; j <= NF; j++)
		{
			if ($i % $j == 0 && j != i) {
				print $i " " $j
				print "result " $i / $j;
				sum_b += $i / $j;
			}
		}
	}

	print $0
	print "difference " max - min " max " max " min " min;

	sum += max - min;
}

END {
	print "checksum A is " sum;
	print "checksum B is " sum_b;
}
