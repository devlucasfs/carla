#ifndef LIB
#error "LIB define not found"
#endif

#define PP_CAT_RAW(a, b) a##b
#define PP_CAT(a, b) PP_CAT_RAW(a, b)

#define PP_STR_RAW(x) #x
#define PP_STR(x) PP_STR_RAW(x)

#define LIB_FN(name) PP_CAT(PP_CAT(lib_, LIB), name)

#define MAKE_ERROR_PRINTER_BY_LIB_X86_64(library)                       \
inline static void                                                      \
PP_CAT(PP_CAT(lib_, library), _error_print)(char *str)                  \
{   unsigned long len = 0;                                              \
    while(str[len] != '\0') len++;                                      \
    asm volatile(                                                       \
        "mov $1, %%rax\n"                                               \
        "mov $1, %%rdi\n"                                               \
        "mov %0, %%rsi\n"                                               \
        "mov %1, %%rdx\n"                                               \
        "syscall\n"                                                     \
        :                                                               \
        : "r"(str), "r"(len)                                            \
        : "rax", "rdi", "rsi", "rdx", "rcx", "r11", "memory"            \
    );                                                                  \
}

#define MAKE_ERROR_EXIT_BY_LIB_X86_64(library)                          \
inline static void                                                      \
PP_CAT(PP_CAT(lib_, library), _error_exit)(void)                        \
{   asm volatile(                                                       \
        "syscall"                                                       \
        :                                                               \
        : "a"(60L), "D"((long)-1)                                       \
        : "rcx", "r11", "memory"                                        \
    );                                                                  \
    __builtin_unreachable();                                            \
}

#define MAKE_ERROR_BY_LIB(library)                                      \
inline static void                                                      \
PP_CAT(PP_CAT(lib_, library), _e_error)(char *msg)                      \
{   PP_CAT(PP_CAT(lib_, library), _error_print)("[");                   \
    PP_CAT(PP_CAT(lib_, library), _error_print)(PP_STR(library));       \
    PP_CAT(PP_CAT(lib_, library), _error_print)(                        \
        "]: A runtime error occurred\n~ ");                             \
    PP_CAT(PP_CAT(lib_, library), _error_print)(msg);                   \
    PP_CAT(PP_CAT(lib_, library), _error_print)("\n");                  \
    PP_CAT(PP_CAT(lib_, library), _error_exit)();                       \
}

MAKE_ERROR_EXIT_BY_LIB_X86_64(LIB)
MAKE_ERROR_PRINTER_BY_LIB_X86_64(LIB)
MAKE_ERROR_BY_LIB(LIB)

#define runtime_error(library) PP_CAT(PP_CAT(lib_, library), _e_error)
