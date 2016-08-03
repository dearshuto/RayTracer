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
    constexpr unsigned int kWidth = 640;
    constexpr unsigned int kHeight = 480;
    shkm::World world;
    shkm::Image image;
    
    world.addCube();
    world.update();
    
    image.initialize(kWidth, kHeight);
    
    shkm::Image::Color color;
    for (int i = 0; i < kWidth; i++){
        for (int j = 0; j < kHeight; j++)
        {
            const shkm::Position3d kFrom(0,0,-30);
            const shkm::Position3d kTo(i-320,j-240,0);
            auto kCollision = world.rayTest(kFrom, kTo);

            if (std::isfinite(kCollision.x()))
            {
                color.R = 255;
                color.G = 255;
                color.B = 255;
            }
            else
            {
                color.R = 0;
                color.G = 0;
                color.B = 0;
            }
            
            image.setColorAt(i, j, color);
        }
    }
    
    image.saveToFile( std::string("test.jpg") );
    
    return EXIT_SUCCESS;
}