# Modified by yejq

QT        += core gui
QT        += widgets
CONFIG    += c++17 staticlib
TEMPLATE   = lib
TARGET     = wubiform

SOURCES += wbDialog.cpp

HEADERS += wubiform.h wbDialog.h

FORMS += wubiform.ui

win32:CMAKE_CXXFLAGS += /source-charset:utf-8 /execution-charset:gb2312
