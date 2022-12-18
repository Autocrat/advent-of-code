#!/bin/bash


function findDirIndex() {
    DIR_INDEX=0
    for i in ${DIRECTORIES[@]}
    do
        if [[ "$1" = "$i" ]]
        then
#            echo Found $1 at $DIR_INDEX
            break
        else
            ((DIR_INDEX++))
        fi
    done
    return ${DIR_INDEX}
}



CURR_DIR=""
while read -r LINE
do
    #check for cd to root
    case "${LINE}" in
        '$ ls')
#            echo ls command
            ;;
        '$ cd'*)
            COMMAND=($LINE)
#           echo Change dir ${COMMAND[2]}
            if [[ ${COMMAND[2]:0:1} =~ [a-z/] ]]
            then
#               echo Change directory to ${COMMAND[2]} 
                CURR_DIR="${CURR_DIR}_${COMMAND[2]}"
                findDirIndex "${CURR_DIR}"
                DIRECTORIES[${DIR_INDEX}]="${CURR_DIR}"
            else
                CURR_DIR=${CURR_DIR%'_'*}
            fi
#            echo "Current Directory: ${CURR_DIR}"
            ;;
        'dir'*)
#            echo Found $LINE
            ;;
        [0-9]*)
#            echo Found file $LINE
            DIR_FILES[${DIR_INDEX}]="${DIR_FILES[${DIR_INDEX}]} ${LINE}"
            ;;
        *)
            echo Something else: $LINE
            ;;
    esac
done < input.txt

NUM_DIRS=${#DIRECTORIES[@]}
#echo $NUM_DIRS
for((i=0;i<${NUM_DIRS};i++))
do
#    echo ${DIRECTORIES[$i]}
#    echo "Files: ${DIR_FILES[$i]}"
    FILES=${DIR_FILES[$i]}
    FILE_ARRAY=($FILES)
    DIR_SIZE=0
    for((j=0;j<${#FILE_ARRAY[@]};j+=2))
    do
        ((DIR_SIZE+=FILE_ARRAY[${j}]))
    done
    DIR_SIZE_ARRAY[${i}]=${DIR_SIZE}
done


# for((i=0;i<${NUM_DIRS};i++))
# do
#     echo Dir: ${DIRECTORIES[$i]}
#     echo "Files: ${DIR_FILES[$i]}"
#     echo LocalDirSize: ${DIR_SIZE_ARRAY[${i}]}
# done


for((i=0;i<${NUM_DIRS};i++))
do
    TOTAL_SUBDIR_SIZE[${i}]=0
    #if this directory's substing was found, add it to the total
    CURR_NAME=${DIRECTORIES[${i}]}
    for((j=0;j<${NUM_DIRS};j++))
    do
#	echo TGC  ${TOTAL_SUBDIR_SIZE[${i}]}
	#compre this directory's name with all of the others
	TEMP1=${DIRECTORIES[${j}]}
	TEMP1_LEN=${#TEMP1}
#	echo SubdirName: $TEMP1 SubdirNameLen: $TEMP1_LEN
	TEMPFOO_NAME=${TEMP1#${CURR_NAME}}
#	echo "Searching for $CURR_NAME in $TEMP1 ($TEMP1_LEN) Result is: $TEMPFOO_NAME ( ${#TEMPFOO_NAME} )"
	TEMPFOO_LEN=${#TEMPFOO_NAME}
#	echo Total DirSize: ${DIR_SIZE_ARRAY[${j}]}
	if(( ${TEMPFOO_LEN} < ${TEMP1_LEN} ))
	then
#	    echo Adding ${TOTAL_SUBDIR_SIZE[${i}]} with ${DIR_SIZE_ARRAY[${j}]}
	    ((TOTAL_SUBDIR_SIZE[${i}]+=${DIR_SIZE_ARRAY[${j}]}))
	fi
    done
#    echo Total Subdir size $CURR_NAME ${TOTAL_SUBDIR_SIZE[${i}]}
done

echo '**************************************'
SUM_TOTAL=0
TOTAL_USED_SPACE=${TOTAL_SUBDIR_SIZE[0]}
FREE_SPACE=0
((FREE_SPACE=70000000-TOTAL_USED_SPACE))
#echo Used: $TOTAL_USED_SPACE  Free: $FREE_SPACE
DELETE_SIZE=${TOTAL_SUBDIR_SIZE[0]}
DELETE_INDEX=0
for((i=0;i<${NUM_DIRS};i++))
do
#    echo ${DIRECTORIES[${i}]} has  ${TOTAL_SUBDIR_SIZE[${i}]}
    if((${TOTAL_SUBDIR_SIZE[${i}]} <= 100000))
    then
#	echo ${DIRECTORIES[${i}]} qualifies
	((SUM_TOTAL+=${TOTAL_SUBDIR_SIZE[${i}]}))
    fi

    ((NEED_SIZE=30000000-FREE_SPACE))
    if((${TOTAL_SUBDIR_SIZE[${i}]} >= ${NEED_SIZE} ))
    then
	if((${TOTAL_SUBDIR_SIZE[${i}]} < ${DELETE_SIZE}))
	then
	    DELETE_INDEX=${i}
	    DELETE_SIZE=${TOTAL_SUBDIR_SIZE[${i}]}
#	    echo Setting Deletion Candidate: Index: ${i} ${DIRECTORIES[${i}]} ${DELETE_SIZE}
	fi
    fi
done
echo Part1 $SUM_TOTAL
echo Part2 $DELETE_SIZE
