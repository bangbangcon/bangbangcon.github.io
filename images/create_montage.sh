#!/bin/bash

# Create temporary directory for images
mkdir -p /tmp/bangbangcon_images/resized/

# Copy speaker images to temp dir, in an order such that if a given
# speaker occurs more than once, newer images will overwrite older
# ones
cp ../2014/images/speakers/*.png /tmp/bangbangcon_images/
cp ../2015/images/speakers/*.png /tmp/bangbangcon_images/
cp ../2016/images/speakers/*.png /tmp/bangbangcon_images/

# Convert images to 50x50px
for image in `ls /tmp/bangbangcon_images/`; do
    convert /tmp/bangbangcon_images/$image -resize 50x50 /tmp/bangbangcon_images/resized/$image
done

# Make 4 copies of each (so we have original + 4) -- determined by experimentation
for image in `ls /tmp/bangbangcon_images/resized/`; do
    for ((i=1; i<=4; i++)); do
        cp /tmp/bangbangcon_images/resized/$image /tmp/bangbangcon_images/resized/$i$image
    done
done

# Arrange in a grid 9 images tall -- determined by experimentation
montage /tmp/bangbangcon_images/resized/*.png -tile x9 -geometry +0+0 montage.png

# Get rid of temp files
rm -r /tmp/bangbangcon_images/
