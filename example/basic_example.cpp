//
//  basic_example.cpp
//  RayTracer
//
//  Created by Shuto Shikama on 2016/08/02.
//
//

#include <iostream>
#include <image/JpegImage.hpp>
#include "scene/world.hpp"

int main(int argc, char** argv)
{
    constexpr unsigned int kWidth = 640;
    constexpr unsigned int kHeight = 480;
    shkm::World world;
    fj::JpegImage image;
    
    world.addCube();
    world.update();
    
    image.initialize(kWidth, kHeight);
    
    fj::NormalizedColor color;
    for (int i = 0; i < kWidth; i++){
        for (int j = 0; j < kHeight; j++)
        {
            const shkm::Position3d kFrom(0,0,-30);
            const shkm::Position3d kTo(i-320,j-240,0);
            auto kCollision = world.rayTest(kFrom, kTo);

            if (std::isfinite(kCollision.x()))
            {
                image.setAt(i, j, fj::NormalizedColor::WHITE);
            }
            else
            {
                image.setAt(i, j, fj::NormalizedColor::BLACK);
            }

        }
    }
    
    image.saveToFile( std::string("test.jpg") );
    
    return EXIT_SUCCESS;
}