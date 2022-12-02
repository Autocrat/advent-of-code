#!/bin/bash

INPUTFILE=CalorieFile.txt

ELF_INDEX=0
CALORIES[${ELF_INDEX}]=0
MAX_CALORIES=0
MAX_CALORIES_ELF=0

while read -r LINE
do
    if [ -n "${LINE}" ]
    then
	((CALORIES[${ELF_INDEX}]+=$LINE))
    if [[ {${CALORIES[${ELF_INDEX}]}} > ${MAX_CALORIES} ]]
    then
        MAX_CALORIES={${CALORIES[${ELF_INDEX}]}}
        MAX_CALORIES_ELF=${ELF_INDEX}
    fi
    else
	CALORIES[$((++ELF_INDEX))]=0    
    fi
done < ${INPUTFILE}


echo ${CALORIES[@]} | tr ' ' '\n' | sort -n | tail -3 | tee top3.txt

TOP_THREE_TOTAL=0
while read LINE
do
    ((TOP_THREE_TOTAL+=${LINE}))
done < top3.txt
echo ${TOP_THREE_TOTAL}
