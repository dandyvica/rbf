#include <lua.h>
#include <lauxlib.h>
#include <lualib.h>

int luacall_initialize();
void luacall_cleanup();
int luacall_loadfile(const char *script);
int luacall_func_string(const char *func, const char *arg, char *ret);
int luacall_getglobal(const char *func);

