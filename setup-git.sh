#!/bin/sh

#[rustfmt::skip]
FILES=./.rusty-hooks/*
for f in $FILES
do
    filename=$(basename -- "$f")
    echo "Processing $filename file..."
    
    printf "#!/bin/sh\n\n./.rusty-hooks/$filename \$1" > .git/hooks/$filename
    chmod +x .git/hooks/$filename
done;