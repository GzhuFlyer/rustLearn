
1) docker prepare
sudo docker pull centos:centos7
sudo docker run -d -it centos:centos7
sudo docker exec -it {containerId} bash   # containerId is the output of previous step.

2)  install the update
yum check-update
yum install -y epel-release
yum install -y cmake3
yum install -y centos-release-scl
yum install -y devtoolset-9
yum install -y devtoolset-7
yum install -y git
yum install -y automake
yum install -y libtool
yum install -y numactl-devel
yum install -y libpsm2-devel
yum install -y rdma-core-devel
yum install -y libconfig-devel
yum install -y zlib-devel
yum install -y boost-devel


3) gcc and  gekkofs
cd /
mkdir repo
cd repo
scl enable devtoolset-9 bash
echo “source /opt/rh/devtoolset-9/enable” >>/etc/profile
git clone https://storage.bsc.es/gitlab/hpc/gekkofs.git
cd gekkofs
git submodule update --init

4) download and build dependancies
./scripts/dl_dep.sh deps
./scripts/compile_dep.sh deps/ /usr/local/

c-json 报错解决
ln -s /usr/bin/cmake3 /usr/bin/cmake
/home/fzw/workspace/bky/gekkofs/deps/json-c/build
cmake .. && make

burstfs 报错解决
source scl_source enable devtoolset-9
source scl_source enable llvm-toolset-7 
 yum install llvm-toolset-7  -y
export LIBCLANG_PATH=/opt/rh/llvm-toolset-7/root/usr/lib64

5) update the CMakeLists.txt
#important:  add -fpermissive to CMAKE_CXX_FLAGS_RELEASE.
#file: ../CMakeLists.txt
#diff is below.

--- a/CMakeLists.txt
+++ b/CMakeLists.txt
@@ -55,7 +55,7 @@ message(STATUS "[gekkofs] Build type: ${CMAKE_BUILD_TYPE}")

 # Compiler flags for various cmake build types
 set(WARNINGS_FLAGS "-Wall -Wextra --pedantic -Wno-unused-parameter -Wno-missing-field-initializers")
-set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -DNDEBUG -O3")
+set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -DNDEBUG -O3 -fpermissive")


6) build gekkofs
mkdir build
cd build
scl enable devtoolset-9 bash
cmake3 -DCMAKE_BUILD_TYPE=Release .. && make -j15
cmake3 -DCMAKE_BUILD_TYPE=Debug ..
make

7) run the gekkofs
mkdir fs_data
mkdir mount_path

./src/daemon/gkfs_daemon -r  $(pwd)/fs_data  -m $(pwd)/mount_path

8)prepare a write.py with content below:
with open("./build/mount_path/test", "w") as f:
	f.write("test message")

9) run the write.py with command,  then check the "fs_data" directory, the fs_data is created in step 7)
LD_PRELOAD=./src/client/libgkfs_intercept.so python write.py

export LIBGKFS_LOG=debug 
export GKFS_LOG_LEVEL=trace

export LIBGKFS_HOSTS_FILE="/home/frank/workspace/bky/gekkofs/build.debug/gkfs_hosts.txt" 
export LIBGKFS_LOG=all
export LIBGKFS_LOG_OUTPUT=$(pwd)/log/cli.log
export LD_PRELOAD=$(pwd)/src/client/libgkfs_intercept.so 

GKFS_DAEMON_LOG_PATH=$(pwd)/log/daemon.log ./src/daemon/gkfs_daemon -r  fs_data  -m mount_path

#Now:

#GKFS_LOG_LEVEL={off,critical,err,warn,info,debug,trace}
--auto-sm


export GKFS_DAEMON_LOG_PATH=$(pwd)/daemon.log
export GKFS_LOG_LEVEL=trace

 ./src/daemon/gkfs_daemon -r  $(pwd)/fs_data  -m $(pwd)/mount_path --auto-sm
  LIBGKFS_LOG=all LIBGKFS_LOG_OUTPUT=client.log LD_PRELOAD=./src/client/libgkfs_intercept.so  ./test -mkdir mount_path/hello

mkdir log

LIBGKFS_LOG=debug LIBGKFS_LOG_OUTPUT=./log/cli.log LD_PRELOAD=./src/client/libgkfs_intercept.so python write.py
LIBGKFS_LOG_SYSCALL_FILTER=epoll_wait,epoll_create LIBGKFS_LOG=all LIBGKFS_LOG_OUTPUT=client.log LD_PRELOAD=./src/client/libgkfs_intercept.so python write.py


rm -rf log/* && LIBGKFS_LOG_SYSCALL_FILTER=epoll_wait,epoll_create LIBGKFS_LOG=all LIBGKFS_LOG_OUTPUT=./log/cli.log LD_PRELOAD=./src/client/libgkfs_intercept.so python write.py

ls mount_path/


10,rust 安装
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

11,查看所有打开的进程以及系统可以打开的最大进程数量
所有进程已经打开的fd数量及允许的最大数量。查询语句：
/proc/sys/fs/file-nr

安装命令“yum install lsof”

执行命令“lsof -n |awk '{print $2}'|sort|uniq -c |sort -nr|more ”，等待运算结果，结果出来后，其中第一列展示的为打开的文件句柄个数，第二列为对应的进程号。


LD_PRELOAD=./target/debug/libintercept.so ls
./target/debug/daemon -m ./mountdir -r ./rootdir -P na+sm -l 12345-0

export GKFS_DAEMON_LOG_PATH=$(pwd)/daemon.log
export GKFS_LOG_LEVEL=trace
./src/daemon/gkfs_daemon -r  $(pwd)/fs_data  -m $(pwd)/mount_path --auto-sm


LIBGKFS_LOG=all LIBGKFS_LOG_OUTPUT=client.log LD_PRELOAD=./src/client/libgkfs_intercept.so  ./test -mkdir mount_path/hello  



#burstfs
 rm -rf /tmp/na_sm-* rootdir/  &&./target/debug/daemon -m ./mountdir -r ./rootdir -P na+sm -l 12345-0

 LD_PRELOAD=./target/debug/libintercept.so  ./opsFile -mkdir mountdir/hello
  LD_PRELOAD=./target/debug/libintercept.so  ./opsFile -mkdir mountdir/world
 LD_PRELOAD=./target/debug/libintercept.so  ./opsFile -mkdir mountdir/hello/world
LD_PRELOAD=./target/debug/libintercept.so ./opsFile -getdent mountdir/
LD_PRELOAD=./target/debug/libintercept.so  ./opsFile -rmdir mountdir/world
LD_PRELOAD=./target/debug/libintercept.so ./opsFile -getdent mountdir/

LD_PRELOAD=./target/debug/libintercept.so  ./opsFile -mk_fork mountdir/hello

  LD_PRELOAD=./target/debug/libintercept.so  ./opsFile -rmdir mountdir/hello

LD_PRELOAD=./target/debug/libintercept.so  ./opsFile -write mountdir/hello

# =======================
client库export
在burstfs目录下执行。
export LD_PRELOAD=$(pwd)/target/debug/libintercept.so
echo $BURSTFS_WORKSPACE
# +++++++++++++


# 自己测试通过的： 
## 1，非拦截目录下
```
 prove -rv /home/fzw/workspace/bky/pjdfstest-master/tests/mkdir/00.t
 prove -rv /home/fzw/workspace/bky/pjdfstest-master/tests/mkdir/01.t
 prove -rv /home/fzw/workspace/bky/pjdfstest-master/tests/mkdir/02.t
 prove -rv /home/fzw/workspace/bky/pjdfstest-master/tests/mkdir/04.t
 prove -rv /home/fzw/workspace/bky/pjdfstest-master/tests/mkdir/05.t
 prove -rv /home/fzw/workspace/bky/pjdfstest-master/tests/mkdir/06.t
 prove -rv /home/fzw/workspace/bky/pjdfstest-master/tests/mkdir/07.t
 prove -rv /home/fzw/workspace/bky/pjdfstest-master/tests/mkdir/08.t
 prove -rv /home/fzw/workspace/bky/pjdfstest-master/tests/mkdir/09.t
 prove -rv /home/fzw/workspace/bky/pjdfstest-master/tests/mkdir/10.t
  prove -rv /home/fzw/workspace/bky/pjdfstest-master/tests/mkdir/11.t
自己测试不通过的：
prove -rv /home/fzw/workspace/bky/pjdfstest-master/tests/mkdir/03.t
prove -rv /home/fzw/workspace/bky/pjdfstest-master/tests/mkdir/12.t
```