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
#include "type/position3d.hpp"

namespace shkm {
    class World;
}


namespace shkm {
struct CollisionInfo
{
    shkm::Position3d Normal;
    shkm::Position3d Position;
};
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
    
    /**
     * シーンの変更を適用する。変更がなければ呼ぶ必要はない。
     */
    void update();
    
    void addCube();
    
    shkm::CollisionInfo rayTest(const shkm::Position3d& from, const shkm::Position3d& to)const;
    
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

#endif /* world_hpp */
