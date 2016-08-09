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
#include <vector>

#include "type/math.hpp"

namespace shkm {
    class World;
}

namespace shkm {
struct CollisionInfo
{
    shkm::Vector3 Normal;
    shkm::Position3d Position;
};
}

class shkm::World
{
public:
    World() = default;
    ~World() = default;
    
    static void setTestScene(shkm::World* world);
    
    /**
     * シーンの変更を適用する。変更がなければ呼ぶ必要はない。
     */
    virtual void update() = 0;
    
    /**
     * @param position ワールド座標[m]
     * @param radius 半径[m]
     */
    virtual void addSphere(const shkm::Position3d& position, const double radius) = 0;
    
    virtual void addCube() = 0;

    /**
     * レイを飛ばして衝突情報を取得する
     */
    virtual shkm::CollisionInfo rayTest(const shkm::Position3d& from, const shkm::Position3d& to)const = 0;
};

#endif /* world_hpp */
