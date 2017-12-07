#!/usr/bin/awk -f

{
	gsub(",", "");
	gsub(/\(/, "");
	gsub(/\)/, "");

	for (i=1; i<=NF; i++)
	{
		if ($i != "->") {
			printf $i
			if (i < NF)
				printf ","
		}
	}
	printf "\n"
}
