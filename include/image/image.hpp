//
//  image.hpp
//  RayTracer
//
//  Created by Shuto on 2016/08/02.
//
//

#ifndef image_hpp
#define image_hpp

#include <string>
#include <vector>

namespace shkm {
    class Image;
}

class shkm::Image
{
public:
    struct Color
    {
        uint8_t R;
        uint8_t G;
        uint8_t B;
    };
    
public:
    Image() = default;
    ~Image() = default;
    
    Image(const shkm::Image& other) = delete;
    Image& operator=(const shkm::Image& other) = delete;
    
    void initialize(const unsigned int width, const unsigned int height);
    
    // 将来的にファイルシステムからの読み込みを追加する
//    void initialize(const std::string& string);
    
    bool saveToFile(const std::string& string)const;
    
    shkm::Image::Color getColorAt(unsigned int x, unsigned int y)const;
    
    void setColorAt(unsigned int x, unsigned int y, shkm::Image::Color& color);
    
private:
    unsigned int convertPosition2D(const unsigned int x, const unsigned int y)const;
    
private:
    unsigned int m_width;
    
    unsigned int m_height;
    
    std::vector<Color> m_colorBuffer;
};

#endif /* image_hpp */
