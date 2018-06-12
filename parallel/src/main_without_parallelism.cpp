#include <stdio.h>
#include <time.h>
#include <iostream>
#include "bmp.cpp"

using namespace std;


char toGrayScale(char red, char green, char blue) {
   return (char)(0.2126f * red + 0.7512f * green + 0.0722 * blue);
}


int main (int argc, char *argv[]) {
    char const *inputFile = "data/in.bmp";
    char const *outputFile = "data/out.bmp";
    unsigned char image[256][256][3];
    clock_t start, end;


    readBotmapImage(inputFile, image);

    start = clock();
    char grayScale;
    for (int i=0; i < 256; i++) {
        for(int j = 0; j < 256; j ++) {
            grayScale = toGrayScale(image[i][j][0], image[i][j][1], image[i][j][2]);
            image[i][j][0] = grayScale;
            image[i][j][1] = grayScale;
            image[i][j][2] = grayScale;
        }
    }
    end = clock();
    printf("time: %d\n",end-start);
    generateBitmapImage(image, 256, 256, outputFile);
    return 0;
}
