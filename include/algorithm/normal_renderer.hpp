//
//  normal_renderer.hpp
//  RayTracer
//
//  Created by Shuto on 2016/08/07.
//
//

#ifndef normal_renderer_hpp
#define normal_renderer_hpp

#include "algorithm/i_algorithm.h"

namespace shkm {
    class NormalRenderer;
}

class shkm::NormalRenderer : public shkm::IAlgorithm
{
public:
    NormalRenderer() = default;
    ~NormalRenderer() = default;
    
    fj::NormalizedColor render(const shkm::Position3d& from, const shkm::Position3d& to, const shkm::World& world)const override;
};

#endif /* normal_renderer_hpp */
