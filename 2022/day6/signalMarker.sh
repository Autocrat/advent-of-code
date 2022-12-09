#!/bin/bash

read LINE < input.txt
FOUND=FALSE
START_OFFSET=0

while [[ ${FOUND} != TRUE ]]
do
    TEST_WORD=${LINE:${START_OFFSET}:4}
    for((i=0;i<3;i++))
    do
	CHAR=${TEST_WORD:${i}:1}
	CHECK=${TEST_WORD//${CHAR}/}
	if (( ${#CHECK} < 3 )) #only 3 characters if there's no match, because you delete the one you're looking for
	then
	    break
	elif (( ${i} == 2 ))
	then
	    FOUND=TRUE
	    echo Found at $((START_OFFSET+4))
	fi
    done
    ((START_OFFSET++))
done

