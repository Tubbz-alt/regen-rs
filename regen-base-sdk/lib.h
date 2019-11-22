#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct String;

struct Test {
  String a;
};

extern "C" {

String get_a(const Test *x);

Test greet();

} // extern "C"
