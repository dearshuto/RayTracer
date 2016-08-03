//
//  world.hpp
//  RayTracer
//
//  Created by Shuto on 2016/08/03.
//
//

#ifndef world_hpp
#define world_hpp

#include <memory>
#include <btBulletDynamicsCommon.h>
#include <BulletCollision/NarrowPhaseCollision/btRaycastCallback.h>
#include <BulletCollision/Gimpact/btGImpactShape.h>

namespace shkm {
    class World;
}

class shkm::World
{
public:
    World()
    : m_collisionConfiguration(new btDefaultCollisionConfiguration())
    , m_dispatcher(new btCollisionDispatcher(m_collisionConfiguration.get()))
    , m_broardphase(new btDbvtBroadphase())
    , m_solver(new btSequentialImpulseConstraintSolver())
    , m_dynamicsWorld(new btDiscreteDynamicsWorld(m_dispatcher.get(), m_broardphase.get(), m_solver.get(), m_collisionConfiguration.get()))
    {
        
    }
    
    ~World() = default;
    
    void rayTest()const;
    
private:
    std::unique_ptr<btCollisionConfiguration> m_collisionConfiguration;
    std::unique_ptr<btCollisionDispatcher> m_dispatcher;
    std::unique_ptr<btBroadphaseInterface> m_broardphase;
    std::unique_ptr<btConstraintSolver> m_solver;
    std::unique_ptr<btDynamicsWorld> m_dynamicsWorld;
    
    std::vector<std::unique_ptr<btCollisionShape>> m_collisionShapes;
};

#endif /* world_hpp */
