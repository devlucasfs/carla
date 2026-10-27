#ifndef DEVELOPMENT_H
#define DEVELOPMENT_H

#include <stdint.h>
#include "nashj_ref.h"

#define mangled(id) development_##id

void mangled(fill) (void *bytes, int32_t start, int32_t size);
void mangled(debug) (void *str);
void mangled(exit) (int code);

#endif
