#/bin/sh
function pause {
    read -n 120 -p "Press 'Enter' to continue..." ; echo " "
}

cargo build --release
export prg=./target/release/ini_to_properties
echo [section1]> ori.ini
echo key1=val1>> ori.ini
echo key2=val2>> ori.ini
echo key3=val3>> ori.ini
echo \;key4=val4>> ori.ini
echo           >> ori.ini
echo [section2]>> ori.ini
echo key1=val1>> ori.ini
echo key2=val2>> ori.ini
echo key3=val3>> ori.ini
echo original:
echo -------------------------------------------
cat ori.ini
echo -------------------------------------------
$prg  /fic:ori 
echo -------------------------------------------
cat ori.properties
echo -------------------------------------------
pause