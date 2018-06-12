#include <stdio.h>
#include <time.h>
#include <iostream>
#include <omp.h>
#include "bmp.cpp"

using namespace std;


char toGrayScale(char red, char green, char blue) {
   return (char)(red + green + blue) / 3;
}


int main (int argc, char *argv[]) {
    char const *inputFile = "data/in.bmp";
    char const *outputFile = "data/out.bmp";
    unsigned char image[256][256][3];
    clock_t start, end;


    readBotmapImage(inputFile, image);

    start = clock();
    char grayScale;


    #pragma omp parallel num_threads(2)
    {
        #pragma omp for private(grayScale, image)
        for (int i=0; i < 256; i++) {
            for(int j = 0; j < 256; j ++) {
                grayScale = toGrayScale(image[i][j][0], image[i][j][1], image[i][j][2]);
                image[i][j][0] = grayScale;
                image[i][j][1] = grayScale;
                image[i][j][2] = grayScale;
            }
        }
        
    }
    
    end = clock();
    printf("%d\n",end-start);
    generateBitmapImage(image, 256, 256, outputFile);
    return 0;
}
