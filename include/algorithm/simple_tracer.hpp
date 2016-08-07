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

namespace fj {
    class SimpleTracer;
}

class fj::SimpleTracer : public shkm::IAlgorithm
{
public:
    SimpleTracer() = default;
    ~SimpleTracer() = default;
    
    fj::NormalizedColor render(const unsigned int x, const unsigned int y, const shkm::World& world)const override;
};

#endif /* simple_tracer_hpp */
