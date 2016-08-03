//
//  position3d.cpp
//  RayTracer
//
//  Created by Shuto on 2016/08/03.
//
//

#include <limits>
#include "type/position3d.hpp"

bool shkm::Position3d::isInfinit()const
{
    if ( std::isinf(this->x()) ) return true;
    if ( std::isinf(this->y()) ) return true;
    if ( std::isinf(this->z()) ) return true;
    return false;
}