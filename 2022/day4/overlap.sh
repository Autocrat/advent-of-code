#!/bin/bash

OVERLAP_COUNT=0
while read LINE
do
    SET1=`echo ${LINE} | cut -d',' -f1`
    SET2=`echo ${LINE} | cut -d',' -f2`

    SET1A=`echo ${SET1} | cut -d'-' -f1`
    SET1B=`echo ${SET1} | cut -d'-' -f2`

    SET2A=`echo ${SET2} | cut -d'-' -f1`
    SET2B=`echo ${SET2} | cut -d'-' -f2`

    if (( ${SET1A} >= ${SET2A} && ${SET1A} <= ${SET2B} ))
    then
	((OVERLAP_COUNT++))	
    elif (( ${SET1B} <= ${SET2B} && ${SET1B} >= ${SET2A} ))
    then
	((OVERLAP_COUNT++))
    elif (( ${SET2A} >= ${SET1A} && ${SET2A} <= ${SET1B} ))
    then
	((OVERLAP_COUNT++))
    elif (( ${SET2B} <= ${SET1B} && ${SET2B} >= ${SET1A} ))	  
    then
	((OVERLAP_COUNT++))
    fi
done < input.txt
echo ${OVERLAP_COUNT}
