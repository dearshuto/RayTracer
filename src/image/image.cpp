//
//  image.cpp
//  RayTracer
//
//  Created by Shuto on 2016/08/02.
//
//

#include <iostream>
#include <jpeglib.h>
#include "image/image.hpp"

void shkm::Image::initialize(const unsigned int width, const unsigned int height)
{
    m_width = width;
    m_height = height;
    m_colorBuffer.resize(width * height*3);
}

bool shkm::Image::saveToFile(const std::string &string)const
{
    struct jpeg_compress_struct cinfo;
    struct jpeg_error_mgr jerr;
    
    cinfo.err = jpeg_std_error(&jerr);
    jpeg_create_compress(&cinfo);
    
    /* 出力ファイルの設定 */
    const std::string filename = "output.jpg";
    FILE *fp = fopen(filename.c_str(), "wb");
    if (fp == NULL) {
        fprintf(stderr, "cannot open %s\n", filename.c_str());
        return false;
    }
    jpeg_stdio_dest(&cinfo, fp);
    
    cinfo.image_width = m_width;
    cinfo.image_height = m_height;
    cinfo.input_components = 3;
    cinfo.in_color_space = JCS_RGB;
    jpeg_set_defaults(&cinfo);
    jpeg_set_quality(&cinfo, 75, TRUE);
    
    /* 圧縮開始 */
        jpeg_start_compress(&cinfo, TRUE);
    
    JSAMPARRAY img = (JSAMPARRAY) malloc(sizeof(JSAMPROW) * m_height);
    for (int i = 0; i < m_height; i++) {
        img[i] = (JSAMPROW) malloc(sizeof(JSAMPLE) * 3 * m_width);
        for (int j = 0; j < m_width; j++) {
            const auto kColor = getColorAt(i, j);
            
            img[i][j*3 + 0] = kColor.R;
            img[i][j*3 + 1] = kColor.G;
            img[i][j*3 + 2] = kColor.B;
        }
    }
    
    /* 書き込む */
    jpeg_write_scanlines(&cinfo, img, m_height);
    
    /* 圧縮終了 */
        jpeg_finish_compress(&cinfo);
    
    jpeg_destroy_compress(&cinfo);
    
    for (int i = 0; i < m_height; i++) {
        free(img[i]);
    }
    
    free(img);
    fclose(fp);
    
    return true;
}

shkm::Image::Color shkm::Image::getColorAt(unsigned int x, unsigned int y)const
{
    shkm::Image::Color color;
    color.R = m_colorBuffer[y*m_width + x*3 + 0];
    color.G = m_colorBuffer[y*m_width + x*3 + 1];
    color.B = m_colorBuffer[y*m_width + x*3 + 2];
    
    return color;
}