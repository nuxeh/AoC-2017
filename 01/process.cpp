#include <fstream>
#include <iostream>

using namespace std;

int process_file_input(ifstream &input) {
	cout << input.get() << endl;
}


int main(int argc, char** argv)
{
	if (!ifstream(argv[1])) {
		cout << argv[0] << ": File not found." << endl;
		exit(1);
	}

	ifstream input;
	input.open(argv[1]);

//	process_file_input(&input);

	char c;
	while(input.get(c) && c != 10)
		cout << int(c) - 48;

	input.close();
}
