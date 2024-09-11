#!/bin/bash

# A script to generate the montage background image used in our 2019
# website.  Depends on ImageMagick and coreutils (shuf).

# Create temporary directory for images
mkdir -p /tmp/bangbangcon_images/

# Copy speaker images to temp dir, in an order such that if a given
# speaker occurs more than once, newer images will overwrite older
# ones
# -u to always update a speaker's photo
cp -u ../*/images/speakers/*.png /tmp/bangbangcon_images/

speakers=`ls /tmp/bangbangcon_images/ | wc -l`
echo
echo "There are $speakers speakers from past !!Cons!"
echo

# Create temporary directories for manipulated images
mkdir /tmp/bangbangcon_images/resized
mkdir /tmp/bangbangcon_images/final

# Convert images to 50x50px
i=1
for image in `ls /tmp/bangbangcon_images/*.png`; do
    convert $image -resize 50x50 /tmp/bangbangcon_images/resized/$i.png
    let "i++";
done

# Numbers determined by experimentation -- this is what looked good!
# If the number of source images changes, these will likely have to
# change too.
COPIES=3
GRID_HEIGHT=15
LEFTOVER_SLOTS=12 # number of spaces left over in grid

# Make copies and order images randomly
j=1
for ((i=1; i<=$COPIES; i++)); do
    for image in `ls /tmp/bangbangcon_images/resized/*.png | shuf`; do
        cp $image /tmp/bangbangcon_images/final/$j.png;
        let "j++";
    done
done

# Pick random images to fill in the gap at the end
k=1
for image in `ls /tmp/bangbangcon_images/final/*.png | shuf | head -n $LEFTOVER_SLOTS`; do
    #echo $image;
    cp $image /tmp/bangbangcon_images/final/extra_$k.png;
    let "k++";
done

# Arrange in a grid
montage /tmp/bangbangcon_images/final/*.png -tile x$GRID_HEIGHT -geometry +0+0 -quality 90 montage.jpg

# Get rid of temp files
rm -r /tmp/bangbangcon_images/
