#!/bin/bash

read LINE < input.txt
FOUND=FALSE
START_OFFSET=0

while [[ ${FOUND} != TRUE ]]
do
    TEST_WORD=${LINE:${START_OFFSET}:14}
    for((i=0;i<13;i++))
    do
	CHAR=${TEST_WORD:${i}:1}
	CHECK=${TEST_WORD//${CHAR}/}
	if (( ${#CHECK} < 13 )) #only 13 characters if there's no match, because you delete the one you're looking for
	then
	    break
	elif [[ ${i} == 12 ]] #make sure this is the last iteration of the loop
	then
	    FOUND=TRUE
	    echo Found at $((START_OFFSET+14))
	fi
    done
    ((START_OFFSET++))
done

