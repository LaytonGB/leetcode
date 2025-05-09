# Read from the file file.txt and output the tenth line to stdout.
count=0
while read l && [ $count -le 10 ]; do
    let 'count = count + 1'
    if [ $count -eq 10 ]; then
        echo $l
        exit 0
    fi
done < file.txt