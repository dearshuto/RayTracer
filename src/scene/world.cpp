//
//  world.cpp
//  RayTracer
//
//  Created by Shuto on 2016/08/03.
//
//

#include <iostream>
#include <limits>
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
    std::unique_ptr<btCollisionShape> groundShape(new btSphereShape(25));
    btScalar mass(0.);
    btVector3 localInertia(0,0,0);
    btTransform groundTransform;
    groundTransform.setIdentity();
    groundTransform.setOrigin(btVector3(60,0,10));
    
    std::unique_ptr<btDefaultMotionState> myMotionState(new btDefaultMotionState(groundTransform));
    btRigidBody::btRigidBodyConstructionInfo rbInfo(mass,myMotionState.get(),groundShape.get(),localInertia);
    std::unique_ptr<btRigidBody> body(new btRigidBody(rbInfo));
    body->setRollingFriction(1);
    body->setFriction(1);
    //add the body to the dynamics world
    m_dynamicsWorld->addRigidBody(body.get());
    
    m_collisionShapes.push_back( std::move(groundShape) );
    m_motionStates.push_back( std::move(myMotionState) );
    m_collisionObjects.push_back( std::move(body) );
}

shkm::CollisionInfo shkm::World::rayTest(const shkm::Position3d& from, const shkm::Position3d& to)const
{
    const btVector3 kFrom(from.x(), from.y(), from.z());
    const btVector3 kTo(to.x(), to.y(), to.z());
    shkm::CollisionInfo collisionInfo;
    
    btCollisionWorld::ClosestRayResultCallback	closestResults(kFrom,kTo);
    closestResults.m_flags |= btTriangleRaycastCallback::kF_FilterBackfaces;
    m_dynamicsWorld->rayTest(kFrom,kTo,closestResults);
    
    if (closestResults.hasHit())
    {
        const btVector3& p = closestResults.m_hitPointWorld;
        const btVector3& kNormal = closestResults.m_hitNormalWorld;
        
        collisionInfo.Position = shkm::Position3d(p.x(), p.y(), p.z());
        collisionInfo.Normal = shkm::Position3d(kNormal.x(), kNormal.y(), kNormal.z());
        
        return collisionInfo;
    }
    
    const auto kInfinity = std::numeric_limits<double>::infinity();
    collisionInfo.Position.x() = kInfinity;
    
    return collisionInfo;
}