import os
import external.image.external_setup as image
import cmake_generator as cmk

rootDirectory = os.getcwd()

os.chdir('external/image')
image.libjpeg_setting()
os.chdir(rootDirectory)

cmk.cmake_setting()
