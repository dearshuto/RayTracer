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
#include "scene/world.hpp"
#include "scene/bullet_world.hpp"

int main(int argc, char** argv)
{
    constexpr unsigned int kWidth = 640;
    constexpr unsigned int kHeight = 480;
    shkm::BulletWorld world;
    fj::JpegImage image;

    world.addCube();
    world.addSphere(shkm::Position3d(-40, 0, 5), 35);
    world.update();
    
    image.initialize(kWidth, kHeight);
    
    fj::NormalizedColor color;
    for (int i = 0; i < kWidth; i++){
        for (int j = 0; j < kHeight; j++)
        {
            const shkm::Position3d kFrom(0,0,-150);
            const shkm::Position3d kTo(i-320,j-240,0);
            auto kCollision = world.rayTest(kFrom, kTo);
            const shkm::Position3d& kPosition = kCollision.Position;
            const shkm::Position3d& kNormal = kCollision.Normal;
            
            if (std::isfinite(kPosition.x()))
            {
                image.setAt(i, j, fj::NormalizedColor( std::abs(kNormal.x()), std::abs(kNormal.y()), std::abs(kNormal.z())) );
            }
            else
            {
                image.setAt(i, j, fj::NormalizedColor::BLACK);
            }

        }
    }
    
    image.saveToFile( std::string("test.jpeg") );
    
    return EXIT_SUCCESS;
}