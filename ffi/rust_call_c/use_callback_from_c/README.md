gcc -shared -fPIC extern.c -o libext.so 
LIBRARY_PATH=. cargo run



<!-- sudo cp libext.so /usr/lib/. -->
