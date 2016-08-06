PROJECT(RayTracer)

add_executable(basic_example basic_example.cpp)
target_link_libraries( basic_example
	ray_tracer
	jpeg
	image
	BulletDynamics
	BulletCollision
	LinearMath
)