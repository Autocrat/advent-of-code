#!/bin/bash

ENCOMPASS_COUNT=0
while read LINE
do
    SET1=`echo ${LINE} | cut -d',' -f1`
    SET2=`echo ${LINE} | cut -d',' -f2`

    SET1A=`echo ${SET1} | cut -d'-' -f1`
    SET1B=`echo ${SET1} | cut -d'-' -f2`

    SET2A=`echo ${SET2} | cut -d'-' -f1`
    SET2B=`echo ${SET2} | cut -d'-' -f2`

    if (( ${SET1A} <= ${SET2A} && ${SET1B} >= ${SET2B} ))
    then
#	echo "1 includes 2: $SET1 $SET2"
	((ENCOMPASS_COUNT++))
    elif (( ${SET2A} <= ${SET1A} && ${SET2B} >= ${SET1B} ))
    then
#	echo "1 includes 2: $SET1 $SET2"
	((ENCOMPASS_COUNT++))
    fi
done < input.txt
echo ${ENCOMPASS_COUNT}
