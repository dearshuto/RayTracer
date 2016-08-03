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

void shkm::World::update()
{
    m_dynamicsWorld->updateAabbs();
    m_dynamicsWorld->computeOverlappingPairs();
}

void shkm::World::addCube()
{
    std::unique_ptr<btCollisionShape> groundShape(new btBoxShape(btVector3(btScalar(20.),btScalar(20.),btScalar(20.))));
    btScalar mass(0.);
    btVector3 localInertia(0,0,0);
    btTransform groundTransform;
    groundTransform.setIdentity();
    groundTransform.setOrigin(btVector3(0,-0,0));
    
    btDefaultMotionState* myMotionState = new btDefaultMotionState(groundTransform);
    btRigidBody::btRigidBodyConstructionInfo rbInfo(mass,myMotionState,groundShape.get(),localInertia);
    btRigidBody* body = new btRigidBody(rbInfo);
    body->setRollingFriction(1);
    body->setFriction(1);
    //add the body to the dynamics world
    m_dynamicsWorld->addRigidBody(body);

    m_collisionShapes.push_back( std::move(groundShape) );
}

void shkm::World::rayTest()const
{
    btVector3 from(-30,1.2,0);
    btVector3 to(0,0,0);
    
    btCollisionWorld::ClosestRayResultCallback	closestResults(from,to);
    closestResults.m_flags |= btTriangleRaycastCallback::kF_FilterBackfaces;
    m_dynamicsWorld->rayTest(from,to,closestResults);
    
    if (closestResults.hasHit())
    {
        btVector3 p = from.lerp(to,closestResults.m_closestHitFraction);
        std::cout << p.x() << ", " << p.y() << " " << p.z() << std::endl;
    }    
}