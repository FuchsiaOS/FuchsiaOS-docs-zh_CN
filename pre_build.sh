ls >list.txt
mkdir docs
while IFS= read -r line; do
    cp -R $line docs
done <list.txt
