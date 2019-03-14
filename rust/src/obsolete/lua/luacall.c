#include <stdio.h>
#include <stdlib.h>
#include <string.h>
 
#include <luacall.h>

// Lua state is kept as a global variable
static lua_State *L;

// global return code
int rc;

// Preliminary steps to use Lua APIs.
int luacall_initialize()
{
    // allocate state and check for success
    if ((L = luaL_newstate()) == NULL) 
    {
        return 1;
    }

    // load Lua libraries
    luaL_openlibs(L);

    return LUA_OK;
}

// Frees Lua handles.
void luacall_cleanup()
{
    lua_close(L);
}

// Loads the Lua script and compiles it.
int luacall_loadfile(const char *script)
{
    if ((rc = luaL_loadfile(L, script)) != LUA_OK)
    {
        return rc;
    }

    if ((rc = lua_pcall(L, 0, 0, 0)) != LUA_OK)
    {
        return rc;
    } 

    return LUA_OK;
}

// Returns the global name type
int luacall_getglobal(const char *func)
{
    return lua_getglobal(L, func);
}

// Calls the function with a string, returning a string. The returned string should be
// allocated in the caller, as a strcpy() call is made to copy the Lua string out of its
// own stack.
int luacall_func_string(const char *func, const char *arg,  char *ret)
{
    // the func name should be a function !
    if ((rc = lua_getglobal(L, func)) != LUA_TFUNCTION) 
    {
        return rc;       
    }
    lua_pushstring(L, arg);           

    if ((rc = lua_pcall(L, 1, 1, 0)) != LUA_OK)
    {
        return rc;
    }

    // get pointer on returned argument
    const char *local_ret = lua_tostring(L, -1);

    // need to copy the string off the Lua stack
    strcpy(ret, local_ret);

    return LUA_OK;

}

