#include <iostream>

//ifstream fin("cifrevecine.in");
//ofstream fout("cifrevecine.out");

int main() {
    unsigned long long n;
    unsigned short k, c = 0;
    unsigned long long m = 0;
    std::cin >> n >> k;
    unsigned short int vf[18];
    unsigned long long inv = 0;
    while(n) inv = inv * 10 + n % 10, n /= 10;
    while(inv) vf[c++] = inv % 10, inv /= 10;

    for(int i = 0; i < c - k + 1; i++) {
        int nr = 0, p = 1;
        int j = i;
        for(int q = 0; q < i; q++) {
            nr = nr * 10 + vf[q];
        }

        for(int q = i + k; q < c; q++) {
              nr = nr * 10 + vf[q];
        }
        m = nr > m ? nr : m;
    }
    std::cout <<"Nr de cifre: " << c << std::endl;

    std::cout << m;
}