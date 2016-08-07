//
//  i_algorithm.h
//  RayTracer
//
//  Created by Shuto on 2016/08/07.
//
//

#ifndef i_algorithm_h
#define i_algorithm_h

#include <type/NormalizedColor.hpp>

namespace shkm {
    class World;
    class Position3d;
    class IAlgorithm;
}

class shkm::IAlgorithm
{
public:
    IAlgorithm() = default;
    virtual~IAlgorithm() = default;
    
    virtual fj::NormalizedColor render(const shkm::Position3d& from, const shkm::Position3d& to, const shkm::World& world)const = 0;
};

#endif /* i_algorithm_h */
