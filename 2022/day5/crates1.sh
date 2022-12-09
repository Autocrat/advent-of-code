#!/bin/bash

STACK_INDEX=0
STATE=INITIAL
MAX_LENGTH=0
#read to find how many stacks
while read -r LINE
do
    FLATTENED=${LINE//' '/}
    LENGTH=${#FLATTENED}
    if [[ ${#LINE} > $MAX_LENGTH ]]
    then
	MAX_LENGTH=${#LINE}
    fi
    ((LENGTH--))
    case ${FLATTENED:LENGTH:1} in
	[0-9])
	    MAX_STACKS=${FLATTENED:LENGTH:1}
	    break
	    ;;
    esac
done < input.txt

#get the stacks, exit when gets to the moves
OFFSET=1
while IFS= read -r FOO
do
    STRING=${FOO/\n/}
    if [[ ${FOO:1:1} == [0-9] ]]
    then
	#first line that is not a crate line
	break
    fi
    for ((OFFSET=1,STACK_INDEX=1;OFFSET < $MAX_LENGTH;OFFSET+=4,STACK_INDEX++))
    do
	CRATE=${FOO:OFFSET:1}
	if [[ ${CRATE} = [A-Z] ]]
	then
	    STACK[${STACK_INDEX}]=${CRATE}${STACK[${STACK_INDEX}]}
	fi
    done
done < input.txt



START_COMMANDS=0
while read COMMANDS
do
    if [[ ${START_COMMANDS} = 1 ]]
    then
	#parse commands
	COMMAND_ARRAY=($COMMANDS)
	NUM_CRATES_TO_MOVE=${COMMAND_ARRAY[1]}
	SOURCE_STACK=${COMMAND_ARRAY[3]}
	DEST_STACK=${COMMAND_ARRAY[5]}

	for((MOVE_COUNT=0;MOVE_COUNT<${NUM_CRATES_TO_MOVE};MOVE_COUNT++))
	do
	    CURRENT_STACK=${STACK[${SOURCE_STACK}]}
	    CURRENT_STACK_SIZE=${#CURRENT_STACK}
	    if [[ $CURRENT_STACK_SIZE > 0 ]]
	    then
		((CURRENT_STACK_SIZE--))
	    fi

	    CRATE=${CURRENT_STACK:${CURRENT_STACK_SIZE}:1}
	    STACK[${DEST_STACK}]=${STACK[${DEST_STACK}]}${CRATE}
	    STACK[${SOURCE_STACK}]=${CURRENT_STACK:0:${CURRENT_STACK_SIZE}}
	done
	
    elif [[ -z ${COMMANDS} ]]
    then
	START_COMMANDS=1
    fi
done < input.txt

echo Top of Stacks
for i in ${STACK[@]}
do
    echo ${i: -1}
done
