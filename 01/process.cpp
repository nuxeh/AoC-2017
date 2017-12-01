#include <fstream>
#include <iostream>

using namespace std;

int main(int argc, char** argv)
{
	if (!ifstream(argv[1])) {
		cout << argv[0] << ": File not found." << endl;
		exit(1);
	}
}
