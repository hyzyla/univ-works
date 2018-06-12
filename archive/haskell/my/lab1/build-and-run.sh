ghc -o build/main -dynamic main.hs 
rm main.hi main.o
echo 
build/./main
