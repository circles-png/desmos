# desmos

renders a bitmap image at ~0 fps with a desmos graph

## usage

```sh
# install it
cargo install --git https://github.com/circles-png/desmos

# run it
#
#    command   image path
#    |         |   max size (image will be scaled
#    |         |   |         so that the largest
#    |         |   |         dimension is this size)
desmos image.png 300

# copy the code from the file created (named "out", very big)
# and run the js code in the devtools console of a desmos graph

# on macos
cat out | pbcopy
```

## how it works

1. open the image from the given path
2. scale it
3. chunk it into 10,000-pixel chunks (desmos has a list length limit of 10,000 elements)
4. for each chunk
   1. transpose the pixels into columns of (x, y, hex colour) triplets
   2. make three expressions
      1. a table to store the data (triplets)
      2. a point function (x, -y) to plot the points, coloured with the hex colour
      3. a colour function to convert the hex colour to rgb
5. put the expressions into one big list
6. generate some js to write them into the desmos state
