#!/bin/awk -f
BEGIN {sum=0}
{sum+=($1-$1%3)/3-2}
END {print sum}
