# i have comments yippee
if (true) {
    int x = 10;
} else {
    float y = 3.14;
}

while (x > 0) {
    x = x - 1;
    continue;
}

for (int i = 0; i < 10; i++) { #skip me bro 
    if (i == 5) break;
}

int count = 42;
float pi = 3.14159;
char letter = 'A';
string greeting = "Hello World";
string escaped = "Text with \"quotes\" and \n newlines";
bool flag = true;
void doNothing() { return; }

fn calculate(int a, int b) { # Hi how are you
    return a + b;
}

x = 10;
y = x++;
z = --x;
result = (x + y) * z / 2 % 3;

if (x < y || x > z) {
    flag = x <= z && y >= x;
}

if (x == y) {
    flag = !flag;
} else if (x != y) {
    flag = true;
}

int bits = 5 & 3;
bits = bits | 8;
bits = bits ^ 15;
bits = ~bits;
bits = bits << 2;
bits = bits >> 1;

int numbers[5];
numbers[0] = 42;

string empty = null;

string test = "This is a \"quoted\" string with \t tab and \n newline";

int negative = -123;
float decimal = -45.67;