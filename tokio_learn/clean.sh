for dir in `ls`; do
    cd $dir;
    for dir2 in `ls`; do
        cd $dir2;
        cargo clean;
        cd ../;
    done
    cd ../;
done