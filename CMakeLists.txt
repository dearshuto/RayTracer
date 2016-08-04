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
include_directories(${CMAKE_SOURCE_DIR}/external/jpeg)

FILE(GLOB_RECURSE image_files
	${CMAKE_SOURCE_DIR}/include/image/*.hpp
	${CMAKE_SOURCE_DIR}/src/image/*.cpp
)

FILE(GLOB_RECURSE scene_files
	${CMAKE_SOURCE_DIR}/include/scene/*.hpp
	${CMAKE_SOURCE_DIR}/src/scene/*.cpp
)

SOURCE_GROUP(image FILES ${image_files})
SOURCE_GROUP(image FILES ${scene_files})

add_library(ray_tracer STATIC
	${image_files}
	${scene_files}
)

add_subdirectory(external/bullet)
add_subdirectory(external/jpeg)


# ここからC++11を有効にする
add_definitions("-Wall -std=c++11")
add_subdirectory(example)