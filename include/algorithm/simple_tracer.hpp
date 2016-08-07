//
//  simple_tracer.hpp
//  RayTracer
//
//  Created by Shuto on 2016/08/07.
//
//

#ifndef simple_tracer_hpp
#define simple_tracer_hpp

#include "algorithm/i_algorithm.h"

namespace shkm {
    class SimpleTracer;
}

class shkm::SimpleTracer : public shkm::IAlgorithm
{
public:
    SimpleTracer() = default;
    ~SimpleTracer() = default;
    
    fj::NormalizedColor render(const shkm::Position3d& from, const shkm::Position3d& to, const shkm::World& world)const override;
};

#endif /* simple_tracer_hpp */
