//
//  basic_example.cpp
//  RayTracer
//
//  Created by Shuto Shikama on 2016/08/02.
//
//

#include <iostream>
#include <image/JpegImage.hpp>
#include <image/BitmapImage.hpp>
#include "algorithm/simple_tracer.hpp"
#include "scene/world.hpp"
#include "scene/bullet_world.hpp"

int main(int argc, char** argv)
{
    constexpr unsigned int kWidth = 640;
    constexpr unsigned int kHeight = 480;
    shkm::BulletWorld world;
    shkm::World::setTestScene(&world);
    
    fj::JpegImage image;
    image.initialize(kWidth, kHeight);
    
    shkm::SimpleTracer algorithm;
    
    fj::NormalizedColor color;
    for (int i = 0; i < kWidth; i++){
        for (int j = 0; j < kHeight; j++)
        {
            const shkm::Position3d kFrom(0,0,-150);
            const shkm::Position3d kTo(i-320,j-240,0);
            const fj::NormalizedColor kColor = algorithm.render(kFrom, kTo, std::cref(world));

            image.setAt(i, j, kColor);
        }
    }
    
    image.saveToFile( std::string("test.jpeg") );
    
    return EXIT_SUCCESS;
}