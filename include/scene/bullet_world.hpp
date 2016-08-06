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
    class BulletWorldImpl;
public:
    BulletWorld();
    ~BulletWorld() = default;
    
    void update()override;
    
    void addCube()override;
    
    shkm::CollisionInfo rayTest(const shkm::Position3d& from, const shkm::Position3d& to)const override;
    
private:
    std::shared_ptr<BulletWorldImpl> m_impl;
};

#endif /* bullet_world_hpp */
