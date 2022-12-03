#!/bin/bash

function getShape() {
    case ${1} in
	[A])
	    SHAPE=ROCK
	    SHAPE_OPP_LOSE=SCISSORS
	    SHAPE_OPP_WIN=PAPER
	    ;;
	[B])
	    SHAPE=PAPER
	    SHAPE_OPP_LOSE=ROCK
	    SHAPE_OPP_WIN=SCISSORS
	    ;;
	[C])
	    SHAPE=SCISSORS
	    SHAPE_OPP_LOSE=PAPER
	    SHAPE_OPP_WIN=ROCK
	    ;;
    esac
}

function getWld() {
    WLD=LOSE
    case ${1} in
	[X])
	    WLD=LOSE
	    ;;
	[Y])
	    WLD=DRAW
	    ;;
	[Z])
	    WLD=WIN
	    ;;
    esac
}


function getScore() {
    case ${1} in
	ROCK)
	    SCORE=1
	    ;;
	SCISSORS)
	    SCORE=3
	    ;;
	PAPER)
	    SCORE=2
	    ;;
    esac
}


function useGuide() {
    SCORE=0
    getShape ${1}
    LEFT_SHAPE=${SHAPE}

    getWld ${2}

    if [[ ${WLD} == DRAW ]]
    then
	getScore ${LEFT_SHAPE}
	((SCORE+=3))
    elif [[ ${WLD} == WIN ]]
    then
	getScore ${SHAPE_OPP_WIN}
	((SCORE+=6))
    else
	 getScore ${SHAPE_OPP_LOSE}
    fi
}

TOTAL_SCORE=0
while read COLUMN1 COLUMN2
do
    useGuide ${COLUMN1} ${COLUMN2}
    ((TOTAL_SCORE+=${SCORE}))
done < foo.txt

echo ${TOTAL_SCORE}

