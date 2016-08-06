//
//  bullet_world.hpp
//  RayTracer
//
//  Created by Shuto on 2016/08/06.
//
//

#ifndef bullet_world_hpp
#define bullet_world_hpp

#include <memory>
#include "scene/world.hpp"

namespace shkm {
    class BulletWorld;
}

class shkm::BulletWorld : public shkm::World
{
public:
    BulletWorld()
    : m_collisionConfiguration(new btDefaultCollisionConfiguration())
    , m_dispatcher(new btCollisionDispatcher(m_collisionConfiguration.get()))
    , m_broardphase(new btDbvtBroadphase())
    , m_solver(new btSequentialImpulseConstraintSolver())
    , m_dynamicsWorld(new btDiscreteDynamicsWorld(m_dispatcher.get(), m_broardphase.get(), m_solver.get(), m_collisionConfiguration.get()))
    {
        
    }
    
    ~BulletWorld() = default;
    
    void update()override;
    
    void addCube()override;
    
    shkm::CollisionInfo rayTest(const shkm::Position3d& from, const shkm::Position3d& to)const override;
    
private:

    std::unique_ptr<btCollisionConfiguration> m_collisionConfiguration;
    std::unique_ptr<btCollisionDispatcher> m_dispatcher;
    std::unique_ptr<btBroadphaseInterface> m_broardphase;
    std::unique_ptr<btConstraintSolver> m_solver;
    std::unique_ptr<btDynamicsWorld> m_dynamicsWorld;
    
    std::vector<std::unique_ptr<btCollisionShape>> m_collisionShapes;
    std::vector<std::unique_ptr<btCollisionObject>> m_collisionObjects;
    std::vector<std::unique_ptr<btMotionState>> m_motionStates;
};

#endif /* bullet_world_hpp */
