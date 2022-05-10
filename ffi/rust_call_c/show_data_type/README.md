export LIBRARY_PATH=$(pwd)

gcc -fPIC -shared testffi.c -o libtestffi.so

//gcc -shared  testffi.c -o libtestffi.so

