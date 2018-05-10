cmake_minimum_required (VERSION 2.8)

include_directories (
	dependencies/hlsdk/dlls
	dependencies/hlsdk/engine
	dependencies/hlsdk/common
	dependencies/hlsdk/public
	dependencies/metamod_p/metamod
)

set (LIBNAME cppmetastub)
set (CMAKE_BUILD_TYPE Debug)
set (FILES proxy/main.cpp)

project (${LIBNAME})
add_library (${LIBNAME} MODULE ${FILES})
set_target_properties (${LIBNAME} PROPERTIES PREFIX "")
