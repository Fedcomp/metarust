#include <extdll.h>
#include <meta_api.h>

meta_globals_t *gpMetaGlobals;
mutil_funcs_t	 *gpMetaUtilFuncs;
globalvars_t	 *gpGlobals;
enginefuncs_t   g_engfuncs;

plugin_info_t info = {
	META_INTERFACE_VERSION,	        // ifvers
	(char *) "CPPMetaStub",					// name
	(char *) "0.1",									// version
	(char *) "30.06.2017",				  // date
	(char *) "Fedcomp",							// author
	(char *) "http://amx-x.ru",	    // url
	(char *) "CPPMETA",							// logtag, all caps please
	PT_CHANGELEVEL,					// (when) loadable
	PT_CHANGELEVEL					// (when) unloadable
};

static META_FUNCTIONS gMetaFunctionTable =
{
	NULL,				// pfnGetEntityAPI				    HL SDK; called before game DLL
	NULL,				// pfnGetEntityAPI_Post			  META; called after game DLL
	NULL,				// pfnGetEntityAPI2				    HL SDK2; called before game DLL
	NULL,				// pfnGetEntityAPI2_Post		  META; called after game DLL
	NULL,				// pfnGetNewDLLFunctions	    HL SDK2; called before game DLL
	NULL,				// pfnGetNewDLLFunctions_Post	META; called after game DLL
	NULL,				// pfnGetEngineFunctions		  META; called before HL engine
	NULL				// pfnGetEngineFunctions_Post	META; called after HL engine
};

C_DLLEXPORT void GiveFnptrsToDll(enginefuncs_t* pengfuncsFromEngine, globalvars_t *pGlobals)
{
	memcpy(&g_engfuncs, pengfuncsFromEngine, sizeof(enginefuncs_t));
	gpGlobals = pGlobals;
}

C_DLLEXPORT int Meta_Query(char *interfaceVersion, plugin_info_t **pinfo, mutil_funcs_t *pMetaUtilFuncs)
{
	*pinfo = &info;
	// gpMetaUtilFuncs = pMetaUtilFuncs;
	return TRUE;
}

C_DLLEXPORT int Meta_Attach(PLUG_LOADTIME now, META_FUNCTIONS *pFunctionTable, meta_globals_t *pMGlobals, gamedll_funcs_t *pGamedllFuncs)
{
	// memcpy(pFunctionTable, &gMetaFunctionTable, sizeof(META_FUNCTIONS));
	// gpMetaGlobals = pMGlobals;

	return TRUE;
}

C_DLLEXPORT int Meta_Detach(PLUG_LOADTIME now, PL_UNLOAD_REASON reason)
{
	return TRUE;
}
