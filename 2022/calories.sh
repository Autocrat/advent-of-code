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


for ((i=0;i<${#CALORIES[*]};i++))
do
    echo Elf number $i is carrying ${CALORIES[i]}
done

echo Elf ${MAX_CALORIES_ELF} is carrying the most with ${CALORIES[${MAX_CALORIES_ELF}]}
