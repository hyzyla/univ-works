ghc -o build/main -dynamic main.hs
if [ $? -eq 0 ]
then
    echo ""
else
    exit 1;
fi

rm main.hi main.o 
build/./main
