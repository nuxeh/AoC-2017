#include <fstream>
#include <iostream>
#include <vector>

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

	unsigned long sum = 0;
	vector<char> values;

	char c;
	while(input.get(c) && c != 10)
	{
		values.push_back(c - 48);
	}

	int i = 0;
	for(auto value: values)
	{
		if (value == values[(i + 1) % values.size()])
			sum += value;

		i++;
	}

	input.close();

	cout << sum << endl;
}
