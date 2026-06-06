#!/bin/sh

export PATH=/home/appliedai/v/gitnu/.build:$PATH

cd /home/appliedai/v/gitnu/src
git -c color.status=never nu status
