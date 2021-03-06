//
//  bullet_world.cpp
//  RayTracer
//
//  Created by Shuto on 2016/08/06.
//
//

#include <btBulletDynamicsCommon.h>
#include <BulletCollision/NarrowPhaseCollision/btRaycastCallback.h>
#include <BulletCollision/Gimpact/btGImpactShape.h>
#include "scene/bullet_world.hpp"


class shkm::BulletWorld::BulletWorldImpl
{
public:
    BulletWorldImpl()
    : m_collisionConfiguration(new btDefaultCollisionConfiguration())
    , m_dispatcher(new btCollisionDispatcher(m_collisionConfiguration.get()))
    , m_broardphase(new btDbvtBroadphase())
    , m_solver(new btSequentialImpulseConstraintSolver())
    , m_dynamicsWorld(new btDiscreteDynamicsWorld(m_dispatcher.get(), m_broardphase.get(), m_solver.get(), m_collisionConfiguration.get()))
    {
        
    }
    ~BulletWorldImpl() = default;
    
    BulletWorldImpl(const BulletWorldImpl& other) = delete;
    BulletWorldImpl& operator=(const BulletWorldImpl& other) = delete;

private:
    std::unique_ptr<btCollisionConfiguration> m_collisionConfiguration;
    std::unique_ptr<btCollisionDispatcher> m_dispatcher;
    std::unique_ptr<btBroadphaseInterface> m_broardphase;
    std::unique_ptr<btConstraintSolver> m_solver;
    std::unique_ptr<btDynamicsWorld> m_dynamicsWorld;
    
    std::vector<std::unique_ptr<btCollisionShape>> m_collisionShapes;
    std::vector<std::unique_ptr<btCollisionObject>> m_collisionObjects;
    std::vector<std::unique_ptr<btMotionState>> m_motionStates;
public:
    
    void update()
    {
        m_dynamicsWorld->updateAabbs();
        m_dynamicsWorld->computeOverlappingPairs();
    }
    
    void addSphere(const shkm::Position3d& position, const double radius)
    {
        std::unique_ptr<btCollisionShape> groundShape(new btSphereShape(radius));
        btScalar mass(0.);
        btVector3 localInertia(0,0,0);
        btTransform groundTransform;
        groundTransform.setIdentity();
        groundTransform.setOrigin(btVector3(position.x(),position.y(),position.z()));
        
        std::unique_ptr<btDefaultMotionState> myMotionState(new btDefaultMotionState(groundTransform));
        btRigidBody::btRigidBodyConstructionInfo rbInfo(mass,myMotionState.get(),groundShape.get(),localInertia);
        std::unique_ptr<btRigidBody> body(new btRigidBody(rbInfo));

        //add the body to the dynamics world
        m_dynamicsWorld->addRigidBody(body.get());
        m_collisionShapes.push_back( std::move(groundShape) );
        m_motionStates.push_back( std::move(myMotionState) );
        m_collisionObjects.push_back( std::move(body) );
    }
    
    void addCube()
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
    
    shkm::CollisionInfo rayTest(const shkm::Position3d& from, const shkm::Position3d& to)const
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
    

};

shkm::BulletWorld::BulletWorld()
: m_impl( std::make_shared<BulletWorldImpl>() )
{
    
}

void shkm::BulletWorld::update()
{
    m_impl->update();
}

void shkm::BulletWorld::addSphere(const shkm::Position3d &position, const double radius)
{
    m_impl->addSphere(position, radius);
}

void shkm::BulletWorld::addCube()
{
    m_impl->addCube();
}

shkm::CollisionInfo shkm::BulletWorld::rayTest(const shkm::Position3d &from, const shkm::Position3d &to)const
{
    return m_impl->rayTest(from, to);
}