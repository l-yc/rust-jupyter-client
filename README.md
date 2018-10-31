# Literate python

A new interface for running interactive python code.

Taking a leaf out of the Rmd file format, this is a markdown file with
fenced code blocks. When the user's cursor is within the python blocks
and the "run cell" command is pressed, that cell is run and the output
is put below the inputs.

## Implementation

The frontend is an Elm application running with a golang web server. The
web server handles forwarding requests on to the kernel.
