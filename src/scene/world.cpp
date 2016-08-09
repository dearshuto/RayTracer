//
//  world.cpp
//  RayTracer
//
//  Created by Shuto on 2016/08/03.
//
//

#include <memory>
#include "scene/world.hpp"

void shkm::World::setTestScene(shkm::World *world)
{
    world->addSphere(shkm::Position3d(45, -25, 0), 50);
    world->addSphere(shkm::Position3d(-50, 0, -5), 35);
    world->addSphere(shkm::Position3d(-10, 70, 15), 25);
    world->update();
}