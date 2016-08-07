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
    class IAlgorithm;
}

class shkm::IAlgorithm
{
public:
    IAlgorithm() = default;
    virtual~IAlgorithm() = default;
    
    virtual fj::NormalizedColor render(const unsigned int x, const unsigned int y, const shkm::World& world)const = 0;
};

#endif /* i_algorithm_h */
