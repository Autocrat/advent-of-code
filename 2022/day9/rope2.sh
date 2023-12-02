#!/bin/bash
declare -A UNIQ_POSITIONS

for i in {0..9}
do
    ROPE_X[$i]=0
    ROPE_Y[$i]=0
done

function abs()
{
    if (( $1 < 0 ))
    then
	return $(( $1 * -1 ))
    else
	return $1
    fi
}


function moveNext()
{
    INDEX=$1
    PREV_INDEX=$(($INDEX - 1 ))

    DIST_X_O=$((${ROPE_X[$PREV_INDEX]} - ${ROPE_X[$INDEX]}))
    if (( $DIST_X_O < -1 ))
    then
	OFFSET_X=-1
    elif (( $DIST_X_O > 1 ))
    then
	OFFSET_X=1
    fi
	
    abs $DIST_X_O
    DIST_X=$?

    DIST_Y_O=$((${ROPE_Y[$PREV_INDEX]} - ${ROPE_Y[$INDEX]}))
    if (( $DIST_Y_O < -1 ))
    then
	OFFSET_Y=-1
    elif (( $DIST_Y_O > 1 ))
    then
	OFFSET_Y=1
    fi

    abs $DIST_Y_O
    DIST_Y=$?

#    echo Index: $INDEX
    if (( $DIST_X > 1 ))
    then
	X=$(( ${ROPE_X[$PREV_INDEX]} - $OFFSET_X ))
	Y=${ROPE_Y[$PREV_INDEX]}
#	echo 1 X: $X Y: $Y
    elif (( $DIST_Y > 1 ))
    then
	X=${ROPE_X[$PREV_INDEX]}
	Y=$(( ${ROPE_Y[$PREV_INDEX]} - $OFFSET_Y ))
#	echo 2 X: $X Y: $Y
    else
	X=${ROPE_X[$INDEX]}
	Y=${ROPE_Y[$INDEX]}
#	echo 3 X: $X Y: $Y
    fi

    ((INDEX++))
    if (( $INDEX < 10 ))
    then
	moveNext $INDEX
    else
	FOO="$X$Y"
	echo $FOO
	UNIQ_POSITIONS[$FOO]=FOO
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
		((ROPE_X[0]++))
		;;
	    L)
		((ROPE_X[0]--))
		;;
	    U)
		((ROPE_Y[0]++))
		;;
	    D)
		((ROPE_Y[0]--))
		;;
	esac
#	echo Index: 0
#	echo Moving: ${ROPE_X[0]} ${ROPE_Y[0]}
	moveNext 1
    done
}




while read -a LINE
do
    moveHead ${LINE[0]} ${LINE[1]}
done < input.txt
echo ${#UNIQ_POSITIONS[@]}
