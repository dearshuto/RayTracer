import os
import subprocess
import platform


def cmake_setting():
	rootDirectory = os.getcwd()
	if not os.path.exists("./build"):
       		os.mkdir("./build")
	os.chdir(rootDirectory + "/build")

	# set up cmake commands
	me = platform.system()
	cmd = ['cmake']
	if me == 'Windows':
		print 'run cmake as Windows'	
		cmd = ['cmake', '..', '-G', 'Visual Studio 14 2015', '-DBUILD_STATIC=ON -DBUILD_SHARED_LIBS=OF)']
	elif me == 'Linux':
		cmd = ['cmake', '..', '-G', 'Makefile', '-DBUILD_STATIC=ON -DBUILD_SHARED_LIBS=OF']
	elif me == 'Darwin':
		cmd = ['cmake', '..', '-G', 'Xcode', '-DBUILD_STATIC=ON -DBUILD_SHARED_LIBS=OF)']

	# run cmake with arguments
	try:
        	res = subprocess.check_call(cmd)
	except:
		print "cmake error"
