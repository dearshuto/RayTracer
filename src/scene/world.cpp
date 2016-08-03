//
//  world.cpp
//  RayTracer
//
//  Created by Shuto on 2016/08/03.
//
//

#include <iostream>
#include <memory>
#include <vector>
#include <btBulletDynamicsCommon.h>
#include <BulletCollision/NarrowPhaseCollision/btRaycastCallback.h>
#include <BulletCollision/Gimpact/btGImpactShape.h>
#include "scene/world.hpp"

void shkm::World::rayTest()const
{
    m_dynamicsWorld->updateAabbs();
    m_dynamicsWorld->computeOverlappingPairs();
    
    
    btVector3 from(-30,1.2,0);
    btVector3 to(30,1.2,0);
    
    btCollisionWorld::ClosestRayResultCallback	closestResults(from,to);
    closestResults.m_flags |= btTriangleRaycastCallback::kF_FilterBackfaces;
    
    m_dynamicsWorld->rayTest(from,to,closestResults);
    
    if (closestResults.hasHit())
    {
        btVector3 p = from.lerp(to,closestResults.m_closestHitFraction);
        std::cout << p.x() << ", " << p.y() << " " << p.z() << std::endl;
    }
}