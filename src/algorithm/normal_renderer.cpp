//
//  normal_renderer.cpp
//  RayTracer
//
//  Created by Shuto on 2016/08/07.
//
//

#include <algorithm>
#include "scene/world.hpp"
#include "algorithm/normal_renderer.hpp"

fj::NormalizedColor shkm::NormalRenderer::render(const shkm::Position3d &from, const shkm::Position3d &to, const shkm::World &worl)const
{
    const auto& kNormal = worl.rayTest(from, to).Normal;
    
    return fj::NormalizedColor( std::abs(kNormal.x()), std::abs(kNormal.y()), std::abs(kNormal.z()) );
}