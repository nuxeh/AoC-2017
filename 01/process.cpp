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

	cout << int(input.get()) - 48 << endl;
	cout << input.get() << endl;
	cout << input.get() << endl;
	cout << input.get() << endl;
	cout << input.get() << endl;
	cout << input.get() << endl;

	input.close();
}
