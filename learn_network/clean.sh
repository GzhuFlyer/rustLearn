for dir in `ls`; do
    cd $dir;
    echo $dir;
    for dir2 in `ls`; do
	echo "come to child dir";
	echo $dir2;
        cd $dir2;
        cargo clean;
        cd ../;
    done
    cd ../;
done
