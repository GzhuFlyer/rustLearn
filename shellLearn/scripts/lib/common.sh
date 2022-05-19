# 冒号没什么用，仅仅是占位符，这里做显示使用
# http://blog.chinaunix.net/uid-20182470-id-5840908.html
: "${__DEFAULT_CLUSTER:=minikube}"
# echo $__DEFAULT_CLUSTER
# echo ${__DEFAULT_CLUSTER:=minikube}

: "${__DEFAULT_CLUSTER:=minikube}"
: "${CLUSTER:=${__DEFAULT_CLUSTER}}"

: "${DROGUE_NS:=drogue-iot}"
: "${CONTAINER:=docker}"
: "${TEST_CERTS_IMAGE:=ghcr.io/drogue-iot/test-cert-generator:latest}"

die() {
    echo "$*" 1>&2
    false # exit to outer shell
}

bold() {
    tput bold || :
    echo "$@"
    tput sgr0 || :
}

progress() {
    echo "$@" >&3
    echo "$@" >>"$LOG"
}

is_default_cluster() {
    test "$__DEFAULT_CLUSTER" == "$CLUSTER"
}
