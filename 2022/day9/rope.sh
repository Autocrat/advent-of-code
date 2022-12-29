#!/bin/bash
#If using a newer version of bash, just put all the tail positions in an associative array
#and count them

#Orientation
#    +Y
# -X    +X
#    -Y   
    
HEAD_POS_X=0
HEAD_POS_Y=0

TAIL_POS_X=0
TAIL_POS_Y=0
TAIL_ARRAY_X[0]=0
TAIL_ARRAY_Y[0]=0
TAIL_POS_INDEX=1
TAIL_COORD_ARRAY=' 0 0|'
declare -A TAILS
function moveTail()
{
    #only have to move the tail 1 space at a time, at most
    #use the direction to figure out where to move it

#......
#......
#......
#....H.
#...T..

# Move U 1 becomes
    
#......
#......
#....H.
#....T.
#......

    TEMP_TAIL_POS_X=${TAIL_POS_X}
    TEMP_TAIL_POS_Y=${TAIL_POS_Y}

    TAIL_DIRECTION=${1}
    case $TAIL_DIRECTION in
	R)
	    TAIL_DISTANCE=$((HEAD_POS_X-TAIL_POS_X))
	    if(($TAIL_DISTANCE > 1))
	    then
		TAIL_POS_X=$((HEAD_POS_X-1))
		TAIL_POS_Y=${HEAD_POS_Y}
	    fi
	    ;;
	L)
	    TAIL_DISTANCE=$((HEAD_POS_X-TAIL_POS_X))
	    if(($TAIL_DISTANCE < -1))
	    then
	    	TAIL_POS_X=$((HEAD_POS_X+1))
	    	TAIL_POS_Y=${HEAD_POS_Y}
	    fi
	    ;;
	U)
	    #check vertical distance.  If the tail is more than 1 from the head,
	    #the new position should be just above/below the head
	    TAIL_DISTANCE=$((HEAD_POS_Y-TAIL_POS_Y))
	    if(($TAIL_DISTANCE > 1))
	    then
		TAIL_POS_Y=$((HEAD_POS_Y-1))
		TAIL_POS_X=${HEAD_POS_X}
	    fi
	    ;;
	D)
	    TAIL_DISTANCE=$((HEAD_POS_Y-TAIL_POS_Y))
	    if(($TAIL_DISTANCE < -1))
	    then
	    	TAIL_POS_Y=$((HEAD_POS_Y+1))
	    	TAIL_POS_X=${HEAD_POS_X}
	    fi
	    ;;
    esac

    if(( ${TEMP_TAIL_POS_X} != ${TAIL_POS_X} || ${TEMP_TAIL_POS_Y} != ${TAIL_POS_Y} ))
    then
	TAIL_ARRAY_X[${TAIL_POS_INDEX}]=${TAIL_POS_X}
	TAIL_ARRAY_Y[${TAIL_POS_INDEX}]=${TAIL_POS_Y}
	TAIL_COORD_ARRAY[${TAIL_POS_INDEX}]="${TAIL_POS_X} ${TAIL_POS_Y}|"
	((TAIL_POS_INDEX++))
	TAILS["${TAIL_POS_X} ${TAIL_POS_Y}|"]="FOO"
    fi
}

function moveHead()
{
    DIRECTION=${1}
    DISTANCE=${2}
#    echo Moving ${DIRECTION} ${DISTANCE} spaces
    for((i=0;i<${DISTANCE};i++))
    do
	case $DIRECTION in
	    R)
		((HEAD_POS_X++))
		;;
	    L)
		((HEAD_POS_X--))
		;;
	    U)
		((HEAD_POS_Y++))
		;;
	    D)
		((HEAD_POS_Y--))
		;;
	esac
	moveTail ${DIRECTION}
#	echo Head: X: ${HEAD_POS_X} Y: ${HEAD_POS_Y}      
#	echo Tail: X: ${TAIL_POS_X} Y: ${TAIL_POS_Y}
    done
}




while read -a LINE
do
    moveHead ${LINE[0]} ${LINE[1]}
done < input.txt

echo Associative array size: ${#TAILS[@]}

#convert to string
#walk the array
#do weird stuff to handle excess spaces that I don't understand
#replace every occurrence of array element with empty string (this is where spaces get weird)
#every time something is replaced, count it
#...
#profit


TAIL_ARRAY_STRING="${TAIL_COORD_ARRAY[@]}"
TAIL_ARRAY_STRING_LEN=${#TAIL_ARRAY_STRING}
echo $TAIL_ARRAY_STRING_LEN
TAIL_ARRAY_STRING_LEN_OLD=${TAIL_ARRAY_STRING_LEN}
VISIT_COUNT=0
for i in "${TAIL_COORD_ARRAY[@]}"
do
    FOO=${i##" "}
    FOO=" ${FOO}"
    TAIL_ARRAY_STRING="${TAIL_ARRAY_STRING//${FOO}}"
    TAIL_ARRAY_STRING_LEN=${#TAIL_ARRAY_STRING}
    if (( ${TAIL_ARRAY_STRING_LEN} != ${TAIL_ARRAY_STRING_LEN_OLD} ))
    then
	((VISIT_COUNT++))
    fi
    TAIL_ARRAY_STRING_LEN_OLD=${TAIL_ARRAY_STRING_LEN}
done
echo Longer version: $VISIT_COUNT
