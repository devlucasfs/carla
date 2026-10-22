#include "runa.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

Runa *__carla_runa_parser_constexpr;
char *__carla_runa_comptime_defines_expr = NULL;

void c_runa_print_core_function(Runa *t) {
    RunaValueFFI val = runa_peek_arg(t, 0);
    char *str = runa_value_to_string(t, val);
    printf("(print) Carla Runa runner said: %s\n", str);
    runa_optional(RUNA_FREE_STRING_BY_VALUE, runa_str_free, str, val);
    runa_value_free(val);
}

void c_runa_init(char *filename) {
    __carla_runa_parser_constexpr = runa_start();
    runa_push_function(__carla_runa_parser_constexpr, "print",
                        (runa_callback)c_runa_print_core_function, 1);
    runa_loadfile(__carla_runa_parser_constexpr, filename);
}

void c_runa_deinit() { runa_free(__carla_runa_parser_constexpr); }

void c_runa_starts(char *expr) {
    if( __carla_runa_comptime_defines_expr != NULL )
        free(__carla_runa_comptime_defines_expr);
    __carla_runa_comptime_defines_expr = strdup(expr);
}

bool c_runa_constexpr(char *expr) {
    const char before[] = "function constexpr()\n\tif ",
                after[] = " then return 1 end\nend\nconstexpr()\n";

    size_t length = strlen(expr) + strlen(__carla_runa_comptime_defines_expr) + 1;
    char *expression =
        (char *)malloc(length + sizeof(before) + sizeof(after) + 1);

    *expression = '\0';
    sprintf(expression, "%s\n%s%s%s",
            __carla_runa_comptime_defines_expr,
            before,
            expr,
            after
    );

    bool result = false;
    RunaValueFFI resolve = runa_loadstring(__carla_runa_parser_constexpr, expression);

    free(expression);
    runa_value_free(resolve);

    if( resolve.tag == runa_integer ) {
        result = resolve.data.integer;
        runa_value_free(resolve); // This isn't needded cuz the value isn't Rust allocated,
                                  // but that is good to follow the pattern
        return result;
    }

    return result;
}
