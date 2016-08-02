//
//  basic_example.cpp
//  RayTracer
//
//  Created by Shuto Shikama on 2016/08/02.
//
//

#include <iostream>
#include <jpeglib.h>
#include "image/image.hpp"

int main(int argc, char** argv)
{
    constexpr unsigned int kWidth = 255;
    constexpr unsigned int kHeight = 255;
    shkm::Image image;
    
    image.initialize(kWidth, kHeight);
    image.saveToFile( std::string("test.jpg") );
    
    return EXIT_SUCCESS;
}