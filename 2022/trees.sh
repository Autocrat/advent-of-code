#!/bin/bash

ROW_INDEX=0
while read ROWS[${ROW_INDEX}]
do
    ((ROW_INDEX++))
done < input.txt

TEMP=${ROWS[0]}
NUM_COLS=${#TEMP}
NUM_ROWS=${#ROWS[@]}
((NUM_ROWS--))
SCORE=0
TOTAL_TREES=$((NUM_COLS*NUM_ROWS))

#echo Columns $NUM_COLS
#echo Rows $NUM_ROWS

#for((i=0;i<${NUM_ROWS};i++))
#do
#    echo ${ROWS[${i}]}
#done
#echo

function getxy()
{
    COLUMN_NUM=${1}
    ROW_NUM=${2}
    TEMP_ROW=${ROWS[${ROW_NUM}]}
    RETVAL=${TEMP_ROW:COLUMN_NUM:1}
    return ${RETVAL}
}

function checkUp()
{
    TEMPH=$1
    TEMPX=$2
    TEMPY=$3

    for((i=${TEMPY};i>0;i--))
    do
	((SCOREUP++))
	((TEMPY--))
	getxy ${TEMPX} ${TEMPY}
	HEIGHT=${?}
	if((${TEMPH} <= ${HEIGHT}))
	then
	    return 1
	fi
    done
 #   ((SCOREUP--))
    return 0
}


function checkDown()
{
    TEMPH=$1
    TEMPX=$2
    TEMPY=$3

    for((i=${TEMPY};i<${NUM_ROWS};i++))
    do
	((SCOREDOWN++))
	((TEMPY++))
	getxy ${TEMPX} ${TEMPY}
	HEIGHT=${?}
	if((${TEMPH} <= ${HEIGHT}))
	then
	    return 1
	fi
    done
    ((SCOREDOWN--))
    return 0
}



function checkRight()
{
    TEMPH=$1
    TEMPX=$2
    TEMPY=$3
    for((i=${TEMPX};i<${NUM_COLS};i++))
    do

	((SCORERIGHT++))
	((TEMPX++))
	getxy ${TEMPX} ${TEMPY}
	HEIGHT=${?}
	if((${TEMPH} <= ${HEIGHT}))
	then
	    return 1
	fi
    done
    ((SCORERIGHT--))
    return 0
}


function checkLeft()
{
    TEMPH=$1
    TEMPX=$2
    TEMPY=$3

    for((i=${TEMPX};i>0;i--))
    do
	((SCORELEFT++))
	((TEMPX--))
	getxy ${TEMPX} ${TEMPY}
	HEIGHT=${?}
	if((${TEMPH} <= ${HEIGHT}))
	then
	    return 1
	fi
    done
#    ((SCORELEFT--))
    return 0
}



NUM_ROWS_MINUS1=$((NUM_ROWS-1))
NUM_COLS_MINUS1=$((NUM_COLS-1))
VISIBLE_COUNT=$(( 2*NUM_ROWS_MINUS1 + 2*NUM_COLS_MINUS1))
BLOCKED_COUNT=0
#echo $NUM_ROWS_MINUS1 $NUM_COLS_MINUS1
for((y=1;y<${NUM_ROWS_MINUS1};y++))
do
   for((x=1;x<${NUM_COLS_MINUS1};x++))
   do

        BLOCKED_WAYS=0
	SCOREUP=0
	SCOREDOWN=0
	SCORERIGHT=0
	SCORELEFT=0
	FOUND=0
	getxy ${x} ${y}
	TREE_HEIGHT=${?}

	
	checkUp ${TREE_HEIGHT} ${x} ${y}
	RESULT=${?}
	if [[ $RESULT == 1 ]]
	then
	    ((BLOCKED_WAYS++))
	else
	    if [[ ${FOUND} = 0 ]]
	    then
		FOUND=1
		((VISIBLE_COUNT++))
	    fi
	fi

	checkDown ${TREE_HEIGHT} ${x} ${y}
	RESULT=${?}
	if [[ $RESULT == 1 ]]
	then
	    ((BLOCKED_WAYS++))
	else
	    if [[ ${FOUND} = 0 ]]
	    then
		FOUND=1
		((VISIBLE_COUNT++))
	    fi
	fi

	checkRight ${TREE_HEIGHT} ${x} ${y}
	RESULT=${?}
	if [[ $RESULT == 1 ]]
	then
	    ((BLOCKED_WAYS++))
	else
	    if [[ ${FOUND} = 0 ]]
	    then
		FOUND=1
		((VISIBLE_COUNT++))
	    fi
	fi

	checkLeft ${TREE_HEIGHT} ${x} ${y}
	RESULT=${?}
	if [[ $RESULT == 1 ]]
	then
	    ((BLOCKED_WAYS++))
	else
	    if [[ ${FOUND} = 0 ]]
	    then
		FOUND=1
		((VISIBLE_COUNT++))
	    fi
	fi


	#echo X: ${x} Y: ${y} H: ${TREE_HEIGHT} Up: $SCOREUP Down: $SCOREDOWN Right: $SCORERIGHT Left: $SCORELEFT
	TEMP_SCORE=$((${SCOREUP}*${SCOREDOWN}*${SCORELEFT}*${SCORERIGHT}))

	if((TEMP_SCORE > SCORE))
	then
	    SCORE=$TEMP_SCORE
	fi
   done
done


echo Visible: ${VISIBLE_COUNT}
#echo Blocked: ${BLOCKED_COUNT}
echo Score $SCORE
