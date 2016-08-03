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
#include "scene/world.hpp"

int main(int argc, char** argv)
{
    constexpr unsigned int kWidth = 256;
    constexpr unsigned int kHeight = 256;
    shkm::World world;
    shkm::Image image;
    
    world.rayTest();
    image.initialize(kWidth, kHeight);
    
    shkm::Image::Color color;
    for (int i = 0; i < kWidth; i++){
        for (int j = 0; j < kHeight; j++)
        {
            color.R = i;
            color.G = j;
            color.B = 127;

            image.setColorAt(i, j, color);
        }
    }
    
    image.saveToFile( std::string("test.jpg") );
    
    return EXIT_SUCCESS;
}