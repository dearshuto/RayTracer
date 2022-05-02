#include <_types/_uint16_t.h>
#include <_types/_uint8_t.h>
#include <cstdint>
#include <sys/_types/_int32_t.h>

namespace sjrt {namespace ffi {
    class PathTracer;
    class RapierScene;
    class ImageBuffer;
    class ExternalBuffer;
    class ExternalScene;
    class System;

    struct Float3 {
      float x;
      float y;
      float z;
      float _padding;
    };

    struct MaterialInfoData
    {
      Float3 normal;
      Float3 position;
    };


    PathTracer* create_path_tracer(uint16_t samplingCount, uint16_t depthMax);

    void destroy_path_tracer(PathTracer* pPathTracer);

    System* create_default_system();

    void destroy_default_system(System* pSystem);

    void render(const System* pSystem, RapierScene* pRapierScene, ImageBuffer* pImageBuffer);

    void render_to_external_buffer(System* pSystem, RapierScene* pRapierScene, ExternalBuffer* pScene);

    void render_with_external_resource(const System* pSystem, ExternalScene* pScene, ExternalBuffer* pExternalBuffer);

    RapierScene* create_default_scene();

    void destroy_default_scene(RapierScene* pScene);

    ImageBuffer* create_default_buffer(int32_t width, int32_t height);

    void destroy_default_buffer(ImageBuffer* pBuffer);

    ExternalBuffer *create_external_buffer(
                                           int32_t(*pGetWidthCallback)(),
                                           int32_t(*pGetHeightCallback)(),
                                           void(*pSetColorCallback)(int32_t x, int32_t y, uint8_t red, uint8_t green, uint8_t blue)
        );

    void destroy_external_buffer(ExternalBuffer* pExternalBuffer);

    ExternalScene* create_external_scene();

    void destroy_external_scene(ExternalScene* pExternalScene);
}}
