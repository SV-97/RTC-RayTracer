#!/bin/bash
if [ ! -d ./pngs ]
then
    mkdir pngs
fi

for f in *.ppm
do
    filename=$(basename "$f" .ppm)
    if [ -f ./pngs/"$filename".png ]
    then
        echo "$filename already converted"
    else
        echo "converting $filename"
        pnmtopng "$filename".ppm > ./pngs/"$filename".png
    fi
done
