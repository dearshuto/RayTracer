PROJECT(RayTracer)
cmake_minimum_required(VERSION 3.0)

# インクルードフォルダを相対パスの検索対象に設定する
add_definitions("-I ${CMAKE_SOURCE_DIR}/include")

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/lib)

include_directories(${CMAKE_SOURCE_DIR}/include)
include_directories(${CMAKE_SOURCE_DIR}/external/bullet/src)
include_directories(${CMAKE_SOURCE_DIR}/external/eigen)
include_directories(${CMAKE_SOURCE_DIR}/external/image/include)

FILE(GLOB_RECURSE algorithm_files
	${CMAKE_SOURCE_DIR}/include/algorithm/*.hpp
	${CMAKE_SOURCE_DIR}/include/algorithm/*.h
	${CMAKE_SOURCE_DIR}/src/algorithm/*.cpp
)

FILE(GLOB_RECURSE scene_files
	${CMAKE_SOURCE_DIR}/include/scene/*.hpp
	${CMAKE_SOURCE_DIR}/src/scene/*.cpp
)

FILE(GLOB_RECURSE type_files
	${CMAKE_SOURCE_DIR}/include/type/*.hpp
	${CMAKE_SOURCE_DIR}/src/type/*.cpp
)

SOURCE_GROUP(algorithm FILES ${algorithm_files})
SOURCE_GROUP(scene FILES ${scene_files})
SOURCE_GROUP(type FILES ${type_files})

add_library(ray_tracer STATIC
	${algorithm_files}
	${scene_files}
	${type_files}
)

add_subdirectory(external/bullet)
add_subdirectory(external/image)

# ここからC++11を有効にする
add_definitions("-Wall -std=c++11")
add_subdirectory(example)
