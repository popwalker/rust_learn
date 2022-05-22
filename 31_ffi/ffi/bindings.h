#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

const char *hello_world();

const char *hello_bad(const char *name);

const char *hello(const char *name);

void free_str(char *s);

} // extern "C"
