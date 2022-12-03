#!/bin/bash

PRIORITY_TOTAL=0
SACK_INDEX=0
while read SACK
do
    SACK_UNIQ=`echo ${SACK} | fold -1 | sort | uniq `
    SACK_GROUP[${SACK_INDEX}]=${SACK_UNIQ}

    ((SACK_INDEX++))
    if (( ${SACK_INDEX} > 2 ))
    then
	SACK_INDEX=0
	GROUP_ITEM=`echo ${SACK_GROUP[@]} | fold -1 | sort | uniq -c | fgrep -w 3 |  tr -d [0-9] | tr -d ' '  `
	ITEM_ASC_VAL=`echo ${GROUP_ITEM} | tr -d '\n' | od -An -t uC`
    
	# Z = 90, A = 65, needs to be 27-52
	#  z = 122, a = 97, needs to be 1-26
	if (( ${ITEM_ASC_VAL} > 96 ))
	then
		PRIORITY=$((ITEM_ASC_VAL - 96))
	else
		PRIORITY=$((ITEM_ASC_VAL - 38))
	fi
	((PRIORITY_TOTAL+=${PRIORITY}))
   fi
    
done < input.txt

echo ${PRIORITY_TOTAL}

