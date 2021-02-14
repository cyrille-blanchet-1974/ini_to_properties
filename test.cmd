@echo off
cargo build --release
set prg=.\target\release\ini_to_properties.exe
echo [section1]> ori.ini
echo key1=val1>> ori.ini
echo key2=val2>> ori.ini
echo ;key4=val4>> ori.ini
echo           >> ori.ini
echo [section2]>> ori.ini
echo key1=val1>> ori.ini
echo key2=val2>> ori.ini
echo original:
echo -------------------------------------------
type ori.ini
echo -------------------------------------------
echo Expected: add key3 in section1
%prg%  /fic:ori.ini
echo -------------------------------------------
type ori.properties
echo -------------------------------------------
pause