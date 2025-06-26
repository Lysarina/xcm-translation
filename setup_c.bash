
echo "Setup for Original variant"

sudo make clean

echo "Setting Makefile"
rm "Makefile.am"
cp -r "Makefile_c.am" "Makefile.am"