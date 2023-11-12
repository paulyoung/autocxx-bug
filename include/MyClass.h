#ifndef MY_CLASS
#define MY_CLASS

#include <stddef.h>

class MyClass
{
 public:
	MyClass();
	virtual ~MyClass();

	static void* operator new( size_t size );
};

#endif // MY_CLASS
