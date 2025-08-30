cmake_minimum_required(VERSION 3.16)

# Project name and version
project(Relipmoc VERSION 1.0.0 LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Compiler flags for better debugging and warnings
if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU" OR CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
    # set(CMAKE_CXX_FLAGS_DEBUG "-g -O0 -Wall -Wextra -Wpedantic")
    set(CMAKE_CXX_FLAGS_DEBUG "-g -O0")
    set(CMAKE_CXX_FLAGS_RELEASE "-O3 -DNDEBUG")
endif()

# Set default build type to Debug if not specified
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Debug)
endif()

# Include directories - using modern target-based approach
set(RELIPMOC_INCLUDE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/include)

# Collect source files (implementation files only)
file(GLOB_RECURSE ALL_SOURCES "src/*.cpp")

# Separate main.cpp from library sources
set(MAIN_SOURCE "")
set(LIB_SOURCES "")

foreach(source ${ALL_SOURCES})
    get_filename_component(filename ${source} NAME)
    if(filename STREQUAL "main.cpp")
        set(MAIN_SOURCE ${source})
    else()
        list(APPEND LIB_SOURCES ${source})
    endif()
endforeach()

# Collect header files for IDE organization
file(GLOB_RECURSE HEADERS "include/relipmoc/*.hpp")

# Create library if there are library sources
if(LIB_SOURCES)
    add_library(${PROJECT_NAME}_lib ${LIB_SOURCES} ${HEADERS})
    target_include_directories(${PROJECT_NAME}_lib 
        PUBLIC 
            ${RELIPMOC_INCLUDE_DIR}
    )
endif()

# Create executable
if(MAIN_SOURCE)
    add_executable(${PROJECT_NAME} ${MAIN_SOURCE})
    target_include_directories(${PROJECT_NAME} 
        PRIVATE 
            ${RELIPMOC_INCLUDE_DIR}
    )
    
    # Link library if it exists
    if(LIB_SOURCES)
        target_link_libraries(${PROJECT_NAME} ${PROJECT_NAME}_lib)
    endif()
else()
    # If no main.cpp, create executable with all sources
    add_executable(${PROJECT_NAME} ${ALL_SOURCES} ${HEADERS})
    target_include_directories(${PROJECT_NAME} 
        PRIVATE 
            ${RELIPMOC_INCLUDE_DIR}
    )
endif()

# Optional: Enable testing
option(BUILD_TESTS "Build test programs" OFF)

if(BUILD_TESTS)
    enable_testing()
    add_subdirectory(tests)
endif()

# Print build information
message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")
message(STATUS "C++ standard: ${CMAKE_CXX_STANDARD}")
message(STATUS "Include directory: ${RELIPMOC_INCLUDE_DIR}")
message(STATUS "Source files: ${SOURCES}")
message(STATUS "Header files: ${HEADERS}")
