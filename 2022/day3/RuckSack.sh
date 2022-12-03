#!/bin/bash

PRIORITY_TOTAL=0
while read SACK
do
    NUM_ITEMS=`echo ${SACK} | tr -d '\n' | wc -c`
    POUCH_LEN=$((NUM_ITEMS/2))

    POUCH1=`echo ${SACK} | cut -b 1-${POUCH_LEN} | fold -1 | sort | uniq `
    POUCH2=`echo ${SACK} | cut -b $((POUCH_LEN+1))- | fold -1 | sort | uniq `
    ITEM_ASC_VAL=`echo $POUCH1 $POUCH2 | fold -1 | sort | uniq -c | fgrep -w 2 | tr -d [0-9] | tr -d ' '  | tr -d '\n' | od -An -t uC`
  
    # Z = 90, A = 65, needs to be 27-52
    # z = 122, a = 97, needs to be 1-26
    if (( ${ITEM_ASC_VAL} > 96 ))
    then
	PRIORITY=$((ITEM_ASC_VAL - 96))
    else
	PRIORITY=$((ITEM_ASC_VAL - 38))
    fi
    
    ((PRIORITY_TOTAL+=${PRIORITY}))
done < input.txt

echo ${PRIORITY_TOTAL}

