MAKEFILE_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR  := $(realpath $(dir $(MAKEFILE_PATH)))

fmt:
	swiftformat .

build: FORCE
	xcodebuild -project Paper.xcodeproj \
		-scheme Paper \
		-configuration Release \
		-archivePath Paper.xcarchive \
		archive

dev: FORCE
	open Paper.xcodeproj

FORCE: ;
