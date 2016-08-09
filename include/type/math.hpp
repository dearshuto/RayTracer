//
//  math.hpp
//  RayTracer
//
//  Created by Shuto on 2016/08/03.
//
//

#ifndef position3d_hpp
#define position3d_hpp

#include <Eigen/Core>

namespace shkm {
    typedef Eigen::Vector3d Vector3;
    class Position3d;
}

class shkm::Position3d : public Vector3
{
public:
    explicit Position3d()
    : shkm::Position3d(0, 0, 0)
    {
        
    }
    
    ~Position3d() = default;
    
    explicit Position3d(const double x, const double y, const double z)
    : Eigen::Vector3d(x, y, z)
    {
        
    }
    
    /**
     * 要素の中に無限大が含まれているか調べる
     */
    bool isInfinit()const;
};

#endif /* position3d_hpp */
