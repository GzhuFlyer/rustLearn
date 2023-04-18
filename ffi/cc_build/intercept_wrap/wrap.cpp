extern "C" {
#include <libsyscall_intercept_hook_point.h>
}

template <class... Args>
inline long
syscall_no_intercept_wrapper(long syscall_number, Args... args) {
    long result;
    int error;
    result = syscall_no_intercept(syscall_number, args...);
    error = syscall_error_code(result);
    if(error != 0) {
        return -error;
    }
    return result;
}
