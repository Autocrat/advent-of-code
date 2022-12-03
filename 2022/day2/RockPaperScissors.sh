#!/bin/bash

function getShape() {
    SHAPE=ROCK
    case ${1} in
	[AX])
	    SHAPE=ROCK
	    ;;
	[BY])
	    SHAPE=PAPER
	    ;;
	[CZ])
	    SHAPE=SCISSORS
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


function compareShapes() {
    SCORE=0
    getShape ${1}
    LEFT_SHAPE=${SHAPE}
    getShape ${2}
    RIGHT_SHAPE=${SHAPE}
    getScore ${RIGHT_SHAPE}

    if [[ ${LEFT_SHAPE} == ${RIGHT_SHAPE} ]]
    then
	echo TIE
	RESULT=TIE
	((SCORE+=3))
    elif [[ ${LEFT_SHAPE} == ROCK && ${RIGHT_SHAPE} == PAPER ]]
    then
	echo ${RIGHT_SHAPE} beats ${LEFT_SHAPE}
	((SCORE+=6))
    elif [[ ${LEFT_SHAPE} == SCISSORS && ${RIGHT_SHAPE} == ROCK ]]
    then
	echo ${RIGHT_SHAPE} beats ${LEFT_SHAPE}
	((SCORE+=6))
    elif [[ ${LEFT_SHAPE} == PAPER && ${RIGHT_SHAPE} == SCISSORS ]]
    then
	echo ${RIGHT_SHAPE} beats ${LEFT_SHAPE}
	((SCORE+=6))
    else
	echo ${RIGHT_SHAPE} loses to ${LEFT_SHAPE}
    fi
}

TOTAL_SCORE=0
while read COLUMN1 COLUMN2
do
    compareShapes ${COLUMN1} ${COLUMN2}
    ((TOTAL_SCORE+=${SCORE}))
    echo ${TOTAL_SCORE}
done < foo.txt

echo ${TOTAL_SCORE}

#Cheating way
SCORE=0
while read FOOLINE
do
    case ${FOOLINE} in
	'A X')
	    ((SCORE+=4))
	    ;;
	'A Y')
	    ((SCORE+=8))
	    ;;
	'A Z')
	    ((SCORE+=3))
	    ;;
	'B X')
	    ((SCORE+=1))
	    ;;
	'B Y')
	    ((SCORE+=5))
	    ;;
	'B Z')
	    ((SCORE+=9))
	    ;;
	'C X')
	    ((SCORE+=7))
	    ;;
	'C Y')
	    ((SCORE+=2))
	    ;;
	'C Z')
	    ((SCORE+=6))
	    ;;
    esac
done < foo.txt
echo $SCORE
