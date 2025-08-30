# Add tests subdirectory only if BUILD_TESTS is enabled
if(BUILD_TESTS)
    # Find required packages for testing (optional)
    # find_package(GTest REQUIRED)
    
    # Create test executable
    add_executable(test_lexer test_lexer.cpp)
    
    # Set include directories for the test target
    target_include_directories(test_lexer 
        PRIVATE 
            ${CMAKE_CURRENT_SOURCE_DIR}/../include
    )
    
    # Link with the library if it exists, otherwise include source files
    if(TARGET ${PROJECT_NAME}_lib)
        target_link_libraries(test_lexer ${PROJECT_NAME}_lib)
    else()
        # Collect library source files (exclude main.cpp)
        file(GLOB_RECURSE LIB_TEST_SOURCES "../src/*.cpp")
        list(FILTER LIB_TEST_SOURCES EXCLUDE REGEX "main\\.cpp$")
        target_sources(test_lexer PRIVATE ${LIB_TEST_SOURCES})
    endif()
    
    # Link libraries if using a testing framework
    # target_link_libraries(test_lexer GTest::GTest GTest::Main)
    
    # Add test to CTest
    add_test(NAME LexerTest COMMAND test_lexer)
endif()
