

//> Chunks of Bytecode value-h
#ifndef clox_value_h
#define clox_value_h
//> Optimization include-string

#include <string.h>
//< Optimization include-string


//> Chunks of Bytecode common-h
#ifndef clox_common_h
#define clox_common_h

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
//> A Virtual Machine define-debug-trace

//> Optimization define-nan-boxing
#define NAN_BOXING
//< Optimization define-nan-boxing
//> Compiling Expressions define-debug-print-code
#define DEBUG_PRINT_CODE
//< Compiling Expressions define-debug-print-code
#define DEBUG_TRACE_EXECUTION
//< A Virtual Machine define-debug-trace
//> Garbage Collection define-stress-gc

#define DEBUG_STRESS_GC
//< Garbage Collection define-stress-gc
//> Garbage Collection define-log-gc
#define DEBUG_LOG_GC
//< Garbage Collection define-log-gc
//> Local Variables uint8-count

#define UINT8_COUNT (UINT8_MAX + 1)
//< Local Variables uint8-count

#endif
//> omit
// In the book, we show them defined, but for working on them locally,
// we don't want them to be.
#undef DEBUG_PRINT_CODE
#undef DEBUG_TRACE_EXECUTION
#undef DEBUG_STRESS_GC
#undef DEBUG_LOG_GC
//< omit



//> Strings forward-declare-obj
typedef struct Obj Obj;
//> forward-declare-obj-string
typedef struct ObjString ObjString;
//< forward-declare-obj-string

//< Strings forward-declare-obj
//> Optimization nan-boxing
#ifdef NAN_BOXING
//> qnan

//> sign-bit
#define SIGN_BIT ((uint64_t)0x8000000000000000)
//< sign-bit
#define QNAN     ((uint64_t)0x7ffc000000000000)
//< qnan
//> tags

#define TAG_NIL   1 // 01.
#define TAG_FALSE 2 // 10.
#define TAG_TRUE  3 // 11.
//< tags

typedef uint64_t Value;
//> is-number

//> is-bool
#define IS_BOOL(value)      (((value) | 1) == TRUE_VAL)
//< is-bool
//> is-nil
#define IS_NIL(value)       ((value) == NIL_VAL)
//< is-nil
#define IS_NUMBER(value)    (((value) & QNAN) != QNAN)
//< is-number
//> is-obj
#define IS_OBJ(value) \
    (((value) & (QNAN | SIGN_BIT)) == (QNAN | SIGN_BIT))
//< is-obj
//> as-number

//> as-bool
#define AS_BOOL(value)      ((value) == TRUE_VAL)
//< as-bool
#define AS_NUMBER(value)    valueToNum(value)
//< as-number
//> as-obj
#define AS_OBJ(value) \
    ((Obj*)(uintptr_t)((value) & ~(SIGN_BIT | QNAN)))
//< as-obj
//> number-val

//> bool-val
#define BOOL_VAL(b)     ((b) ? TRUE_VAL : FALSE_VAL)
//< bool-val
//> false-true-vals
#define FALSE_VAL       ((Value)(uint64_t)(QNAN | TAG_FALSE))
#define TRUE_VAL        ((Value)(uint64_t)(QNAN | TAG_TRUE))
//< false-true-vals
//> nil-val
#define NIL_VAL         ((Value)(uint64_t)(QNAN | TAG_NIL))
//< nil-val
#define NUMBER_VAL(num) numToValue(num)
//< number-val
//> obj-val
#define OBJ_VAL(obj) \
    (Value)(SIGN_BIT | QNAN | (uint64_t)(uintptr_t)(obj))
//< obj-val
//> value-to-num

static inline double valueToNum(Value value) {
  double num;
  memcpy(&num, &value, sizeof(Value));
  return num;
}
//< value-to-num
//> num-to-value

static inline Value numToValue(double num) {
  Value value;
  memcpy(&value, &num, sizeof(double));
  return value;
}
//< num-to-value

#else

//< Optimization nan-boxing
//> Types of Values value-type
typedef enum {
  VAL_BOOL,
  VAL_NIL, // [user-types]
  VAL_NUMBER,
//> Strings val-obj
  VAL_OBJ
//< Strings val-obj
} ValueType;

//< Types of Values value-type
/* Chunks of Bytecode value-h < Types of Values value
typedef double Value;
*/
//> Types of Values value
typedef struct {
  ValueType type;
  union {
    bool boolean;
    double number;
//> Strings union-object
    Obj* obj;
//< Strings union-object
  } as; // [as]
} Value;
//< Types of Values value
//> Types of Values is-macros

#define IS_BOOL(value)    ((value).type == VAL_BOOL)
#define IS_NIL(value)     ((value).type == VAL_NIL)
#define IS_NUMBER(value)  ((value).type == VAL_NUMBER)
//> Strings is-obj
#define IS_OBJ(value)     ((value).type == VAL_OBJ)
//< Strings is-obj
//< Types of Values is-macros
//> Types of Values as-macros

//> Strings as-obj
#define AS_OBJ(value)     ((value).as.obj)
//< Strings as-obj
#define AS_BOOL(value)    ((value).as.boolean)
#define AS_NUMBER(value)  ((value).as.number)
//< Types of Values as-macros
//> Types of Values value-macros

#define BOOL_VAL(value)   ((Value){VAL_BOOL, {.boolean = value}})
#define NIL_VAL           ((Value){VAL_NIL, {.number = 0}})
#define NUMBER_VAL(value) ((Value){VAL_NUMBER, {.number = value}})
//> Strings obj-val
#define OBJ_VAL(object)   ((Value){VAL_OBJ, {.obj = (Obj*)object}})
//< Strings obj-val
//< Types of Values value-macros
//> Optimization end-if-nan-boxing

#endif
//< Optimization end-if-nan-boxing
//> value-array

typedef struct {
  int capacity;
  int count;
  Value* values;
} ValueArray;
//< value-array
//> array-fns-h

//> Types of Values values-equal-h
bool valuesEqual(Value a, Value b);
//< Types of Values values-equal-h
void initValueArray(ValueArray* array);
void writeValueArray(ValueArray* array, Value value);
void freeValueArray(ValueArray* array);
//< array-fns-h
//> print-value-h
void printValue(Value value);
//< print-value-h

#endif



//> A Virtual Machine vm-h
#ifndef clox_vm_h
#define clox_vm_h


//> Chunks of Bytecode chunk-h
#ifndef clox_chunk_h
#define clox_chunk_h

//> chunk-h-include-value
//< chunk-h-include-value
//> op-enum

typedef enum {
//> op-constant
  OP_CONSTANT,
//< op-constant
//> Types of Values literal-ops
  OP_NIL,
  OP_TRUE,
  OP_FALSE,
//< Types of Values literal-ops
//> Global Variables pop-op
  OP_POP,
//< Global Variables pop-op
//> Local Variables get-local-op
  OP_GET_LOCAL,
//< Local Variables get-local-op
//> Local Variables set-local-op
  OP_SET_LOCAL,
//< Local Variables set-local-op
//> Global Variables get-global-op
  OP_GET_GLOBAL,
//< Global Variables get-global-op
//> Global Variables define-global-op
  OP_DEFINE_GLOBAL,
//< Global Variables define-global-op
//> Global Variables set-global-op
  OP_SET_GLOBAL,
//< Global Variables set-global-op
//> Closures upvalue-ops
  OP_GET_UPVALUE,
  OP_SET_UPVALUE,
//< Closures upvalue-ops
//> Classes and Instances property-ops
  OP_GET_PROPERTY,
  OP_SET_PROPERTY,
//< Classes and Instances property-ops
//> Superclasses get-super-op
  OP_GET_SUPER,
//< Superclasses get-super-op
//> Types of Values comparison-ops
  OP_EQUAL,
  OP_GREATER,
  OP_LESS,
//< Types of Values comparison-ops
//> A Virtual Machine binary-ops
  OP_ADD,
  OP_SUBTRACT,
  OP_MULTIPLY,
  OP_DIVIDE,
//> Types of Values not-op
  OP_NOT,
//< Types of Values not-op
//< A Virtual Machine binary-ops
//> A Virtual Machine negate-op
  OP_NEGATE,
//< A Virtual Machine negate-op
//> Global Variables op-print
  OP_PRINT,
//< Global Variables op-print
//> Jumping Back and Forth jump-op
  OP_JUMP,
//< Jumping Back and Forth jump-op
//> Jumping Back and Forth jump-if-false-op
  OP_JUMP_IF_FALSE,
//< Jumping Back and Forth jump-if-false-op
//> Jumping Back and Forth loop-op
  OP_LOOP,
//< Jumping Back and Forth loop-op
//> Calls and Functions op-call
  OP_CALL,
//< Calls and Functions op-call
//> Methods and Initializers invoke-op
  OP_INVOKE,
//< Methods and Initializers invoke-op
//> Superclasses super-invoke-op
  OP_SUPER_INVOKE,
//< Superclasses super-invoke-op
//> Closures closure-op
  OP_CLOSURE,
//< Closures closure-op
//> Closures close-upvalue-op
  OP_CLOSE_UPVALUE,
//< Closures close-upvalue-op
  OP_RETURN,
//> Classes and Instances class-op
  OP_CLASS,
//< Classes and Instances class-op
//> Superclasses inherit-op
  OP_INHERIT,
//< Superclasses inherit-op
//> Methods and Initializers method-op
  OP_METHOD
//< Methods and Initializers method-op
} OpCode;
//< op-enum
//> chunk-struct

typedef struct {
//> count-and-capacity
  int count;
  int capacity;
//< count-and-capacity
  uint8_t* code;
//> chunk-lines
  int* lines;
//< chunk-lines
//> chunk-constants
  ValueArray constants;
//< chunk-constants
} Chunk;
//< chunk-struct
//> init-chunk-h

void initChunk(Chunk* chunk);
//< init-chunk-h
//> free-chunk-h
void freeChunk(Chunk* chunk);
//< free-chunk-h
/* Chunks of Bytecode write-chunk-h < Chunks of Bytecode write-chunk-with-line-h
void writeChunk(Chunk* chunk, uint8_t byte);
*/
//> write-chunk-with-line-h
void writeChunk(Chunk* chunk, uint8_t byte, int line);
//< write-chunk-with-line-h
//> add-constant-h
int addConstant(Chunk* chunk, Value value);
//< add-constant-h

#endif


//> Calls and Functions vm-include-object

//> Strings object-h
#ifndef clox_object_h
#define clox_object_h

//> Calls and Functions object-include-chunk
//< Calls and Functions object-include-chunk
//> Classes and Instances object-include-table

//> Hash Tables table-h
#ifndef clox_table_h
#define clox_table_h

//> entry

typedef struct {
  ObjString* key;
  Value value;
} Entry;
//< entry

typedef struct {
  int count;
  int capacity;
  Entry* entries;
} Table;

//> init-table-h
void initTable(Table* table);
//> free-table-h
void freeTable(Table* table);
//< free-table-h
//> table-get-h
bool tableGet(Table* table, ObjString* key, Value* value);
//< table-get-h
//> table-set-h
bool tableSet(Table* table, ObjString* key, Value value);
//< table-set-h
//> table-delete-h
bool tableDelete(Table* table, ObjString* key);
//< table-delete-h
//> table-add-all-h
void tableAddAll(Table* from, Table* to);
//< table-add-all-h
//> table-find-string-h
ObjString* tableFindString(Table* table, const char* chars,
                           int length, uint32_t hash);
//< table-find-string-h
//> Garbage Collection table-remove-white-h

void tableRemoveWhite(Table* table);
//< Garbage Collection table-remove-white-h
//> Garbage Collection mark-table-h
void markTable(Table* table);
//< Garbage Collection mark-table-h

//< init-table-h
#endif


//< Classes and Instances object-include-table
//> obj-type-macro

#define OBJ_TYPE(value)        (AS_OBJ(value)->type)
//< obj-type-macro
//> is-string

//> Methods and Initializers is-bound-method
#define IS_BOUND_METHOD(value) isObjType(value, OBJ_BOUND_METHOD)
//< Methods and Initializers is-bound-method
//> Classes and Instances is-class
#define IS_CLASS(value)        isObjType(value, OBJ_CLASS)
//< Classes and Instances is-class
//> Closures is-closure
#define IS_CLOSURE(value)      isObjType(value, OBJ_CLOSURE)
//< Closures is-closure
//> Calls and Functions is-function
#define IS_FUNCTION(value)     isObjType(value, OBJ_FUNCTION)
//< Calls and Functions is-function
//> Classes and Instances is-instance
#define IS_INSTANCE(value)     isObjType(value, OBJ_INSTANCE)
//< Classes and Instances is-instance
//> Calls and Functions is-native
#define IS_NATIVE(value)       isObjType(value, OBJ_NATIVE)
//< Calls and Functions is-native
#define IS_STRING(value)       isObjType(value, OBJ_STRING)
//< is-string
//> as-string

//> Methods and Initializers as-bound-method
#define AS_BOUND_METHOD(value) ((ObjBoundMethod*)AS_OBJ(value))
//< Methods and Initializers as-bound-method
//> Classes and Instances as-class
#define AS_CLASS(value)        ((ObjClass*)AS_OBJ(value))
//< Classes and Instances as-class
//> Closures as-closure
#define AS_CLOSURE(value)      ((ObjClosure*)AS_OBJ(value))
//< Closures as-closure
//> Calls and Functions as-function
#define AS_FUNCTION(value)     ((ObjFunction*)AS_OBJ(value))
//< Calls and Functions as-function
//> Classes and Instances as-instance
#define AS_INSTANCE(value)     ((ObjInstance*)AS_OBJ(value))
//< Classes and Instances as-instance
//> Calls and Functions as-native
#define AS_NATIVE(value) \
    (((ObjNative*)AS_OBJ(value))->function)
//< Calls and Functions as-native
#define AS_STRING(value)       ((ObjString*)AS_OBJ(value))
#define AS_CSTRING(value)      (((ObjString*)AS_OBJ(value))->chars)
//< as-string
//> obj-type

typedef enum {
//> Methods and Initializers obj-type-bound-method
  OBJ_BOUND_METHOD,
//< Methods and Initializers obj-type-bound-method
//> Classes and Instances obj-type-class
  OBJ_CLASS,
//< Classes and Instances obj-type-class
//> Closures obj-type-closure
  OBJ_CLOSURE,
//< Closures obj-type-closure
//> Calls and Functions obj-type-function
  OBJ_FUNCTION,
//< Calls and Functions obj-type-function
//> Classes and Instances obj-type-instance
  OBJ_INSTANCE,
//< Classes and Instances obj-type-instance
//> Calls and Functions obj-type-native
  OBJ_NATIVE,
//< Calls and Functions obj-type-native
  OBJ_STRING,
//> Closures obj-type-upvalue
  OBJ_UPVALUE
//< Closures obj-type-upvalue
} ObjType;
//< obj-type

struct Obj {
  ObjType type;
//> Garbage Collection is-marked-field
  bool isMarked;
//< Garbage Collection is-marked-field
//> next-field
  struct Obj* next;
//< next-field
};
//> Calls and Functions obj-function

typedef struct {
  Obj obj;
  int arity;
//> Closures upvalue-count
  int upvalueCount;
//< Closures upvalue-count
  Chunk chunk;
  ObjString* name;
} ObjFunction;
//< Calls and Functions obj-function
//> Calls and Functions obj-native

typedef Value (*NativeFn)(int argCount, Value* args);

typedef struct {
  Obj obj;
  NativeFn function;
} ObjNative;
//< Calls and Functions obj-native
//> obj-string

struct ObjString {
  Obj obj;
  int length;
  char* chars;
//> Hash Tables obj-string-hash
  uint32_t hash;
//< Hash Tables obj-string-hash
};
//< obj-string
//> Closures obj-upvalue
typedef struct ObjUpvalue {
  Obj obj;
  Value* location;
//> closed-field
  Value closed;
//< closed-field
//> next-field
  struct ObjUpvalue* next;
//< next-field
} ObjUpvalue;
//< Closures obj-upvalue
//> Closures obj-closure
typedef struct {
  Obj obj;
  ObjFunction* function;
//> upvalue-fields
  ObjUpvalue** upvalues;
  int upvalueCount;
//< upvalue-fields
} ObjClosure;
//< Closures obj-closure
//> Classes and Instances obj-class

typedef struct {
  Obj obj;
  ObjString* name;
//> Methods and Initializers class-methods
  Table methods;
//< Methods and Initializers class-methods
} ObjClass;
//< Classes and Instances obj-class
//> Classes and Instances obj-instance

typedef struct {
  Obj obj;
  ObjClass* klass;
  Table fields; // [fields]
} ObjInstance;
//< Classes and Instances obj-instance

//> Methods and Initializers obj-bound-method
typedef struct {
  Obj obj;
  Value receiver;
  ObjClosure* method;
} ObjBoundMethod;

//< Methods and Initializers obj-bound-method
//> Methods and Initializers new-bound-method-h
ObjBoundMethod* newBoundMethod(Value receiver,
                               ObjClosure* method);
//< Methods and Initializers new-bound-method-h
//> Classes and Instances new-class-h
ObjClass* newClass(ObjString* name);
//< Classes and Instances new-class-h
//> Closures new-closure-h
ObjClosure* newClosure(ObjFunction* function);
//< Closures new-closure-h
//> Calls and Functions new-function-h
ObjFunction* newFunction();
//< Calls and Functions new-function-h
//> Classes and Instances new-instance-h
ObjInstance* newInstance(ObjClass* klass);
//< Classes and Instances new-instance-h
//> Calls and Functions new-native-h
ObjNative* newNative(NativeFn function);
//< Calls and Functions new-native-h
//> take-string-h
ObjString* takeString(char* chars, int length);
//< take-string-h
//> copy-string-h
ObjString* copyString(const char* chars, int length);
//> Closures new-upvalue-h
ObjUpvalue* newUpvalue(Value* slot);
//< Closures new-upvalue-h
//> print-object-h
void printObject(Value value);
//< print-object-h

//< copy-string-h
//> is-obj-type
static inline bool isObjType(Value value, ObjType type) {
  return IS_OBJ(value) && AS_OBJ(value)->type == type;
}

//< is-obj-type
#endif


//< Calls and Functions vm-include-object
//> Hash Tables vm-include-table
//< Hash Tables vm-include-table
//> vm-include-value
//< vm-include-value
//> stack-max

//< stack-max
/* A Virtual Machine stack-max < Calls and Functions frame-max
#define STACK_MAX 256
*/
//> Calls and Functions frame-max
#define FRAMES_MAX 64
#define STACK_MAX (FRAMES_MAX * UINT8_COUNT)
//< Calls and Functions frame-max
//> Calls and Functions call-frame

typedef struct {
/* Calls and Functions call-frame < Closures call-frame-closure
  ObjFunction* function;
*/
//> Closures call-frame-closure
  ObjClosure* closure;
//< Closures call-frame-closure
  uint8_t* ip;
  Value* slots;
} CallFrame;
//< Calls and Functions call-frame

typedef struct {
/* A Virtual Machine vm-h < Calls and Functions frame-array
  Chunk* chunk;
*/
/* A Virtual Machine ip < Calls and Functions frame-array
  uint8_t* ip;
*/
//> Calls and Functions frame-array
  CallFrame frames[FRAMES_MAX];
  int frameCount;

//< Calls and Functions frame-array
//> vm-stack
  Value stack[STACK_MAX];
  Value* stackTop;
//< vm-stack
//> Global Variables vm-globals
  Table globals;
//< Global Variables vm-globals
//> Hash Tables vm-strings
  Table strings;
//< Hash Tables vm-strings
//> Methods and Initializers vm-init-string
  ObjString* initString;
//< Methods and Initializers vm-init-string
//> Closures open-upvalues-field
  ObjUpvalue* openUpvalues;
//< Closures open-upvalues-field
//> Garbage Collection vm-fields

  size_t bytesAllocated;
  size_t nextGC;
//< Garbage Collection vm-fields
//> Strings objects-root
  Obj* objects;
//< Strings objects-root
//> Garbage Collection vm-gray-stack
  int grayCount;
  int grayCapacity;
  Obj** grayStack;
//< Garbage Collection vm-gray-stack
} VM;

//> interpret-result
typedef enum {
  INTERPRET_OK,
  INTERPRET_COMPILE_ERROR,
  INTERPRET_RUNTIME_ERROR
} InterpretResult;

//< interpret-result
//> Strings extern-vm
extern VM vm;

//< Strings extern-vm
void initVM();
void freeVM();
/* A Virtual Machine interpret-h < Scanning on Demand vm-interpret-h
InterpretResult interpret(Chunk* chunk);
*/
//> Scanning on Demand vm-interpret-h
InterpretResult interpret(const char* source);
//< Scanning on Demand vm-interpret-h
//> push-pop
void push(Value value);
Value pop();
//< push-pop

#endif



//> Chunks of Bytecode debug-h
#ifndef clox_debug_h
#define clox_debug_h


void disassembleChunk(Chunk* chunk, const char* name);
int disassembleInstruction(Chunk* chunk, int offset);

#endif



//> Scanning on Demand scanner-h
#ifndef clox_scanner_h
#define clox_scanner_h
//> token-type

typedef enum {
  // Single-character tokens.
  TOKEN_LEFT_PAREN, TOKEN_RIGHT_PAREN,
  TOKEN_LEFT_BRACE, TOKEN_RIGHT_BRACE,
  TOKEN_COMMA, TOKEN_DOT, TOKEN_MINUS, TOKEN_PLUS,
  TOKEN_SEMICOLON, TOKEN_SLASH, TOKEN_STAR,
  // One or two character tokens.
  TOKEN_BANG, TOKEN_BANG_EQUAL,
  TOKEN_EQUAL, TOKEN_EQUAL_EQUAL,
  TOKEN_GREATER, TOKEN_GREATER_EQUAL,
  TOKEN_LESS, TOKEN_LESS_EQUAL,
  // Literals.
  TOKEN_IDENTIFIER, TOKEN_STRING, TOKEN_NUMBER,
  // Keywords.
  TOKEN_AND, TOKEN_CLASS, TOKEN_ELSE, TOKEN_FALSE,
  TOKEN_FOR, TOKEN_FUN, TOKEN_IF, TOKEN_NIL, TOKEN_OR,
  TOKEN_PRINT, TOKEN_RETURN, TOKEN_SUPER, TOKEN_THIS,
  TOKEN_TRUE, TOKEN_VAR, TOKEN_WHILE,

  TOKEN_ERROR, TOKEN_EOF
} TokenType;
//< token-type
//> token-struct

typedef struct {
  TokenType type;
  const char* start;
  int length;
  int line;
} Token;
//< token-struct

void initScanner(const char* source);
//> scan-token-h
Token scanToken();
//< scan-token-h

#endif



//> Scanning on Demand compiler-h
#ifndef clox_compiler_h
#define clox_compiler_h

//> Strings compiler-include-object
//< Strings compiler-include-object
//> Compiling Expressions compile-h

//< Compiling Expressions compile-h
/* Scanning on Demand compiler-h < Compiling Expressions compile-h
void compile(const char* source);
*/
/* Compiling Expressions compile-h < Calls and Functions compile-h
bool compile(const char* source, Chunk* chunk);
*/
//> Calls and Functions compile-h
ObjFunction* compile(const char* source);
//< Calls and Functions compile-h
//> Garbage Collection mark-compiler-roots-h
void markCompilerRoots();
//< Garbage Collection mark-compiler-roots-h

#endif



//> Chunks of Bytecode memory-h
#ifndef clox_memory_h
#define clox_memory_h

//> Strings memory-include-object
//< Strings memory-include-object

//> Strings allocate
#define ALLOCATE(type, count) \
    (type*)reallocate(NULL, 0, sizeof(type) * (count))
//> free

#define FREE(type, pointer) reallocate(pointer, sizeof(type), 0)
//< free

//< Strings allocate
#define GROW_CAPACITY(capacity) \
    ((capacity) < 8 ? 8 : (capacity) * 2)
//> grow-array

#define GROW_ARRAY(type, pointer, oldCount, newCount) \
    (type*)reallocate(pointer, sizeof(type) * (oldCount), \
        sizeof(type) * (newCount))
//> free-array

#define FREE_ARRAY(type, pointer, oldCount) \
    reallocate(pointer, sizeof(type) * (oldCount), 0)
//< free-array

void* reallocate(void* pointer, size_t oldSize, size_t newSize);
//< grow-array
//> Garbage Collection mark-object-h
void markObject(Obj* object);
//< Garbage Collection mark-object-h
//> Garbage Collection mark-value-h
void markValue(Value value);
//< Garbage Collection mark-value-h
//> Garbage Collection collect-garbage-h
void collectGarbage();
//< Garbage Collection collect-garbage-h
//> Strings free-objects-h
void freeObjects();
//< Strings free-objects-h

#endif



//> Chunks of Bytecode chunk-c
#include <stdlib.h>

//> chunk-c-include-memory
//< chunk-c-include-memory
//> Garbage Collection chunk-include-vm
//< Garbage Collection chunk-include-vm

void initChunk(Chunk* chunk) {
  chunk->count = 0;
  chunk->capacity = 0;
  chunk->code = NULL;
//> chunk-null-lines
  chunk->lines = NULL;
//< chunk-null-lines
//> chunk-init-constant-array
  initValueArray(&chunk->constants);
//< chunk-init-constant-array
}
//> free-chunk
void freeChunk(Chunk* chunk) {
  FREE_ARRAY(uint8_t, chunk->code, chunk->capacity);
//> chunk-free-lines
  FREE_ARRAY(int, chunk->lines, chunk->capacity);
//< chunk-free-lines
//> chunk-free-constants
  freeValueArray(&chunk->constants);
//< chunk-free-constants
  initChunk(chunk);
}
//< free-chunk
/* Chunks of Bytecode write-chunk < Chunks of Bytecode write-chunk-with-line
void writeChunk(Chunk* chunk, uint8_t byte) {
*/
//> write-chunk
//> write-chunk-with-line
void writeChunk(Chunk* chunk, uint8_t byte, int line) {
//< write-chunk-with-line
  if (chunk->capacity < chunk->count + 1) {
    int oldCapacity = chunk->capacity;
    chunk->capacity = GROW_CAPACITY(oldCapacity);
    chunk->code = GROW_ARRAY(uint8_t, chunk->code,
        oldCapacity, chunk->capacity);
//> write-chunk-line
    chunk->lines = GROW_ARRAY(int, chunk->lines,
        oldCapacity, chunk->capacity);
//< write-chunk-line
  }

  chunk->code[chunk->count] = byte;
//> chunk-write-line
  chunk->lines[chunk->count] = line;
//< chunk-write-line
  chunk->count++;
}
//< write-chunk
//> add-constant
int addConstant(Chunk* chunk, Value value) {
//> Garbage Collection add-constant-push
  push(value);
//< Garbage Collection add-constant-push
  writeValueArray(&chunk->constants, value);
//> Garbage Collection add-constant-pop
  pop();
//< Garbage Collection add-constant-pop
  return chunk->constants.count - 1;
}
//< add-constant



//> Scanning on Demand compiler-c
#include <stdio.h>
//> Compiling Expressions compiler-include-stdlib
#include <stdlib.h>
//< Compiling Expressions compiler-include-stdlib
//> Local Variables compiler-include-string
#include <string.h>
//< Local Variables compiler-include-string

//> Garbage Collection compiler-include-memory
//< Garbage Collection compiler-include-memory
//> Compiling Expressions include-debug

#ifdef DEBUG_PRINT_CODE
#endif
//< Compiling Expressions include-debug
//> Compiling Expressions parser

typedef struct {
  Token current;
  Token previous;
//> had-error-field
  bool hadError;
//< had-error-field
//> panic-mode-field
  bool panicMode;
//< panic-mode-field
} Parser;
//> precedence

typedef enum {
  PREC_NONE,
  PREC_ASSIGNMENT,  // =
  PREC_OR,          // or
  PREC_AND,         // and
  PREC_EQUALITY,    // == !=
  PREC_COMPARISON,  // < > <= >=
  PREC_TERM,        // + -
  PREC_FACTOR,      // * /
  PREC_UNARY,       // ! -
  PREC_CALL,        // . ()
  PREC_PRIMARY
} Precedence;
//< precedence
//> parse-fn-type

//< parse-fn-type
/* Compiling Expressions parse-fn-type < Global Variables parse-fn-type
typedef void (*ParseFn)();
*/
//> Global Variables parse-fn-type
typedef void (*ParseFn)(bool canAssign);
//< Global Variables parse-fn-type
//> parse-rule

typedef struct {
  ParseFn prefix;
  ParseFn infix;
  Precedence precedence;
} ParseRule;
//< parse-rule
//> Local Variables local-struct

typedef struct {
  Token name;
  int depth;
//> Closures is-captured-field
  bool isCaptured;
//< Closures is-captured-field
} Local;
//< Local Variables local-struct
//> Closures upvalue-struct
typedef struct {
  uint8_t index;
  bool isLocal;
} Upvalue;
//< Closures upvalue-struct
//> Calls and Functions function-type-enum
typedef enum {
  TYPE_FUNCTION,
//> Methods and Initializers initializer-type-enum
  TYPE_INITIALIZER,
//< Methods and Initializers initializer-type-enum
//> Methods and Initializers method-type-enum
  TYPE_METHOD,
//< Methods and Initializers method-type-enum
  TYPE_SCRIPT
} FunctionType;
//< Calls and Functions function-type-enum
//> Local Variables compiler-struct

/* Local Variables compiler-struct < Calls and Functions enclosing-field
typedef struct {
*/
//> Calls and Functions enclosing-field
typedef struct Compiler {
  struct Compiler* enclosing;
//< Calls and Functions enclosing-field
//> Calls and Functions function-fields
  ObjFunction* function;
  FunctionType type;

//< Calls and Functions function-fields
  Local locals[UINT8_COUNT];
  int localCount;
//> Closures upvalues-array
  Upvalue upvalues[UINT8_COUNT];
//< Closures upvalues-array
  int scopeDepth;
} Compiler;
//< Local Variables compiler-struct
//> Methods and Initializers class-compiler-struct

typedef struct ClassCompiler {
  struct ClassCompiler* enclosing;
//> Superclasses has-superclass
  bool hasSuperclass;
//< Superclasses has-superclass
} ClassCompiler;
//< Methods and Initializers class-compiler-struct

Parser parser;
//< Compiling Expressions parser
//> Local Variables current-compiler
Compiler* current = NULL;
//< Local Variables current-compiler
//> Methods and Initializers current-class
ClassCompiler* currentClass = NULL;
//< Methods and Initializers current-class
//> Compiling Expressions compiling-chunk
/* Compiling Expressions compiling-chunk < Calls and Functions current-chunk
Chunk* compilingChunk;

static Chunk* currentChunk() {
  return compilingChunk;
}
*/
//> Calls and Functions current-chunk

static Chunk* currentChunk() {
  return &current->function->chunk;
}
//< Calls and Functions current-chunk

//< Compiling Expressions compiling-chunk
//> Compiling Expressions error-at
static void errorAt(Token* token, const char* message) {
//> check-panic-mode
  if (parser.panicMode) return;
//< check-panic-mode
//> set-panic-mode
  parser.panicMode = true;
//< set-panic-mode
  fprintf(stderr, "[line %d] Error", token->line);

  if (token->type == TOKEN_EOF) {
    fprintf(stderr, " at end");
  } else if (token->type == TOKEN_ERROR) {
    // Nothing.
  } else {
    fprintf(stderr, " at '%.*s'", token->length, token->start);
  }

  fprintf(stderr, ": %s\n", message);
  parser.hadError = true;
}
//< Compiling Expressions error-at
//> Compiling Expressions error
static void error(const char* message) {
  errorAt(&parser.previous, message);
}
//< Compiling Expressions error
//> Compiling Expressions error-at-current
static void errorAtCurrent(const char* message) {
  errorAt(&parser.current, message);
}
//< Compiling Expressions error-at-current
//> Compiling Expressions advance

static void advance_compiler() {
  parser.previous = parser.current;

  for (;;) {
    parser.current = scanToken();
    if (parser.current.type != TOKEN_ERROR) break;

    errorAtCurrent(parser.current.start);
  }
}
//< Compiling Expressions advance
//> Compiling Expressions consume
static void consume(TokenType type, const char* message) {
  if (parser.current.type == type) {
    advance_compiler();
    return;
  }

  errorAtCurrent(message);
}
//< Compiling Expressions consume
//> Global Variables check
static bool check(TokenType type) {
  return parser.current.type == type;
}
//< Global Variables check
//> Global Variables match
static bool match_compiler(TokenType type) {
  if (!check(type)) return false;
  advance_compiler();
  return true;
}
//< Global Variables match
//> Compiling Expressions emit-byte
static void emitByte(uint8_t byte) {
  writeChunk(currentChunk(), byte, parser.previous.line);
}
//< Compiling Expressions emit-byte
//> Compiling Expressions emit-bytes
static void emitBytes(uint8_t byte1, uint8_t byte2) {
  emitByte(byte1);
  emitByte(byte2);
}
//< Compiling Expressions emit-bytes
//> Jumping Back and Forth emit-loop
static void emitLoop(int loopStart) {
  emitByte(OP_LOOP);

  int offset = currentChunk()->count - loopStart + 2;
  if (offset > UINT16_MAX) error("Loop body too large.");

  emitByte((offset >> 8) & 0xff);
  emitByte(offset & 0xff);
}
//< Jumping Back and Forth emit-loop
//> Jumping Back and Forth emit-jump
static int emitJump(uint8_t instruction) {
  emitByte(instruction);
  emitByte(0xff);
  emitByte(0xff);
  return currentChunk()->count - 2;
}
//< Jumping Back and Forth emit-jump
//> Compiling Expressions emit-return
static void emitReturn() {
/* Calls and Functions return-nil < Methods and Initializers return-this
  emitByte(OP_NIL);
*/
//> Methods and Initializers return-this
  if (current->type == TYPE_INITIALIZER) {
    emitBytes(OP_GET_LOCAL, 0);
  } else {
    emitByte(OP_NIL);
  }

//< Methods and Initializers return-this
  emitByte(OP_RETURN);
}
//< Compiling Expressions emit-return
//> Compiling Expressions make-constant
static uint8_t makeConstant(Value value) {
  int constant = addConstant(currentChunk(), value);
  if (constant > UINT8_MAX) {
    error("Too many constants in one chunk.");
    return 0;
  }

  return (uint8_t)constant;
}
//< Compiling Expressions make-constant
//> Compiling Expressions emit-constant
static void emitConstant(Value value) {
  emitBytes(OP_CONSTANT, makeConstant(value));
}
//< Compiling Expressions emit-constant
//> Jumping Back and Forth patch-jump
static void patchJump(int offset) {
  // -2 to adjust for the bytecode for the jump offset itself.
  int jump = currentChunk()->count - offset - 2;

  if (jump > UINT16_MAX) {
    error("Too much code to jump over.");
  }

  currentChunk()->code[offset] = (jump >> 8) & 0xff;
  currentChunk()->code[offset + 1] = jump & 0xff;
}
//< Jumping Back and Forth patch-jump
//> Local Variables init-compiler
/* Local Variables init-compiler < Calls and Functions init-compiler
static void initCompiler(Compiler* compiler) {
*/
//> Calls and Functions init-compiler
static void initCompiler(Compiler* compiler, FunctionType type) {
//> store-enclosing
  compiler->enclosing = current;
//< store-enclosing
  compiler->function = NULL;
  compiler->type = type;
//< Calls and Functions init-compiler
  compiler->localCount = 0;
  compiler->scopeDepth = 0;
//> Calls and Functions init-function
  compiler->function = newFunction();
//< Calls and Functions init-function
  current = compiler;
//> Calls and Functions init-function-name
  if (type != TYPE_SCRIPT) {
    current->function->name = copyString(parser.previous.start,
                                         parser.previous.length);
  }
//< Calls and Functions init-function-name
//> Calls and Functions init-function-slot

  Local* local = &current->locals[current->localCount++];
  local->depth = 0;
//> Closures init-zero-local-is-captured
  local->isCaptured = false;
//< Closures init-zero-local-is-captured
/* Calls and Functions init-function-slot < Methods and Initializers slot-zero
  local->name.start = "";
  local->name.length = 0;
*/
//> Methods and Initializers slot-zero
  if (type != TYPE_FUNCTION) {
    local->name.start = "this";
    local->name.length = 4;
  } else {
    local->name.start = "";
    local->name.length = 0;
  }
//< Methods and Initializers slot-zero
//< Calls and Functions init-function-slot
}
//< Local Variables init-compiler
//> Compiling Expressions end-compiler
/* Compiling Expressions end-compiler < Calls and Functions end-compiler
static void endCompiler() {
*/
//> Calls and Functions end-compiler
static ObjFunction* endCompiler() {
//< Calls and Functions end-compiler
  emitReturn();
//> Calls and Functions end-function
  ObjFunction* function = current->function;

//< Calls and Functions end-function
//> dump-chunk
#ifdef DEBUG_PRINT_CODE
  if (!parser.hadError) {
/* Compiling Expressions dump-chunk < Calls and Functions disassemble-end
    disassembleChunk(currentChunk(), "code");
*/
//> Calls and Functions disassemble-end
    disassembleChunk(currentChunk(), function->name != NULL
        ? function->name->chars : "<script>");
//< Calls and Functions disassemble-end
  }
#endif
//< dump-chunk
//> Calls and Functions return-function

//> restore-enclosing
  current = current->enclosing;
//< restore-enclosing
  return function;
//< Calls and Functions return-function
}
//< Compiling Expressions end-compiler
//> Local Variables begin-scope
static void beginScope() {
  current->scopeDepth++;
}
//< Local Variables begin-scope
//> Local Variables end-scope
static void endScope() {
  current->scopeDepth--;
//> pop-locals

  while (current->localCount > 0 &&
         current->locals[current->localCount - 1].depth >
            current->scopeDepth) {
/* Local Variables pop-locals < Closures end-scope
    emitByte(OP_POP);
*/
//> Closures end-scope
    if (current->locals[current->localCount - 1].isCaptured) {
      emitByte(OP_CLOSE_UPVALUE);
    } else {
      emitByte(OP_POP);
    }
//< Closures end-scope
    current->localCount--;
  }
//< pop-locals
}
//< Local Variables end-scope
//> Compiling Expressions forward-declarations

static void expression();
//> Global Variables forward-declarations
static void statement();
static void declaration();
//< Global Variables forward-declarations
static ParseRule* getRule(TokenType type);
static void parsePrecedence(Precedence precedence);

//< Compiling Expressions forward-declarations
//> Global Variables identifier-constant
static uint8_t identifierConstant(Token* name) {
  return makeConstant(OBJ_VAL(copyString(name->start,
                                         name->length)));
}
//< Global Variables identifier-constant
//> Local Variables identifiers-equal
static bool identifiersEqual(Token* a, Token* b) {
  if (a->length != b->length) return false;
  return memcmp(a->start, b->start, a->length) == 0;
}
//< Local Variables identifiers-equal
//> Local Variables resolve-local
static int resolveLocal(Compiler* compiler, Token* name) {
  for (int i = compiler->localCount - 1; i >= 0; i--) {
    Local* local = &compiler->locals[i];
    if (identifiersEqual(name, &local->name)) {
//> own-initializer-error
      if (local->depth == -1) {
        error("Can't read local variable in its own initializer.");
      }
//< own-initializer-error
      return i;
    }
  }

  return -1;
}
//< Local Variables resolve-local
//> Closures add-upvalue
static int addUpvalue(Compiler* compiler, uint8_t index,
                      bool isLocal) {
  int upvalueCount = compiler->function->upvalueCount;
//> existing-upvalue

  for (int i = 0; i < upvalueCount; i++) {
    Upvalue* upvalue = &compiler->upvalues[i];
    if (upvalue->index == index && upvalue->isLocal == isLocal) {
      return i;
    }
  }

//< existing-upvalue
//> too-many-upvalues
  if (upvalueCount == UINT8_COUNT) {
    error("Too many closure variables in function.");
    return 0;
  }

//< too-many-upvalues
  compiler->upvalues[upvalueCount].isLocal = isLocal;
  compiler->upvalues[upvalueCount].index = index;
  return compiler->function->upvalueCount++;
}
//< Closures add-upvalue
//> Closures resolve-upvalue
static int resolveUpvalue(Compiler* compiler, Token* name) {
  if (compiler->enclosing == NULL) return -1;

  int local = resolveLocal(compiler->enclosing, name);
  if (local != -1) {
//> mark-local-captured
    compiler->enclosing->locals[local].isCaptured = true;
//< mark-local-captured
    return addUpvalue(compiler, (uint8_t)local, true);
  }

//> resolve-upvalue-recurse
  int upvalue = resolveUpvalue(compiler->enclosing, name);
  if (upvalue != -1) {
    return addUpvalue(compiler, (uint8_t)upvalue, false);
  }

//< resolve-upvalue-recurse
  return -1;
}
//< Closures resolve-upvalue
//> Local Variables add-local
static void addLocal(Token name) {
//> too-many-locals
  if (current->localCount == UINT8_COUNT) {
    error("Too many local variables in function.");
    return;
  }

//< too-many-locals
  Local* local = &current->locals[current->localCount++];
  local->name = name;
/* Local Variables add-local < Local Variables declare-undefined
  local->depth = current->scopeDepth;
*/
//> declare-undefined
  local->depth = -1;
//< declare-undefined
//> Closures init-is-captured
  local->isCaptured = false;
//< Closures init-is-captured
}
//< Local Variables add-local
//> Local Variables declare-variable
static void declareVariable() {
  if (current->scopeDepth == 0) return;

  Token* name = &parser.previous;
//> existing-in-scope
  for (int i = current->localCount - 1; i >= 0; i--) {
    Local* local = &current->locals[i];
    if (local->depth != -1 && local->depth < current->scopeDepth) {
      break; // [negative]
    }

    if (identifiersEqual(name, &local->name)) {
      error("Already a variable with this name in this scope.");
    }
  }

//< existing-in-scope
  addLocal(*name);
}
//< Local Variables declare-variable
//> Global Variables parse-variable
static uint8_t parseVariable(const char* errorMessage) {
  consume(TOKEN_IDENTIFIER, errorMessage);
//> Local Variables parse-local

  declareVariable();
  if (current->scopeDepth > 0) return 0;

//< Local Variables parse-local
  return identifierConstant(&parser.previous);
}
//< Global Variables parse-variable
//> Local Variables mark-initialized
static void markInitialized() {
//> Calls and Functions check-depth
  if (current->scopeDepth == 0) return;
//< Calls and Functions check-depth
  current->locals[current->localCount - 1].depth =
      current->scopeDepth;
}
//< Local Variables mark-initialized
//> Global Variables define-variable
static void defineVariable(uint8_t global) {
//> Local Variables define-variable
  if (current->scopeDepth > 0) {
//> define-local
    markInitialized();
//< define-local
    return;
  }

//< Local Variables define-variable
  emitBytes(OP_DEFINE_GLOBAL, global);
}
//< Global Variables define-variable
//> Calls and Functions argument-list
static uint8_t argumentList() {
  uint8_t argCount = 0;
  if (!check(TOKEN_RIGHT_PAREN)) {
    do {
      expression();
//> arg-limit
      if (argCount == 255) {
        error("Can't have more than 255 arguments.");
      }
//< arg-limit
      argCount++;
    } while (match_compiler(TOKEN_COMMA));
  }
  consume(TOKEN_RIGHT_PAREN, "Expect ')' after arguments.");
  return argCount;
}
//< Calls and Functions argument-list
//> Jumping Back and Forth and
static void and_(bool canAssign) {
  int endJump = emitJump(OP_JUMP_IF_FALSE);

  emitByte(OP_POP);
  parsePrecedence(PREC_AND);

  patchJump(endJump);
}
//< Jumping Back and Forth and
//> Compiling Expressions binary
/* Compiling Expressions binary < Global Variables binary
static void binary() {
*/
//> Global Variables binary
static void binary(bool canAssign) {
//< Global Variables binary
  TokenType operatorType = parser.previous.type;
  ParseRule* rule = getRule(operatorType);
  parsePrecedence((Precedence)(rule->precedence + 1));

  switch (operatorType) {
//> Types of Values comparison-operators
    case TOKEN_BANG_EQUAL:    emitBytes(OP_EQUAL, OP_NOT); break;
    case TOKEN_EQUAL_EQUAL:   emitByte(OP_EQUAL); break;
    case TOKEN_GREATER:       emitByte(OP_GREATER); break;
    case TOKEN_GREATER_EQUAL: emitBytes(OP_LESS, OP_NOT); break;
    case TOKEN_LESS:          emitByte(OP_LESS); break;
    case TOKEN_LESS_EQUAL:    emitBytes(OP_GREATER, OP_NOT); break;
//< Types of Values comparison-operators
    case TOKEN_PLUS:          emitByte(OP_ADD); break;
    case TOKEN_MINUS:         emitByte(OP_SUBTRACT); break;
    case TOKEN_STAR:          emitByte(OP_MULTIPLY); break;
    case TOKEN_SLASH:         emitByte(OP_DIVIDE); break;
    default: return; // Unreachable.
  }
}
//< Compiling Expressions binary
//> Calls and Functions compile-call
static void call_compiler(bool canAssign) {
  uint8_t argCount = argumentList();
  emitBytes(OP_CALL, argCount);
}
//< Calls and Functions compile-call
//> Classes and Instances compile-dot
static void dot(bool canAssign) {
  consume(TOKEN_IDENTIFIER, "Expect property name after '.'.");
  uint8_t name = identifierConstant(&parser.previous);

  if (canAssign && match_compiler(TOKEN_EQUAL)) {
    expression();
    emitBytes(OP_SET_PROPERTY, name);
//> Methods and Initializers parse-call
  } else if (match_compiler(TOKEN_LEFT_PAREN)) {
    uint8_t argCount = argumentList();
    emitBytes(OP_INVOKE, name);
    emitByte(argCount);
//< Methods and Initializers parse-call
  } else {
    emitBytes(OP_GET_PROPERTY, name);
  }
}
//< Classes and Instances compile-dot
//> Types of Values parse-literal
/* Types of Values parse-literal < Global Variables parse-literal
static void literal() {
*/
//> Global Variables parse-literal
static void literal(bool canAssign) {
//< Global Variables parse-literal
  switch (parser.previous.type) {
    case TOKEN_FALSE: emitByte(OP_FALSE); break;
    case TOKEN_NIL: emitByte(OP_NIL); break;
    case TOKEN_TRUE: emitByte(OP_TRUE); break;
    default: return; // Unreachable.
  }
}
//< Types of Values parse-literal
//> Compiling Expressions grouping
/* Compiling Expressions grouping < Global Variables grouping
static void grouping() {
*/
//> Global Variables grouping
static void grouping(bool canAssign) {
//< Global Variables grouping
  expression();
  consume(TOKEN_RIGHT_PAREN, "Expect ')' after expression.");
}
//< Compiling Expressions grouping
/* Compiling Expressions number < Global Variables number
static void number() {
*/
//> Compiling Expressions number
//> Global Variables number
static void number_compiler(bool canAssign) {
//< Global Variables number
  double value = strtod(parser.previous.start, NULL);
/* Compiling Expressions number < Types of Values const-number-val
  emitConstant(value);
*/
//> Types of Values const-number-val
  emitConstant(NUMBER_VAL(value));
//< Types of Values const-number-val
}
//< Compiling Expressions number
//> Jumping Back and Forth or
static void or_(bool canAssign) {
  int elseJump = emitJump(OP_JUMP_IF_FALSE);
  int endJump = emitJump(OP_JUMP);

  patchJump(elseJump);
  emitByte(OP_POP);

  parsePrecedence(PREC_OR);
  patchJump(endJump);
}
//< Jumping Back and Forth or
/* Strings parse-string < Global Variables string
static void string() {
*/
//> Strings parse-string
//> Global Variables string
static void string_compiler(bool canAssign) {
//< Global Variables string
  emitConstant(OBJ_VAL(copyString(parser.previous.start + 1,
                                  parser.previous.length - 2)));
}
//< Strings parse-string
/* Global Variables read-named-variable < Global Variables named-variable-signature
static void namedVariable(Token name) {
*/
//> Global Variables named-variable-signature
static void namedVariable(Token name, bool canAssign) {
//< Global Variables named-variable-signature
/* Global Variables read-named-variable < Local Variables named-local
  uint8_t arg = identifierConstant(&name);
*/
//> Global Variables read-named-variable
//> Local Variables named-local
  uint8_t getOp, setOp;
  int arg = resolveLocal(current, &name);
  if (arg != -1) {
    getOp = OP_GET_LOCAL;
    setOp = OP_SET_LOCAL;
//> Closures named-variable-upvalue
  } else if ((arg = resolveUpvalue(current, &name)) != -1) {
    getOp = OP_GET_UPVALUE;
    setOp = OP_SET_UPVALUE;
//< Closures named-variable-upvalue
  } else {
    arg = identifierConstant(&name);
    getOp = OP_GET_GLOBAL;
    setOp = OP_SET_GLOBAL;
  }
//< Local Variables named-local
/* Global Variables read-named-variable < Global Variables named-variable
  emitBytes(OP_GET_GLOBAL, arg);
*/
//> named-variable

/* Global Variables named-variable < Global Variables named-variable-can-assign
  if (match(TOKEN_EQUAL)) {
*/
//> named-variable-can-assign
  if (canAssign && match_compiler(TOKEN_EQUAL)) {
//< named-variable-can-assign
    expression();
/* Global Variables named-variable < Local Variables emit-set
    emitBytes(OP_SET_GLOBAL, arg);
*/
//> Local Variables emit-set
    emitBytes(setOp, (uint8_t)arg);
//< Local Variables emit-set
  } else {
/* Global Variables named-variable < Local Variables emit-get
    emitBytes(OP_GET_GLOBAL, arg);
*/
//> Local Variables emit-get
    emitBytes(getOp, (uint8_t)arg);
//< Local Variables emit-get
  }
//< named-variable
}
//< Global Variables read-named-variable
/* Global Variables variable-without-assign < Global Variables variable
static void variable() {
  namedVariable(parser.previous);
}
*/
//> Global Variables variable
static void variable(bool canAssign) {
  namedVariable(parser.previous, canAssign);
}
//< Global Variables variable
//> Superclasses synthetic-token
static Token syntheticToken(const char* text) {
  Token token;
  token.start = text;
  token.length = (int)strlen(text);
  return token;
}
//< Superclasses synthetic-token
//> Superclasses super
static void super_(bool canAssign) {
//> super-errors
  if (currentClass == NULL) {
    error("Can't use 'super' outside of a class.");
  } else if (!currentClass->hasSuperclass) {
    error("Can't use 'super' in a class with no superclass.");
  }

//< super-errors
  consume(TOKEN_DOT, "Expect '.' after 'super'.");
  consume(TOKEN_IDENTIFIER, "Expect superclass method name.");
  uint8_t name = identifierConstant(&parser.previous);
//> super-get

  namedVariable(syntheticToken("this"), false);
/* Superclasses super-get < Superclasses super-invoke
  namedVariable(syntheticToken("super"), false);
  emitBytes(OP_GET_SUPER, name);
*/
//< super-get
//> super-invoke
  if (match_compiler(TOKEN_LEFT_PAREN)) {
    uint8_t argCount = argumentList();
    namedVariable(syntheticToken("super"), false);
    emitBytes(OP_SUPER_INVOKE, name);
    emitByte(argCount);
  } else {
    namedVariable(syntheticToken("super"), false);
    emitBytes(OP_GET_SUPER, name);
  }
//< super-invoke
}
//< Superclasses super
//> Methods and Initializers this
static void this_(bool canAssign) {
//> this-outside-class
  if (currentClass == NULL) {
    error("Can't use 'this' outside of a class.");
    return;
  }

//< this-outside-class
  variable(false);
} // [this]
//< Methods and Initializers this
//> Compiling Expressions unary
/* Compiling Expressions unary < Global Variables unary
static void unary() {
*/
//> Global Variables unary
static void unary(bool canAssign) {
//< Global Variables unary
  TokenType operatorType = parser.previous.type;

  // Compile the operand.
/* Compiling Expressions unary < Compiling Expressions unary-operand
  expression();
*/
//> unary-operand
  parsePrecedence(PREC_UNARY);
//< unary-operand

  // Emit the operator instruction.
  switch (operatorType) {
//> Types of Values compile-not
    case TOKEN_BANG: emitByte(OP_NOT); break;
//< Types of Values compile-not
    case TOKEN_MINUS: emitByte(OP_NEGATE); break;
    default: return; // Unreachable.
  }
}
//< Compiling Expressions unary
//> Compiling Expressions rules
ParseRule rules[] = {
/* Compiling Expressions rules < Calls and Functions infix-left-paren
  [TOKEN_LEFT_PAREN]    = {grouping, NULL,   PREC_NONE},
*/
//> Calls and Functions infix-left-paren
  [TOKEN_LEFT_PAREN]    = {grouping, call_compiler,   PREC_CALL},
//< Calls and Functions infix-left-paren
  [TOKEN_RIGHT_PAREN]   = {NULL,     NULL,   PREC_NONE},
  [TOKEN_LEFT_BRACE]    = {NULL,     NULL,   PREC_NONE}, // [big]
  [TOKEN_RIGHT_BRACE]   = {NULL,     NULL,   PREC_NONE},
  [TOKEN_COMMA]         = {NULL,     NULL,   PREC_NONE},
/* Compiling Expressions rules < Classes and Instances table-dot
  [TOKEN_DOT]           = {NULL,     NULL,   PREC_NONE},
*/
//> Classes and Instances table-dot
  [TOKEN_DOT]           = {NULL,     dot,    PREC_CALL},
//< Classes and Instances table-dot
  [TOKEN_MINUS]         = {unary,    binary, PREC_TERM},
  [TOKEN_PLUS]          = {NULL,     binary, PREC_TERM},
  [TOKEN_SEMICOLON]     = {NULL,     NULL,   PREC_NONE},
  [TOKEN_SLASH]         = {NULL,     binary, PREC_FACTOR},
  [TOKEN_STAR]          = {NULL,     binary, PREC_FACTOR},
/* Compiling Expressions rules < Types of Values table-not
  [TOKEN_BANG]          = {NULL,     NULL,   PREC_NONE},
*/
//> Types of Values table-not
  [TOKEN_BANG]          = {unary,    NULL,   PREC_NONE},
//< Types of Values table-not
/* Compiling Expressions rules < Types of Values table-equal
  [TOKEN_BANG_EQUAL]    = {NULL,     NULL,   PREC_NONE},
*/
//> Types of Values table-equal
  [TOKEN_BANG_EQUAL]    = {NULL,     binary, PREC_EQUALITY},
//< Types of Values table-equal
  [TOKEN_EQUAL]         = {NULL,     NULL,   PREC_NONE},
/* Compiling Expressions rules < Types of Values table-comparisons
  [TOKEN_EQUAL_EQUAL]   = {NULL,     NULL,   PREC_NONE},
  [TOKEN_GREATER]       = {NULL,     NULL,   PREC_NONE},
  [TOKEN_GREATER_EQUAL] = {NULL,     NULL,   PREC_NONE},
  [TOKEN_LESS]          = {NULL,     NULL,   PREC_NONE},
  [TOKEN_LESS_EQUAL]    = {NULL,     NULL,   PREC_NONE},
*/
//> Types of Values table-comparisons
  [TOKEN_EQUAL_EQUAL]   = {NULL,     binary, PREC_EQUALITY},
  [TOKEN_GREATER]       = {NULL,     binary, PREC_COMPARISON},
  [TOKEN_GREATER_EQUAL] = {NULL,     binary, PREC_COMPARISON},
  [TOKEN_LESS]          = {NULL,     binary, PREC_COMPARISON},
  [TOKEN_LESS_EQUAL]    = {NULL,     binary, PREC_COMPARISON},
//< Types of Values table-comparisons
/* Compiling Expressions rules < Global Variables table-identifier
  [TOKEN_IDENTIFIER]    = {NULL,     NULL,   PREC_NONE},
*/
//> Global Variables table-identifier
  [TOKEN_IDENTIFIER]    = {variable, NULL,   PREC_NONE},
//< Global Variables table-identifier
/* Compiling Expressions rules < Strings table-string
  [TOKEN_STRING]        = {NULL,     NULL,   PREC_NONE},
*/
//> Strings table-string
  [TOKEN_STRING]        = {string_compiler,   NULL,   PREC_NONE},
//< Strings table-string
  [TOKEN_NUMBER]        = {number_compiler,   NULL,   PREC_NONE},
/* Compiling Expressions rules < Jumping Back and Forth table-and
  [TOKEN_AND]           = {NULL,     NULL,   PREC_NONE},
*/
//> Jumping Back and Forth table-and
  [TOKEN_AND]           = {NULL,     and_,   PREC_AND},
//< Jumping Back and Forth table-and
  [TOKEN_CLASS]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_ELSE]          = {NULL,     NULL,   PREC_NONE},
/* Compiling Expressions rules < Types of Values table-false
  [TOKEN_FALSE]         = {NULL,     NULL,   PREC_NONE},
*/
//> Types of Values table-false
  [TOKEN_FALSE]         = {literal,  NULL,   PREC_NONE},
//< Types of Values table-false
  [TOKEN_FOR]           = {NULL,     NULL,   PREC_NONE},
  [TOKEN_FUN]           = {NULL,     NULL,   PREC_NONE},
  [TOKEN_IF]            = {NULL,     NULL,   PREC_NONE},
/* Compiling Expressions rules < Types of Values table-nil
  [TOKEN_NIL]           = {NULL,     NULL,   PREC_NONE},
*/
//> Types of Values table-nil
  [TOKEN_NIL]           = {literal,  NULL,   PREC_NONE},
//< Types of Values table-nil
/* Compiling Expressions rules < Jumping Back and Forth table-or
  [TOKEN_OR]            = {NULL,     NULL,   PREC_NONE},
*/
//> Jumping Back and Forth table-or
  [TOKEN_OR]            = {NULL,     or_,    PREC_OR},
//< Jumping Back and Forth table-or
  [TOKEN_PRINT]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_RETURN]        = {NULL,     NULL,   PREC_NONE},
/* Compiling Expressions rules < Superclasses table-super
  [TOKEN_SUPER]         = {NULL,     NULL,   PREC_NONE},
*/
//> Superclasses table-super
  [TOKEN_SUPER]         = {super_,   NULL,   PREC_NONE},
//< Superclasses table-super
/* Compiling Expressions rules < Methods and Initializers table-this
  [TOKEN_THIS]          = {NULL,     NULL,   PREC_NONE},
*/
//> Methods and Initializers table-this
  [TOKEN_THIS]          = {this_,    NULL,   PREC_NONE},
//< Methods and Initializers table-this
/* Compiling Expressions rules < Types of Values table-true
  [TOKEN_TRUE]          = {NULL,     NULL,   PREC_NONE},
*/
//> Types of Values table-true
  [TOKEN_TRUE]          = {literal,  NULL,   PREC_NONE},
//< Types of Values table-true
  [TOKEN_VAR]           = {NULL,     NULL,   PREC_NONE},
  [TOKEN_WHILE]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_ERROR]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_EOF]           = {NULL,     NULL,   PREC_NONE},
};
//< Compiling Expressions rules
//> Compiling Expressions parse-precedence
static void parsePrecedence(Precedence precedence) {
/* Compiling Expressions parse-precedence < Compiling Expressions precedence-body
  // What goes here?
*/
//> precedence-body
  advance_compiler();
  ParseFn prefixRule = getRule(parser.previous.type)->prefix;
  if (prefixRule == NULL) {
    error("Expect expression.");
    return;
  }

/* Compiling Expressions precedence-body < Global Variables prefix-rule
  prefixRule();
*/
//> Global Variables prefix-rule
  bool canAssign = precedence <= PREC_ASSIGNMENT;
  prefixRule(canAssign);
//< Global Variables prefix-rule
//> infix

  while (precedence <= getRule(parser.current.type)->precedence) {
    advance_compiler();
    ParseFn infixRule = getRule(parser.previous.type)->infix;
/* Compiling Expressions infix < Global Variables infix-rule
    infixRule();
*/
//> Global Variables infix-rule
    infixRule(canAssign);
//< Global Variables infix-rule
  }
//> Global Variables invalid-assign

  if (canAssign && match_compiler(TOKEN_EQUAL)) {
    error("Invalid assignment target.");
  }
//< Global Variables invalid-assign
//< infix
//< precedence-body
}
//< Compiling Expressions parse-precedence
//> Compiling Expressions get-rule
static ParseRule* getRule(TokenType type) {
  return &rules[type];
}
//< Compiling Expressions get-rule
//> Compiling Expressions expression
static void expression() {
/* Compiling Expressions expression < Compiling Expressions expression-body
  // What goes here?
*/
//> expression-body
  parsePrecedence(PREC_ASSIGNMENT);
//< expression-body
}
//< Compiling Expressions expression
//> Local Variables block
static void block() {
  while (!check(TOKEN_RIGHT_BRACE) && !check(TOKEN_EOF)) {
    declaration();
  }

  consume(TOKEN_RIGHT_BRACE, "Expect '}' after block.");
}
//< Local Variables block
//> Calls and Functions compile-function
static void function(FunctionType type) {
  Compiler compiler;
  initCompiler(&compiler, type);
  beginScope(); // [no-end-scope]

  consume(TOKEN_LEFT_PAREN, "Expect '(' after function name.");
//> parameters
  if (!check(TOKEN_RIGHT_PAREN)) {
    do {
      current->function->arity++;
      if (current->function->arity > 255) {
        errorAtCurrent("Can't have more than 255 parameters.");
      }
      uint8_t constant = parseVariable("Expect parameter name.");
      defineVariable(constant);
    } while (match_compiler(TOKEN_COMMA));
  }
//< parameters
  consume(TOKEN_RIGHT_PAREN, "Expect ')' after parameters.");
  consume(TOKEN_LEFT_BRACE, "Expect '{' before function body.");
  block();

  ObjFunction* function = endCompiler();
/* Calls and Functions compile-function < Closures emit-closure
  emitBytes(OP_CONSTANT, makeConstant(OBJ_VAL(function)));
*/
//> Closures emit-closure
  emitBytes(OP_CLOSURE, makeConstant(OBJ_VAL(function)));
//< Closures emit-closure
//> Closures capture-upvalues

  for (int i = 0; i < function->upvalueCount; i++) {
    emitByte(compiler.upvalues[i].isLocal ? 1 : 0);
    emitByte(compiler.upvalues[i].index);
  }
//< Closures capture-upvalues
}
//< Calls and Functions compile-function
//> Methods and Initializers method
static void method() {
  consume(TOKEN_IDENTIFIER, "Expect method name.");
  uint8_t constant = identifierConstant(&parser.previous);
//> method-body

//< method-body
/* Methods and Initializers method-body < Methods and Initializers method-type
  FunctionType type = TYPE_FUNCTION;
*/
//> method-type
  FunctionType type = TYPE_METHOD;
//< method-type
//> initializer-name
  if (parser.previous.length == 4 &&
      memcmp(parser.previous.start, "init", 4) == 0) {
    type = TYPE_INITIALIZER;
  }

//< initializer-name
//> method-body
  function(type);
//< method-body
  emitBytes(OP_METHOD, constant);
}
//< Methods and Initializers method
//> Classes and Instances class-declaration
static void classDeclaration() {
  consume(TOKEN_IDENTIFIER, "Expect class name.");
//> Methods and Initializers class-name
  Token className = parser.previous;
//< Methods and Initializers class-name
  uint8_t nameConstant = identifierConstant(&parser.previous);
  declareVariable();

  emitBytes(OP_CLASS, nameConstant);
  defineVariable(nameConstant);

//> Methods and Initializers create-class-compiler
  ClassCompiler classCompiler;
//> Superclasses init-has-superclass
  classCompiler.hasSuperclass = false;
//< Superclasses init-has-superclass
  classCompiler.enclosing = currentClass;
  currentClass = &classCompiler;

//< Methods and Initializers create-class-compiler
//> Superclasses compile-superclass
  if (match_compiler(TOKEN_LESS)) {
    consume(TOKEN_IDENTIFIER, "Expect superclass name.");
    variable(false);
//> inherit-self

    if (identifiersEqual(&className, &parser.previous)) {
      error("A class can't inherit from itself.");
    }

//< inherit-self
//> superclass-variable
    beginScope();
    addLocal(syntheticToken("super"));
    defineVariable(0);

//< superclass-variable
    namedVariable(className, false);
    emitByte(OP_INHERIT);
//> set-has-superclass
    classCompiler.hasSuperclass = true;
//< set-has-superclass
  }

//< Superclasses compile-superclass
//> Methods and Initializers load-class
  namedVariable(className, false);
//< Methods and Initializers load-class
  consume(TOKEN_LEFT_BRACE, "Expect '{' before class body.");
//> Methods and Initializers class-body
  while (!check(TOKEN_RIGHT_BRACE) && !check(TOKEN_EOF)) {
    method();
  }
//< Methods and Initializers class-body
  consume(TOKEN_RIGHT_BRACE, "Expect '}' after class body.");
//> Methods and Initializers pop-class
  emitByte(OP_POP);
//< Methods and Initializers pop-class
//> Superclasses end-superclass-scope

  if (classCompiler.hasSuperclass) {
    endScope();
  }
//< Superclasses end-superclass-scope
//> Methods and Initializers pop-enclosing

  currentClass = currentClass->enclosing;
//< Methods and Initializers pop-enclosing
}
//< Classes and Instances class-declaration
//> Calls and Functions fun-declaration
static void funDeclaration() {
  uint8_t global = parseVariable("Expect function name.");
  markInitialized();
  function(TYPE_FUNCTION);
  defineVariable(global);
}
//< Calls and Functions fun-declaration
//> Global Variables var-declaration
static void varDeclaration() {
  uint8_t global = parseVariable("Expect variable name.");

  if (match_compiler(TOKEN_EQUAL)) {
    expression();
  } else {
    emitByte(OP_NIL);
  }
  consume(TOKEN_SEMICOLON,
          "Expect ';' after variable declaration.");

  defineVariable(global);
}
//< Global Variables var-declaration
//> Global Variables expression-statement
static void expressionStatement() {
  expression();
  consume(TOKEN_SEMICOLON, "Expect ';' after expression.");
  emitByte(OP_POP);
}
//< Global Variables expression-statement
//> Jumping Back and Forth for-statement
static void forStatement() {
//> for-begin-scope
  beginScope();
//< for-begin-scope
  consume(TOKEN_LEFT_PAREN, "Expect '(' after 'for'.");
/* Jumping Back and Forth for-statement < Jumping Back and Forth for-initializer
  consume(TOKEN_SEMICOLON, "Expect ';'.");
*/
//> for-initializer
  if (match_compiler(TOKEN_SEMICOLON)) {
    // No initializer.
  } else if (match_compiler(TOKEN_VAR)) {
    varDeclaration();
  } else {
    expressionStatement();
  }
//< for-initializer

  int loopStart = currentChunk()->count;
/* Jumping Back and Forth for-statement < Jumping Back and Forth for-exit
  consume(TOKEN_SEMICOLON, "Expect ';'.");
*/
//> for-exit
  int exitJump = -1;
  if (!match_compiler(TOKEN_SEMICOLON)) {
    expression();
    consume(TOKEN_SEMICOLON, "Expect ';' after loop condition.");

    // Jump out of the loop if the condition is false.
    exitJump = emitJump(OP_JUMP_IF_FALSE);
    emitByte(OP_POP); // Condition.
  }

//< for-exit
/* Jumping Back and Forth for-statement < Jumping Back and Forth for-increment
  consume(TOKEN_RIGHT_PAREN, "Expect ')' after for clauses.");
*/
//> for-increment
  if (!match_compiler(TOKEN_RIGHT_PAREN)) {
    int bodyJump = emitJump(OP_JUMP);
    int incrementStart = currentChunk()->count;
    expression();
    emitByte(OP_POP);
    consume(TOKEN_RIGHT_PAREN, "Expect ')' after for clauses.");

    emitLoop(loopStart);
    loopStart = incrementStart;
    patchJump(bodyJump);
  }
//< for-increment

  statement();
  emitLoop(loopStart);
//> exit-jump

  if (exitJump != -1) {
    patchJump(exitJump);
    emitByte(OP_POP); // Condition.
  }

//< exit-jump
//> for-end-scope
  endScope();
//< for-end-scope
}
//< Jumping Back and Forth for-statement
//> Jumping Back and Forth if-statement
static void ifStatement() {
  consume(TOKEN_LEFT_PAREN, "Expect '(' after 'if'.");
  expression();
  consume(TOKEN_RIGHT_PAREN, "Expect ')' after condition."); // [paren]

  int thenJump = emitJump(OP_JUMP_IF_FALSE);
//> pop-then
  emitByte(OP_POP);
//< pop-then
  statement();

//> jump-over-else
  int elseJump = emitJump(OP_JUMP);

//< jump-over-else
  patchJump(thenJump);
//> pop-end
  emitByte(OP_POP);
//< pop-end
//> compile-else

  if (match_compiler(TOKEN_ELSE)) statement();
//< compile-else
//> patch-else
  patchJump(elseJump);
//< patch-else
}
//< Jumping Back and Forth if-statement
//> Global Variables print-statement
static void printStatement() {
  expression();
  consume(TOKEN_SEMICOLON, "Expect ';' after value.");
  emitByte(OP_PRINT);
}
//< Global Variables print-statement
//> Calls and Functions return-statement
static void returnStatement() {
//> return-from-script
  if (current->type == TYPE_SCRIPT) {
    error("Can't return from top-level code.");
  }

//< return-from-script
  if (match_compiler(TOKEN_SEMICOLON)) {
    emitReturn();
  } else {
//> Methods and Initializers return-from-init
    if (current->type == TYPE_INITIALIZER) {
      error("Can't return a value from an initializer.");
    }

//< Methods and Initializers return-from-init
    expression();
    consume(TOKEN_SEMICOLON, "Expect ';' after return value.");
    emitByte(OP_RETURN);
  }
}
//< Calls and Functions return-statement
//> Jumping Back and Forth while-statement
static void whileStatement() {
//> loop-start
  int loopStart = currentChunk()->count;
//< loop-start
  consume(TOKEN_LEFT_PAREN, "Expect '(' after 'while'.");
  expression();
  consume(TOKEN_RIGHT_PAREN, "Expect ')' after condition.");

  int exitJump = emitJump(OP_JUMP_IF_FALSE);
  emitByte(OP_POP);
  statement();
//> loop
  emitLoop(loopStart);
//< loop

  patchJump(exitJump);
  emitByte(OP_POP);
}
//< Jumping Back and Forth while-statement
//> Global Variables synchronize
static void synchronize() {
  parser.panicMode = false;

  while (parser.current.type != TOKEN_EOF) {
    if (parser.previous.type == TOKEN_SEMICOLON) return;
    switch (parser.current.type) {
      case TOKEN_CLASS:
      case TOKEN_FUN:
      case TOKEN_VAR:
      case TOKEN_FOR:
      case TOKEN_IF:
      case TOKEN_WHILE:
      case TOKEN_PRINT:
      case TOKEN_RETURN:
        return;

      default:
        ; // Do nothing.
    }

    advance_compiler();
  }
}
//< Global Variables synchronize
//> Global Variables declaration
static void declaration() {
//> Classes and Instances match-class
  if (match_compiler(TOKEN_CLASS)) {
    classDeclaration();
/* Calls and Functions match-fun < Classes and Instances match-class
  if (match(TOKEN_FUN)) {
*/
  } else if (match_compiler(TOKEN_FUN)) {
//< Classes and Instances match-class
//> Calls and Functions match-fun
    funDeclaration();
/* Global Variables match-var < Calls and Functions match-fun
  if (match(TOKEN_VAR)) {
*/
  } else if (match_compiler(TOKEN_VAR)) {
//< Calls and Functions match-fun
//> match-var
    varDeclaration();
  } else {
    statement();
  }
//< match-var
/* Global Variables declaration < Global Variables match-var
  statement();
*/
//> call-synchronize

  if (parser.panicMode) synchronize();
//< call-synchronize
}
//< Global Variables declaration
//> Global Variables statement
static void statement() {
  if (match_compiler(TOKEN_PRINT)) {
    printStatement();
//> Jumping Back and Forth parse-for
  } else if (match_compiler(TOKEN_FOR)) {
    forStatement();
//< Jumping Back and Forth parse-for
//> Jumping Back and Forth parse-if
  } else if (match_compiler(TOKEN_IF)) {
    ifStatement();
//< Jumping Back and Forth parse-if
//> Calls and Functions match-return
  } else if (match_compiler(TOKEN_RETURN)) {
    returnStatement();
//< Calls and Functions match-return
//> Jumping Back and Forth parse-while
  } else if (match_compiler(TOKEN_WHILE)) {
    whileStatement();
//< Jumping Back and Forth parse-while
//> Local Variables parse-block
  } else if (match_compiler(TOKEN_LEFT_BRACE)) {
    beginScope();
    block();
    endScope();
//< Local Variables parse-block
//> parse-expressions-statement
  } else {
    expressionStatement();
//< parse-expressions-statement
  }
}
//< Global Variables statement

/* Scanning on Demand compiler-c < Compiling Expressions compile-signature
void compile(const char* source) {
*/
/* Compiling Expressions compile-signature < Calls and Functions compile-signature
bool compile(const char* source, Chunk* chunk) {
*/
//> Calls and Functions compile-signature
ObjFunction* compile(const char* source) {
//< Calls and Functions compile-signature
  initScanner(source);
/* Scanning on Demand dump-tokens < Compiling Expressions compile-chunk
  int line = -1;
  for (;;) {
    Token token = scanToken();
    if (token.line != line) {
      printf("%4d ", token.line);
      line = token.line;
    } else {
      printf("   | ");
    }
    printf("%2d '%.*s'\n", token.type, token.length, token.start); // [format]

    if (token.type == TOKEN_EOF) break;
  }
*/
//> Local Variables compiler
  Compiler compiler;
//< Local Variables compiler
/* Local Variables compiler < Calls and Functions call-init-compiler
  initCompiler(&compiler);
*/
//> Calls and Functions call-init-compiler
  initCompiler(&compiler, TYPE_SCRIPT);
//< Calls and Functions call-init-compiler
/* Compiling Expressions init-compile-chunk < Calls and Functions call-init-compiler
  compilingChunk = chunk;
*/
//> Compiling Expressions compile-chunk
//> init-parser-error

  parser.hadError = false;
  parser.panicMode = false;

//< init-parser-error
  advance_compiler();
//< Compiling Expressions compile-chunk
/* Compiling Expressions compile-chunk < Global Variables compile
  expression();
  consume(TOKEN_EOF, "Expect end of expression.");
*/
//> Global Variables compile

  while (!match_compiler(TOKEN_EOF)) {
    declaration();
  }

//< Global Variables compile
/* Compiling Expressions finish-compile < Calls and Functions call-end-compiler
  endCompiler();
*/
/* Compiling Expressions return-had-error < Calls and Functions call-end-compiler
  return !parser.hadError;
*/
//> Calls and Functions call-end-compiler
  ObjFunction* function = endCompiler();
  return parser.hadError ? NULL : function;
//< Calls and Functions call-end-compiler
}
//> Garbage Collection mark-compiler-roots
void markCompilerRoots() {
  Compiler* compiler = current;
  while (compiler != NULL) {
    markObject((Obj*)compiler->function);
    compiler = compiler->enclosing;
  }
}
//< Garbage Collection mark-compiler-roots



//> Chunks of Bytecode debug-c
#include <stdio.h>

//> Closures debug-include-object
//< Closures debug-include-object
//> debug-include-value
//< debug-include-value

void disassembleChunk(Chunk* chunk, const char* name) {
  printf("== %s ==\n", name);
  
  for (int offset = 0; offset < chunk->count;) {
    offset = disassembleInstruction(chunk, offset);
  }
}
//> constant-instruction
static int constantInstruction(const char* name, Chunk* chunk,
                               int offset) {
  uint8_t constant = chunk->code[offset + 1];
  printf("%-16s %4d '", name, constant);
  printValue(chunk->constants.values[constant]);
  printf("'\n");
//> return-after-operand
  return offset + 2;
//< return-after-operand
}
//< constant-instruction
//> Methods and Initializers invoke-instruction
static int invokeInstruction(const char* name, Chunk* chunk,
                                int offset) {
  uint8_t constant = chunk->code[offset + 1];
  uint8_t argCount = chunk->code[offset + 2];
  printf("%-16s (%d args) %4d '", name, argCount, constant);
  printValue(chunk->constants.values[constant]);
  printf("'\n");
  return offset + 3;
}
//< Methods and Initializers invoke-instruction
//> simple-instruction
static int simpleInstruction(const char* name, int offset) {
  printf("%s\n", name);
  return offset + 1;
}
//< simple-instruction
//> Local Variables byte-instruction
static int byteInstruction(const char* name, Chunk* chunk,
                           int offset) {
  uint8_t slot = chunk->code[offset + 1];
  printf("%-16s %4d\n", name, slot);
  return offset + 2; // [debug]
}
//< Local Variables byte-instruction
//> Jumping Back and Forth jump-instruction
static int jumpInstruction(const char* name, int sign,
                           Chunk* chunk, int offset) {
  uint16_t jump = (uint16_t)(chunk->code[offset + 1] << 8);
  jump |= chunk->code[offset + 2];
  printf("%-16s %4d -> %d\n", name, offset,
         offset + 3 + sign * jump);
  return offset + 3;
}
//< Jumping Back and Forth jump-instruction
//> disassemble-instruction
int disassembleInstruction(Chunk* chunk, int offset) {
  printf("%04d ", offset);
//> show-location
  if (offset > 0 &&
      chunk->lines[offset] == chunk->lines[offset - 1]) {
    printf("   | ");
  } else {
    printf("%4d ", chunk->lines[offset]);
  }
//< show-location
  
  uint8_t instruction = chunk->code[offset];
  switch (instruction) {
//> disassemble-constant
    case OP_CONSTANT:
      return constantInstruction("OP_CONSTANT", chunk, offset);
//< disassemble-constant
//> Types of Values disassemble-literals
    case OP_NIL:
      return simpleInstruction("OP_NIL", offset);
    case OP_TRUE:
      return simpleInstruction("OP_TRUE", offset);
    case OP_FALSE:
      return simpleInstruction("OP_FALSE", offset);
//< Types of Values disassemble-literals
//> Global Variables disassemble-pop
    case OP_POP:
      return simpleInstruction("OP_POP", offset);
//< Global Variables disassemble-pop
//> Local Variables disassemble-local
    case OP_GET_LOCAL:
      return byteInstruction("OP_GET_LOCAL", chunk, offset);
    case OP_SET_LOCAL:
      return byteInstruction("OP_SET_LOCAL", chunk, offset);
//< Local Variables disassemble-local
//> Global Variables disassemble-get-global
    case OP_GET_GLOBAL:
      return constantInstruction("OP_GET_GLOBAL", chunk, offset);
//< Global Variables disassemble-get-global
//> Global Variables disassemble-define-global
    case OP_DEFINE_GLOBAL:
      return constantInstruction("OP_DEFINE_GLOBAL", chunk,
                                 offset);
//< Global Variables disassemble-define-global
//> Global Variables disassemble-set-global
    case OP_SET_GLOBAL:
      return constantInstruction("OP_SET_GLOBAL", chunk, offset);
//< Global Variables disassemble-set-global
//> Closures disassemble-upvalue-ops
    case OP_GET_UPVALUE:
      return byteInstruction("OP_GET_UPVALUE", chunk, offset);
    case OP_SET_UPVALUE:
      return byteInstruction("OP_SET_UPVALUE", chunk, offset);
//< Closures disassemble-upvalue-ops
//> Classes and Instances disassemble-property-ops
    case OP_GET_PROPERTY:
      return constantInstruction("OP_GET_PROPERTY", chunk, offset);
    case OP_SET_PROPERTY:
      return constantInstruction("OP_SET_PROPERTY", chunk, offset);
//< Classes and Instances disassemble-property-ops
//> Superclasses disassemble-get-super
    case OP_GET_SUPER:
      return constantInstruction("OP_GET_SUPER", chunk, offset);
//< Superclasses disassemble-get-super
//> Types of Values disassemble-comparison
    case OP_EQUAL:
      return simpleInstruction("OP_EQUAL", offset);
    case OP_GREATER:
      return simpleInstruction("OP_GREATER", offset);
    case OP_LESS:
      return simpleInstruction("OP_LESS", offset);
//< Types of Values disassemble-comparison
//> A Virtual Machine disassemble-binary
    case OP_ADD:
      return simpleInstruction("OP_ADD", offset);
    case OP_SUBTRACT:
      return simpleInstruction("OP_SUBTRACT", offset);
    case OP_MULTIPLY:
      return simpleInstruction("OP_MULTIPLY", offset);
    case OP_DIVIDE:
      return simpleInstruction("OP_DIVIDE", offset);
//> Types of Values disassemble-not
    case OP_NOT:
      return simpleInstruction("OP_NOT", offset);
//< Types of Values disassemble-not
//< A Virtual Machine disassemble-binary
//> A Virtual Machine disassemble-negate
    case OP_NEGATE:
      return simpleInstruction("OP_NEGATE", offset);
//< A Virtual Machine disassemble-negate
//> Global Variables disassemble-print
    case OP_PRINT:
      return simpleInstruction("OP_PRINT", offset);
//< Global Variables disassemble-print
//> Jumping Back and Forth disassemble-jump
    case OP_JUMP:
      return jumpInstruction("OP_JUMP", 1, chunk, offset);
    case OP_JUMP_IF_FALSE:
      return jumpInstruction("OP_JUMP_IF_FALSE", 1, chunk, offset);
//< Jumping Back and Forth disassemble-jump
//> Jumping Back and Forth disassemble-loop
    case OP_LOOP:
      return jumpInstruction("OP_LOOP", -1, chunk, offset);
//< Jumping Back and Forth disassemble-loop
//> Calls and Functions disassemble-call
    case OP_CALL:
      return byteInstruction("OP_CALL", chunk, offset);
//< Calls and Functions disassemble-call
//> Methods and Initializers disassemble-invoke
    case OP_INVOKE:
      return invokeInstruction("OP_INVOKE", chunk, offset);
//< Methods and Initializers disassemble-invoke
//> Superclasses disassemble-super-invoke
    case OP_SUPER_INVOKE:
      return invokeInstruction("OP_SUPER_INVOKE", chunk, offset);
//< Superclasses disassemble-super-invoke
//> Closures disassemble-closure
    case OP_CLOSURE: {
      offset++;
      uint8_t constant = chunk->code[offset++];
      printf("%-16s %4d ", "OP_CLOSURE", constant);
      printValue(chunk->constants.values[constant]);
      printf("\n");
//> disassemble-upvalues

      ObjFunction* function = AS_FUNCTION(
          chunk->constants.values[constant]);
      for (int j = 0; j < function->upvalueCount; j++) {
        int isLocal = chunk->code[offset++];
        int index = chunk->code[offset++];
        printf("%04d      |                     %s %d\n",
               offset - 2, isLocal ? "local" : "upvalue", index);
      }
      
//< disassemble-upvalues
      return offset;
    }
//< Closures disassemble-closure
//> Closures disassemble-close-upvalue
    case OP_CLOSE_UPVALUE:
      return simpleInstruction("OP_CLOSE_UPVALUE", offset);
//< Closures disassemble-close-upvalue
    case OP_RETURN:
      return simpleInstruction("OP_RETURN", offset);
//> Classes and Instances disassemble-class
    case OP_CLASS:
      return constantInstruction("OP_CLASS", chunk, offset);
//< Classes and Instances disassemble-class
//> Superclasses disassemble-inherit
    case OP_INHERIT:
      return simpleInstruction("OP_INHERIT", offset);
//< Superclasses disassemble-inherit
//> Methods and Initializers disassemble-method
    case OP_METHOD:
      return constantInstruction("OP_METHOD", chunk, offset);
//< Methods and Initializers disassemble-method
    default:
      printf("Unknown opcode %d\n", instruction);
      return offset + 1;
  }
}
//< disassemble-instruction



//> Chunks of Bytecode memory-c
#include <stdlib.h>

//> Garbage Collection memory-include-compiler
//< Garbage Collection memory-include-compiler
//> Strings memory-include-vm
//< Strings memory-include-vm
//> Garbage Collection debug-log-includes

#ifdef DEBUG_LOG_GC
#include <stdio.h>
#endif
//< Garbage Collection debug-log-includes
//> Garbage Collection heap-grow-factor

#define GC_HEAP_GROW_FACTOR 2
//< Garbage Collection heap-grow-factor

void* reallocate(void* pointer, size_t oldSize, size_t newSize) {
//> Garbage Collection updated-bytes-allocated
  vm.bytesAllocated += newSize - oldSize;
//< Garbage Collection updated-bytes-allocated
//> Garbage Collection call-collect
  if (newSize > oldSize) {
#ifdef DEBUG_STRESS_GC
    collectGarbage();
#endif
//> collect-on-next

    if (vm.bytesAllocated > vm.nextGC) {
      collectGarbage();
    }
//< collect-on-next
  }

//< Garbage Collection call-collect
  if (newSize == 0) {
    free(pointer);
    return NULL;
  }

  void* result = realloc(pointer, newSize);
//> out-of-memory
  if (result == NULL) exit(1);
//< out-of-memory
  return result;
}
//> Garbage Collection mark-object
void markObject(Obj* object) {
  if (object == NULL) return;
//> check-is-marked
  if (object->isMarked) return;

//< check-is-marked
//> log-mark-object
#ifdef DEBUG_LOG_GC
  printf("%p mark ", (void*)object);
  printValue(OBJ_VAL(object));
  printf("\n");
#endif

//< log-mark-object
  object->isMarked = true;
//> add-to-gray-stack

  if (vm.grayCapacity < vm.grayCount + 1) {
    vm.grayCapacity = GROW_CAPACITY(vm.grayCapacity);
    vm.grayStack = (Obj**)realloc(vm.grayStack,
                                  sizeof(Obj*) * vm.grayCapacity);
//> exit-gray-stack

    if (vm.grayStack == NULL) exit(1);
//< exit-gray-stack
  }

  vm.grayStack[vm.grayCount++] = object;
//< add-to-gray-stack
}
//< Garbage Collection mark-object
//> Garbage Collection mark-value
void markValue(Value value) {
  if (IS_OBJ(value)) markObject(AS_OBJ(value));
}
//< Garbage Collection mark-value
//> Garbage Collection mark-array
static void markArray(ValueArray* array) {
  for (int i = 0; i < array->count; i++) {
    markValue(array->values[i]);
  }
}
//< Garbage Collection mark-array
//> Garbage Collection blacken-object
static void blackenObject(Obj* object) {
//> log-blacken-object
#ifdef DEBUG_LOG_GC
  printf("%p blacken ", (void*)object);
  printValue(OBJ_VAL(object));
  printf("\n");
#endif

//< log-blacken-object
  switch (object->type) {
//> Methods and Initializers blacken-bound-method
    case OBJ_BOUND_METHOD: {
      ObjBoundMethod* bound = (ObjBoundMethod*)object;
      markValue(bound->receiver);
      markObject((Obj*)bound->method);
      break;
    }
//< Methods and Initializers blacken-bound-method
//> Classes and Instances blacken-class
    case OBJ_CLASS: {
      ObjClass* klass = (ObjClass*)object;
      markObject((Obj*)klass->name);
//> Methods and Initializers mark-methods
      markTable(&klass->methods);
//< Methods and Initializers mark-methods
      break;
    }
//< Classes and Instances blacken-class
//> blacken-closure
    case OBJ_CLOSURE: {
      ObjClosure* closure = (ObjClosure*)object;
      markObject((Obj*)closure->function);
      for (int i = 0; i < closure->upvalueCount; i++) {
        markObject((Obj*)closure->upvalues[i]);
      }
      break;
    }
//< blacken-closure
//> blacken-function
    case OBJ_FUNCTION: {
      ObjFunction* function = (ObjFunction*)object;
      markObject((Obj*)function->name);
      markArray(&function->chunk.constants);
      break;
    }
//< blacken-function
//> Classes and Instances blacken-instance
    case OBJ_INSTANCE: {
      ObjInstance* instance = (ObjInstance*)object;
      markObject((Obj*)instance->klass);
      markTable(&instance->fields);
      break;
    }
//< Classes and Instances blacken-instance
//> blacken-upvalue
    case OBJ_UPVALUE:
      markValue(((ObjUpvalue*)object)->closed);
      break;
//< blacken-upvalue
    case OBJ_NATIVE:
    case OBJ_STRING:
      break;
  }
}
//< Garbage Collection blacken-object
//> Strings free-object
static void freeObject(Obj* object) {
//> Garbage Collection log-free-object
#ifdef DEBUG_LOG_GC
  printf("%p free type %d\n", (void*)object, object->type);
#endif

//< Garbage Collection log-free-object
  switch (object->type) {
//> Methods and Initializers free-bound-method
    case OBJ_BOUND_METHOD:
      FREE(ObjBoundMethod, object);
      break;
//< Methods and Initializers free-bound-method
//> Classes and Instances free-class
    case OBJ_CLASS: {
//> Methods and Initializers free-methods
      ObjClass* klass = (ObjClass*)object;
      freeTable(&klass->methods);
//< Methods and Initializers free-methods
      FREE(ObjClass, object);
      break;
    } // [braces]
//< Classes and Instances free-class
//> Closures free-closure
    case OBJ_CLOSURE: {
//> free-upvalues
      ObjClosure* closure = (ObjClosure*)object;
      FREE_ARRAY(ObjUpvalue*, closure->upvalues,
                 closure->upvalueCount);
//< free-upvalues
      FREE(ObjClosure, object);
      break;
    }
//< Closures free-closure
//> Calls and Functions free-function
    case OBJ_FUNCTION: {
      ObjFunction* function = (ObjFunction*)object;
      freeChunk(&function->chunk);
      FREE(ObjFunction, object);
      break;
    }
//< Calls and Functions free-function
//> Classes and Instances free-instance
    case OBJ_INSTANCE: {
      ObjInstance* instance = (ObjInstance*)object;
      freeTable(&instance->fields);
      FREE(ObjInstance, object);
      break;
    }
//< Classes and Instances free-instance
//> Calls and Functions free-native
    case OBJ_NATIVE:
      FREE(ObjNative, object);
      break;
//< Calls and Functions free-native
    case OBJ_STRING: {
      ObjString* string = (ObjString*)object;
      FREE_ARRAY(char, string->chars, string->length + 1);
      FREE(ObjString, object);
      break;
    }
//> Closures free-upvalue
    case OBJ_UPVALUE:
      FREE(ObjUpvalue, object);
      break;
//< Closures free-upvalue
  }
}
//< Strings free-object
//> Garbage Collection mark-roots
static void markRoots() {
  for (Value* slot = vm.stack; slot < vm.stackTop; slot++) {
    markValue(*slot);
  }
//> mark-closures

  for (int i = 0; i < vm.frameCount; i++) {
    markObject((Obj*)vm.frames[i].closure);
  }
//< mark-closures
//> mark-open-upvalues

  for (ObjUpvalue* upvalue = vm.openUpvalues;
       upvalue != NULL;
       upvalue = upvalue->next) {
    markObject((Obj*)upvalue);
  }
//< mark-open-upvalues
//> mark-globals

  markTable(&vm.globals);
//< mark-globals
//> call-mark-compiler-roots
  markCompilerRoots();
//< call-mark-compiler-roots
//> Methods and Initializers mark-init-string
  markObject((Obj*)vm.initString);
//< Methods and Initializers mark-init-string
}
//< Garbage Collection mark-roots
//> Garbage Collection trace-references
static void traceReferences() {
  while (vm.grayCount > 0) {
    Obj* object = vm.grayStack[--vm.grayCount];
    blackenObject(object);
  }
}
//< Garbage Collection trace-references
//> Garbage Collection sweep
static void sweep() {
  Obj* previous = NULL;
  Obj* object = vm.objects;
  while (object != NULL) {
    if (object->isMarked) {
//> unmark
      object->isMarked = false;
//< unmark
      previous = object;
      object = object->next;
    } else {
      Obj* unreached = object;
      object = object->next;
      if (previous != NULL) {
        previous->next = object;
      } else {
        vm.objects = object;
      }

      freeObject(unreached);
    }
  }
}
//< Garbage Collection sweep
//> Garbage Collection collect-garbage
void collectGarbage() {
//> log-before-collect
#ifdef DEBUG_LOG_GC
  printf("-- gc begin\n");
//> log-before-size
  size_t before = vm.bytesAllocated;
//< log-before-size
#endif
//< log-before-collect
//> call-mark-roots

  markRoots();
//< call-mark-roots
//> call-trace-references
  traceReferences();
//< call-trace-references
//> sweep-strings
  tableRemoveWhite(&vm.strings);
//< sweep-strings
//> call-sweep
  sweep();
//< call-sweep
//> update-next-gc

  vm.nextGC = vm.bytesAllocated * GC_HEAP_GROW_FACTOR;
//< update-next-gc
//> log-after-collect

#ifdef DEBUG_LOG_GC
  printf("-- gc end\n");
//> log-collected-amount
  printf("   collected %zu bytes (from %zu to %zu) next at %zu\n",
         before - vm.bytesAllocated, before, vm.bytesAllocated,
         vm.nextGC);
//< log-collected-amount
#endif
//< log-after-collect
}
//< Garbage Collection collect-garbage
//> Strings free-objects
void freeObjects() {
  Obj* object = vm.objects;
  while (object != NULL) {
    Obj* next = object->next;
    freeObject(object);
    object = next;
  }
//> Garbage Collection free-gray-stack

  free(vm.grayStack);
//< Garbage Collection free-gray-stack
}
//< Strings free-objects



//> Strings object-c
#include <stdio.h>
#include <string.h>

//> Hash Tables object-include-table
//< Hash Tables object-include-table
//> allocate-obj

#define ALLOCATE_OBJ(type, objectType) \
    (type*)allocateObject(sizeof(type), objectType)
//< allocate-obj
//> allocate-object

static Obj* allocateObject(size_t size, ObjType type) {
  Obj* object = (Obj*)reallocate(NULL, 0, size);
  object->type = type;
//> Garbage Collection init-is-marked
  object->isMarked = false;
//< Garbage Collection init-is-marked
//> add-to-list
  
  object->next = vm.objects;
  vm.objects = object;
//< add-to-list
//> Garbage Collection debug-log-allocate

#ifdef DEBUG_LOG_GC
  printf("%p allocate %zu for %d\n", (void*)object, size, type);
#endif

//< Garbage Collection debug-log-allocate
  return object;
}
//< allocate-object
//> Methods and Initializers new-bound-method
ObjBoundMethod* newBoundMethod(Value receiver,
                               ObjClosure* method) {
  ObjBoundMethod* bound = ALLOCATE_OBJ(ObjBoundMethod,
                                       OBJ_BOUND_METHOD);
  bound->receiver = receiver;
  bound->method = method;
  return bound;
}
//< Methods and Initializers new-bound-method
//> Classes and Instances new-class
ObjClass* newClass(ObjString* name) {
  ObjClass* klass = ALLOCATE_OBJ(ObjClass, OBJ_CLASS);
  klass->name = name; // [klass]
//> Methods and Initializers init-methods
  initTable(&klass->methods);
//< Methods and Initializers init-methods
  return klass;
}
//< Classes and Instances new-class
//> Closures new-closure
ObjClosure* newClosure(ObjFunction* function) {
//> allocate-upvalue-array
  ObjUpvalue** upvalues = ALLOCATE(ObjUpvalue*,
                                   function->upvalueCount);
  for (int i = 0; i < function->upvalueCount; i++) {
    upvalues[i] = NULL;
  }

//< allocate-upvalue-array
  ObjClosure* closure = ALLOCATE_OBJ(ObjClosure, OBJ_CLOSURE);
  closure->function = function;
//> init-upvalue-fields
  closure->upvalues = upvalues;
  closure->upvalueCount = function->upvalueCount;
//< init-upvalue-fields
  return closure;
}
//< Closures new-closure
//> Calls and Functions new-function
ObjFunction* newFunction() {
  ObjFunction* function = ALLOCATE_OBJ(ObjFunction, OBJ_FUNCTION);
  function->arity = 0;
//> Closures init-upvalue-count
  function->upvalueCount = 0;
//< Closures init-upvalue-count
  function->name = NULL;
  initChunk(&function->chunk);
  return function;
}
//< Calls and Functions new-function
//> Classes and Instances new-instance
ObjInstance* newInstance(ObjClass* klass) {
  ObjInstance* instance = ALLOCATE_OBJ(ObjInstance, OBJ_INSTANCE);
  instance->klass = klass;
  initTable(&instance->fields);
  return instance;
}
//< Classes and Instances new-instance
//> Calls and Functions new-native
ObjNative* newNative(NativeFn function) {
  ObjNative* native = ALLOCATE_OBJ(ObjNative, OBJ_NATIVE);
  native->function = function;
  return native;
}
//< Calls and Functions new-native

/* Strings allocate-string < Hash Tables allocate-string
static ObjString* allocateString(char* chars, int length) {
*/
//> allocate-string
//> Hash Tables allocate-string
static ObjString* allocateString(char* chars, int length,
                                 uint32_t hash) {
//< Hash Tables allocate-string
  ObjString* string = ALLOCATE_OBJ(ObjString, OBJ_STRING);
  string->length = length;
  string->chars = chars;
//> Hash Tables allocate-store-hash
  string->hash = hash;
//< Hash Tables allocate-store-hash
//> Hash Tables allocate-store-string
//> Garbage Collection push-string

  push(OBJ_VAL(string));
//< Garbage Collection push-string
  tableSet(&vm.strings, string, NIL_VAL);
//> Garbage Collection pop-string
  pop();

//< Garbage Collection pop-string
//< Hash Tables allocate-store-string
  return string;
}
//< allocate-string
//> Hash Tables hash-string
static uint32_t hashString(const char* key, int length) {
  uint32_t hash = 2166136261u;
  for (int i = 0; i < length; i++) {
    hash ^= (uint8_t)key[i];
    hash *= 16777619;
  }
  return hash;
}
//< Hash Tables hash-string
//> take-string
ObjString* takeString(char* chars, int length) {
/* Strings take-string < Hash Tables take-string-hash
  return allocateString(chars, length);
*/
//> Hash Tables take-string-hash
  uint32_t hash = hashString(chars, length);
//> take-string-intern
  ObjString* interned = tableFindString(&vm.strings, chars, length,
                                        hash);
  if (interned != NULL) {
    FREE_ARRAY(char, chars, length + 1);
    return interned;
  }

//< take-string-intern
  return allocateString(chars, length, hash);
//< Hash Tables take-string-hash
}
//< take-string
ObjString* copyString(const char* chars, int length) {
//> Hash Tables copy-string-hash
  uint32_t hash = hashString(chars, length);
//> copy-string-intern
  ObjString* interned = tableFindString(&vm.strings, chars, length,
                                        hash);
  if (interned != NULL) return interned;

//< copy-string-intern
//< Hash Tables copy-string-hash
  char* heapChars = ALLOCATE(char, length + 1);
  memcpy(heapChars, chars, length);
  heapChars[length] = '\0';
/* Strings object-c < Hash Tables copy-string-allocate
  return allocateString(heapChars, length);
*/
//> Hash Tables copy-string-allocate
  return allocateString(heapChars, length, hash);
//< Hash Tables copy-string-allocate
}
//> Closures new-upvalue
ObjUpvalue* newUpvalue(Value* slot) {
  ObjUpvalue* upvalue = ALLOCATE_OBJ(ObjUpvalue, OBJ_UPVALUE);
//> init-closed
  upvalue->closed = NIL_VAL;
//< init-closed
  upvalue->location = slot;
//> init-next
  upvalue->next = NULL;
//< init-next
  return upvalue;
}
//< Closures new-upvalue
//> Calls and Functions print-function-helper
static void printFunction(ObjFunction* function) {
//> print-script
  if (function->name == NULL) {
    printf("<script>");
    return;
  }
//< print-script
  printf("<fn %s>", function->name->chars);
}
//< Calls and Functions print-function-helper
//> print-object
void printObject(Value value) {
  switch (OBJ_TYPE(value)) {
//> Methods and Initializers print-bound-method
    case OBJ_BOUND_METHOD:
      printFunction(AS_BOUND_METHOD(value)->method->function);
      break;
//< Methods and Initializers print-bound-method
//> Classes and Instances print-class
    case OBJ_CLASS:
      printf("%s", AS_CLASS(value)->name->chars);
      break;
//< Classes and Instances print-class
//> Closures print-closure
    case OBJ_CLOSURE:
      printFunction(AS_CLOSURE(value)->function);
      break;
//< Closures print-closure
//> Calls and Functions print-function
    case OBJ_FUNCTION:
      printFunction(AS_FUNCTION(value));
      break;
//< Calls and Functions print-function
//> Classes and Instances print-instance
    case OBJ_INSTANCE:
      printf("%s instance",
             AS_INSTANCE(value)->klass->name->chars);
      break;
//< Classes and Instances print-instance
//> Calls and Functions print-native
    case OBJ_NATIVE:
      printf("<native fn>");
      break;
//< Calls and Functions print-native
    case OBJ_STRING:
      printf("%s", AS_CSTRING(value));
      break;
//> Closures print-upvalue
    case OBJ_UPVALUE:
      printf("upvalue");
      break;
//< Closures print-upvalue
  }
}
//< print-object



//> Scanning on Demand scanner-c
#include <stdio.h>
#include <string.h>


typedef struct {
  const char* start;
  const char* current;
  int line;
} Scanner;

Scanner scanner;
//> init-scanner
void initScanner(const char* source) {
  scanner.start = source;
  scanner.current = source;
  scanner.line = 1;
}
//< init-scanner
//> is-alpha
static bool isAlpha(char c) {
  return (c >= 'a' && c <= 'z') ||
         (c >= 'A' && c <= 'Z') ||
          c == '_';
}
//< is-alpha
//> is-digit
static bool isDigit(char c) {
  return c >= '0' && c <= '9';
}
//< is-digit
//> is-at-end
static bool isAtEnd() {
  return *scanner.current == '\0';
}
//< is-at-end
//> advance
static char advance_scanner() {
  scanner.current++;
  return scanner.current[-1];
}
//< advance
//> peek
static char peek_scanner() {
  return *scanner.current;
}
//< peek
//> peek-next
static char peekNext() {
  if (isAtEnd()) return '\0';
  return scanner.current[1];
}
//< peek-next
//> match
static bool match_scanner(char expected) {
  if (isAtEnd()) return false;
  if (*scanner.current != expected) return false;
  scanner.current++;
  return true;
}
//< match
//> make-token
static Token makeToken(TokenType type) {
  Token token;
  token.type = type;
  token.start = scanner.start;
  token.length = (int)(scanner.current - scanner.start);
  token.line = scanner.line;
  return token;
}
//< make-token
//> error-token
static Token errorToken(const char* message) {
  Token token;
  token.type = TOKEN_ERROR;
  token.start = message;
  token.length = (int)strlen(message);
  token.line = scanner.line;
  return token;
}
//< error-token
//> skip-whitespace
static void skipWhitespace() {
  for (;;) {
    char c = peek_scanner();
    switch (c) {
      case ' ':
      case '\r':
      case '\t':
        advance_scanner();
        break;
//> newline
      case '\n':
        scanner.line++;
        advance_scanner();
        break;
//< newline
//> comment
      case '/':
        if (peekNext() == '/') {
          // A comment goes until the end of the line.
          while (peek_scanner() != '\n' && !isAtEnd()) advance_scanner();
        } else {
          return;
        }
        break;
//< comment
      default:
        return;
    }
  }
}
//< skip-whitespace
//> check-keyword
static TokenType checkKeyword(int start, int length,
    const char* rest, TokenType type) {
  if (scanner.current - scanner.start == start + length &&
      memcmp(scanner.start + start, rest, length) == 0) {
    return type;
  }

  return TOKEN_IDENTIFIER;
}
//< check-keyword
//> identifier-type
static TokenType identifierType() {
//> keywords
  switch (scanner.start[0]) {
    case 'a': return checkKeyword(1, 2, "nd", TOKEN_AND);
    case 'c': return checkKeyword(1, 4, "lass", TOKEN_CLASS);
    case 'e': return checkKeyword(1, 3, "lse", TOKEN_ELSE);
//> keyword-f
    case 'f':
      if (scanner.current - scanner.start > 1) {
        switch (scanner.start[1]) {
          case 'a': return checkKeyword(2, 3, "lse", TOKEN_FALSE);
          case 'o': return checkKeyword(2, 1, "r", TOKEN_FOR);
          case 'u': return checkKeyword(2, 1, "n", TOKEN_FUN);
        }
      }
      break;
//< keyword-f
    case 'i': return checkKeyword(1, 1, "f", TOKEN_IF);
    case 'n': return checkKeyword(1, 2, "il", TOKEN_NIL);
    case 'o': return checkKeyword(1, 1, "r", TOKEN_OR);
    case 'p': return checkKeyword(1, 4, "rint", TOKEN_PRINT);
    case 'r': return checkKeyword(1, 5, "eturn", TOKEN_RETURN);
    case 's': return checkKeyword(1, 4, "uper", TOKEN_SUPER);
//> keyword-t
    case 't':
      if (scanner.current - scanner.start > 1) {
        switch (scanner.start[1]) {
          case 'h': return checkKeyword(2, 2, "is", TOKEN_THIS);
          case 'r': return checkKeyword(2, 2, "ue", TOKEN_TRUE);
        }
      }
      break;
//< keyword-t
    case 'v': return checkKeyword(1, 2, "ar", TOKEN_VAR);
    case 'w': return checkKeyword(1, 4, "hile", TOKEN_WHILE);
  }

//< keywords
  return TOKEN_IDENTIFIER;
}
//< identifier-type
//> identifier
static Token identifier() {
  while (isAlpha(peek_scanner()) || isDigit(peek_scanner())) advance_scanner();
  return makeToken(identifierType());
}
//< identifier
//> number
static Token number_scanner() {
  while (isDigit(peek_scanner())) advance_scanner();

  // Look for a fractional part.
  if (peek_scanner() == '.' && isDigit(peekNext())) {
    // Consume the ".".
    advance_scanner();

    while (isDigit(peek_scanner())) advance_scanner();
  }

  return makeToken(TOKEN_NUMBER);
}
//< number
//> string
static Token string_scanner() {
  while (peek_scanner() != '"' && !isAtEnd()) {
    if (peek_scanner() == '\n') scanner.line++;
    advance_scanner();
  }

  if (isAtEnd()) return errorToken("Unterminated string.");

  // The closing quote.
  advance_scanner();
  return makeToken(TOKEN_STRING);
}
//< string
//> scan-token
Token scanToken() {
//> call-skip-whitespace
  skipWhitespace();
//< call-skip-whitespace
  scanner.start = scanner.current;

  if (isAtEnd()) return makeToken(TOKEN_EOF);
//> scan-char

  char c = advance_scanner();
//> scan-identifier
  if (isAlpha(c)) return identifier();
//< scan-identifier
//> scan-number
  if (isDigit(c)) return number_scanner();
//< scan-number

  switch (c) {
    case '(': return makeToken(TOKEN_LEFT_PAREN);
    case ')': return makeToken(TOKEN_RIGHT_PAREN);
    case '{': return makeToken(TOKEN_LEFT_BRACE);
    case '}': return makeToken(TOKEN_RIGHT_BRACE);
    case ';': return makeToken(TOKEN_SEMICOLON);
    case ',': return makeToken(TOKEN_COMMA);
    case '.': return makeToken(TOKEN_DOT);
    case '-': return makeToken(TOKEN_MINUS);
    case '+': return makeToken(TOKEN_PLUS);
    case '/': return makeToken(TOKEN_SLASH);
    case '*': return makeToken(TOKEN_STAR);
//> two-char
    case '!':
      return makeToken(
          match_scanner('=') ? TOKEN_BANG_EQUAL : TOKEN_BANG);
    case '=':
      return makeToken(
          match_scanner('=') ? TOKEN_EQUAL_EQUAL : TOKEN_EQUAL);
    case '<':
      return makeToken(
          match_scanner('=') ? TOKEN_LESS_EQUAL : TOKEN_LESS);
    case '>':
      return makeToken(
          match_scanner('=') ? TOKEN_GREATER_EQUAL : TOKEN_GREATER);
//< two-char
//> scan-string
    case '"': return string_scanner();
//< scan-string
  }
//< scan-char

  return errorToken("Unexpected character.");
}
//< scan-token



//> Hash Tables table-c
#include <stdlib.h>
#include <string.h>


//> max-load
#define TABLE_MAX_LOAD 0.75

//< max-load
void initTable(Table* table) {
  table->count = 0;
  table->capacity = 0;
  table->entries = NULL;
}
//> free-table
void freeTable(Table* table) {
  FREE_ARRAY(Entry, table->entries, table->capacity);
  initTable(table);
}
//< free-table
//> find-entry
//> omit
// NOTE: The "Optimization" chapter has a manual copy of this function.
// If you change it here, make sure to update that copy.
//< omit
static Entry* findEntry(Entry* entries, int capacity,
                        ObjString* key) {
/* Hash Tables find-entry < Optimization initial-index
  uint32_t index = key->hash % capacity;
*/
//> Optimization initial-index
  uint32_t index = key->hash & (capacity - 1);
//< Optimization initial-index
//> find-entry-tombstone
  Entry* tombstone = NULL;
  
//< find-entry-tombstone
  for (;;) {
    Entry* entry = &entries[index];
/* Hash Tables find-entry < Hash Tables find-tombstone
    if (entry->key == key || entry->key == NULL) {
      return entry;
    }
*/
//> find-tombstone
    if (entry->key == NULL) {
      if (IS_NIL(entry->value)) {
        // Empty entry.
        return tombstone != NULL ? tombstone : entry;
      } else {
        // We found a tombstone.
        if (tombstone == NULL) tombstone = entry;
      }
    } else if (entry->key == key) {
      // We found the key.
      return entry;
    }
//< find-tombstone

/* Hash Tables find-entry < Optimization next-index
    index = (index + 1) % capacity;
*/
//> Optimization next-index
    index = (index + 1) & (capacity - 1);
//< Optimization next-index
  }
}
//< find-entry
//> table-get
bool tableGet(Table* table, ObjString* key, Value* value) {
  if (table->count == 0) return false;

  Entry* entry = findEntry(table->entries, table->capacity, key);
  if (entry->key == NULL) return false;

  *value = entry->value;
  return true;
}
//< table-get
//> table-adjust-capacity
static void adjustCapacity(Table* table, int capacity) {
  Entry* entries = ALLOCATE(Entry, capacity);
  for (int i = 0; i < capacity; i++) {
    entries[i].key = NULL;
    entries[i].value = NIL_VAL;
  }
//> re-hash

//> resize-init-count
  table->count = 0;
//< resize-init-count
  for (int i = 0; i < table->capacity; i++) {
    Entry* entry = &table->entries[i];
    if (entry->key == NULL) continue;

    Entry* dest = findEntry(entries, capacity, entry->key);
    dest->key = entry->key;
    dest->value = entry->value;
//> resize-increment-count
    table->count++;
//< resize-increment-count
  }
//< re-hash

//> Hash Tables free-old-array
  FREE_ARRAY(Entry, table->entries, table->capacity);
//< Hash Tables free-old-array
  table->entries = entries;
  table->capacity = capacity;
}
//< table-adjust-capacity
//> table-set
bool tableSet(Table* table, ObjString* key, Value value) {
//> table-set-grow
  if (table->count + 1 > table->capacity * TABLE_MAX_LOAD) {
    int capacity = GROW_CAPACITY(table->capacity);
    adjustCapacity(table, capacity);
  }

//< table-set-grow
  Entry* entry = findEntry(table->entries, table->capacity, key);
  bool isNewKey = entry->key == NULL;
/* Hash Tables table-set < Hash Tables set-increment-count
  if (isNewKey) table->count++;
*/
//> set-increment-count
  if (isNewKey && IS_NIL(entry->value)) table->count++;
//< set-increment-count

  entry->key = key;
  entry->value = value;
  return isNewKey;
}
//< table-set
//> table-delete
bool tableDelete(Table* table, ObjString* key) {
  if (table->count == 0) return false;

  // Find the entry.
  Entry* entry = findEntry(table->entries, table->capacity, key);
  if (entry->key == NULL) return false;

  // Place a tombstone in the entry.
  entry->key = NULL;
  entry->value = BOOL_VAL(true);
  return true;
}
//< table-delete
//> table-add-all
void tableAddAll(Table* from, Table* to) {
  for (int i = 0; i < from->capacity; i++) {
    Entry* entry = &from->entries[i];
    if (entry->key != NULL) {
      tableSet(to, entry->key, entry->value);
    }
  }
}
//< table-add-all
//> table-find-string
ObjString* tableFindString(Table* table, const char* chars,
                           int length, uint32_t hash) {
  if (table->count == 0) return NULL;

/* Hash Tables table-find-string < Optimization find-string-index
  uint32_t index = hash % table->capacity;
*/
//> Optimization find-string-index
  uint32_t index = hash & (table->capacity - 1);
//< Optimization find-string-index
  for (;;) {
    Entry* entry = &table->entries[index];
    if (entry->key == NULL) {
      // Stop if we find an empty non-tombstone entry.
      if (IS_NIL(entry->value)) return NULL;
    } else if (entry->key->length == length &&
        entry->key->hash == hash &&
        memcmp(entry->key->chars, chars, length) == 0) {
      // We found it.
      return entry->key;
    }

/* Hash Tables table-find-string < Optimization find-string-next
    index = (index + 1) % table->capacity;
*/
//> Optimization find-string-next
    index = (index + 1) & (table->capacity - 1);
//< Optimization find-string-next
  }
}
//< table-find-string
//> Garbage Collection table-remove-white
void tableRemoveWhite(Table* table) {
  for (int i = 0; i < table->capacity; i++) {
    Entry* entry = &table->entries[i];
    if (entry->key != NULL && !entry->key->obj.isMarked) {
      tableDelete(table, entry->key);
    }
  }
}
//< Garbage Collection table-remove-white
//> Garbage Collection mark-table
void markTable(Table* table) {
  for (int i = 0; i < table->capacity; i++) {
    Entry* entry = &table->entries[i];
    markObject((Obj*)entry->key);
    markValue(entry->value);
  }
}
//< Garbage Collection mark-table



//> Chunks of Bytecode value-c
#include <stdio.h>
//> Strings value-include-string
#include <string.h>
//< Strings value-include-string

//> Strings value-include-object
//< Strings value-include-object

void initValueArray(ValueArray* array) {
  array->values = NULL;
  array->capacity = 0;
  array->count = 0;
}
//> write-value-array
void writeValueArray(ValueArray* array, Value value) {
  if (array->capacity < array->count + 1) {
    int oldCapacity = array->capacity;
    array->capacity = GROW_CAPACITY(oldCapacity);
    array->values = GROW_ARRAY(Value, array->values,
                               oldCapacity, array->capacity);
  }
  
  array->values[array->count] = value;
  array->count++;
}
//< write-value-array
//> free-value-array
void freeValueArray(ValueArray* array) {
  FREE_ARRAY(Value, array->values, array->capacity);
  initValueArray(array);
}
//< free-value-array
//> print-value
void printValue(Value value) {
//> Optimization print-value
#ifdef NAN_BOXING
  if (IS_BOOL(value)) {
    printf(AS_BOOL(value) ? "true" : "false");
  } else if (IS_NIL(value)) {
    printf("nil");
  } else if (IS_NUMBER(value)) {
    printf("%g", AS_NUMBER(value));
  } else if (IS_OBJ(value)) {
    printObject(value);
  }
#else
//< Optimization print-value
/* Chunks of Bytecode print-value < Types of Values print-number-value
  printf("%g", value);
*/
/* Types of Values print-number-value < Types of Values print-value
 printf("%g", AS_NUMBER(value));
 */
//> Types of Values print-value
  switch (value.type) {
    case VAL_BOOL:
      printf(AS_BOOL(value) ? "true" : "false");
      break;
    case VAL_NIL: printf("nil"); break;
    case VAL_NUMBER: printf("%g", AS_NUMBER(value)); break;
//> Strings call-print-object
    case VAL_OBJ: printObject(value); break;
//< Strings call-print-object
  }
//< Types of Values print-value
//> Optimization end-print-value
#endif
//< Optimization end-print-value
}
//< print-value
//> Types of Values values-equal
bool valuesEqual(Value a, Value b) {
//> Optimization values-equal
#ifdef NAN_BOXING
//> nan-equality
  if (IS_NUMBER(a) && IS_NUMBER(b)) {
    return AS_NUMBER(a) == AS_NUMBER(b);
  }
//< nan-equality
  return a == b;
#else
//< Optimization values-equal
  if (a.type != b.type) return false;
  switch (a.type) {
    case VAL_BOOL:   return AS_BOOL(a) == AS_BOOL(b);
    case VAL_NIL:    return true;
    case VAL_NUMBER: return AS_NUMBER(a) == AS_NUMBER(b);
/* Strings strings-equal < Hash Tables equal
    case VAL_OBJ: {
      ObjString* aString = AS_STRING(a);
      ObjString* bString = AS_STRING(b);
      return aString->length == bString->length &&
          memcmp(aString->chars, bString->chars,
                 aString->length) == 0;
    }
 */
//> Hash Tables equal
    case VAL_OBJ:    return AS_OBJ(a) == AS_OBJ(b);
//< Hash Tables equal
    default:         return false; // Unreachable.
  }
//> Optimization end-values-equal
#endif
//< Optimization end-values-equal
}
//< Types of Values values-equal



//> A Virtual Machine vm-c
//> Types of Values include-stdarg
#include <stdarg.h>
//< Types of Values include-stdarg
//> vm-include-stdio
#include <stdio.h>
//> Strings vm-include-string
#include <string.h>
//< Strings vm-include-string
//> Calls and Functions vm-include-time
#include <time.h>
//< Calls and Functions vm-include-time

//< vm-include-stdio
//> Scanning on Demand vm-include-compiler
//< Scanning on Demand vm-include-compiler
//> vm-include-debug
//< vm-include-debug
//> Strings vm-include-object-memory
//< Strings vm-include-object-memory

VM vm; // [one]
//> Calls and Functions clock-native
static Value clockNative(int argCount, Value* args) {
  return NUMBER_VAL((double)clock() / CLOCKS_PER_SEC);
}
//< Calls and Functions clock-native
//> reset-stack
static void resetStack() {
  vm.stackTop = vm.stack;
//> Calls and Functions reset-frame-count
  vm.frameCount = 0;
//< Calls and Functions reset-frame-count
//> Closures init-open-upvalues
  vm.openUpvalues = NULL;
//< Closures init-open-upvalues
}
//< reset-stack
//> Types of Values runtime-error
static void runtimeError(const char* format, ...) {
  va_list args;
  va_start(args, format);
  vfprintf(stderr, format, args);
  va_end(args);
  fputs("\n", stderr);

/* Types of Values runtime-error < Calls and Functions runtime-error-temp
  size_t instruction = vm.ip - vm.chunk->code - 1;
  int line = vm.chunk->lines[instruction];
*/
/* Calls and Functions runtime-error-temp < Calls and Functions runtime-error-stack
  CallFrame* frame = &vm.frames[vm.frameCount - 1];
  size_t instruction = frame->ip - frame->function->chunk.code - 1;
  int line = frame->function->chunk.lines[instruction];
*/
/* Types of Values runtime-error < Calls and Functions runtime-error-stack
  fprintf(stderr, "[line %d] in script\n", line);
*/
//> Calls and Functions runtime-error-stack
  for (int i = vm.frameCount - 1; i >= 0; i--) {
    CallFrame* frame = &vm.frames[i];
/* Calls and Functions runtime-error-stack < Closures runtime-error-function
    ObjFunction* function = frame->function;
*/
//> Closures runtime-error-function
    ObjFunction* function = frame->closure->function;
//< Closures runtime-error-function
    size_t instruction = frame->ip - function->chunk.code - 1;
    fprintf(stderr, "[line %d] in ", // [minus]
            function->chunk.lines[instruction]);
    if (function->name == NULL) {
      fprintf(stderr, "script\n");
    } else {
      fprintf(stderr, "%s()\n", function->name->chars);
    }
  }

//< Calls and Functions runtime-error-stack
  resetStack();
}
//< Types of Values runtime-error
//> Calls and Functions define-native
static void defineNative(const char* name, NativeFn function) {
  push(OBJ_VAL(copyString(name, (int)strlen(name))));
  push(OBJ_VAL(newNative(function)));
  tableSet(&vm.globals, AS_STRING(vm.stack[0]), vm.stack[1]);
  pop();
  pop();
}
//< Calls and Functions define-native

void initVM() {
//> call-reset-stack
  resetStack();
//< call-reset-stack
//> Strings init-objects-root
  vm.objects = NULL;
//< Strings init-objects-root
//> Garbage Collection init-gc-fields
  vm.bytesAllocated = 0;
  vm.nextGC = 1024 * 1024;
//< Garbage Collection init-gc-fields
//> Garbage Collection init-gray-stack

  vm.grayCount = 0;
  vm.grayCapacity = 0;
  vm.grayStack = NULL;
//< Garbage Collection init-gray-stack
//> Global Variables init-globals

  initTable(&vm.globals);
//< Global Variables init-globals
//> Hash Tables init-strings
  initTable(&vm.strings);
//< Hash Tables init-strings
//> Methods and Initializers init-init-string

//> null-init-string
  vm.initString = NULL;
//< null-init-string
  vm.initString = copyString("init", 4);
//< Methods and Initializers init-init-string
//> Calls and Functions define-native-clock

  defineNative("clock", clockNative);
//< Calls and Functions define-native-clock
}

void freeVM() {
//> Global Variables free-globals
  freeTable(&vm.globals);
//< Global Variables free-globals
//> Hash Tables free-strings
  freeTable(&vm.strings);
//< Hash Tables free-strings
//> Methods and Initializers clear-init-string
  vm.initString = NULL;
//< Methods and Initializers clear-init-string
//> Strings call-free-objects
  freeObjects();
//< Strings call-free-objects
}
//> push
void push(Value value) {
  *vm.stackTop = value;
  vm.stackTop++;
}
//< push
//> pop
Value pop() {
  vm.stackTop--;
  return *vm.stackTop;
}
//< pop
//> Types of Values peek
static Value peek_vm(int distance) {
  return vm.stackTop[-1 - distance];
}
//< Types of Values peek
/* Calls and Functions call < Closures call-signature
static bool call(ObjFunction* function, int argCount) {
*/
//> Calls and Functions call
//> Closures call-signature
static bool call_vm(ObjClosure* closure, int argCount) {
//< Closures call-signature
/* Calls and Functions check-arity < Closures check-arity
  if (argCount != function->arity) {
    runtimeError("Expected %d arguments but got %d.",
        function->arity, argCount);
*/
//> Closures check-arity
  if (argCount != closure->function->arity) {
    runtimeError("Expected %d arguments but got %d.",
        closure->function->arity, argCount);
//< Closures check-arity
//> check-arity
    return false;
  }

//< check-arity
//> check-overflow
  if (vm.frameCount == FRAMES_MAX) {
    runtimeError("Stack overflow.");
    return false;
  }

//< check-overflow
  CallFrame* frame = &vm.frames[vm.frameCount++];
/* Calls and Functions call < Closures call-init-closure
  frame->function = function;
  frame->ip = function->chunk.code;
*/
//> Closures call-init-closure
  frame->closure = closure;
  frame->ip = closure->function->chunk.code;
//< Closures call-init-closure
  frame->slots = vm.stackTop - argCount - 1;
  return true;
}
//< Calls and Functions call
//> Calls and Functions call-value
static bool callValue(Value callee, int argCount) {
  if (IS_OBJ(callee)) {
    switch (OBJ_TYPE(callee)) {
//> Methods and Initializers call-bound-method
      case OBJ_BOUND_METHOD: {
        ObjBoundMethod* bound = AS_BOUND_METHOD(callee);
//> store-receiver
        vm.stackTop[-argCount - 1] = bound->receiver;
//< store-receiver
        return call_vm(bound->method, argCount);
      }
//< Methods and Initializers call-bound-method
//> Classes and Instances call-class
      case OBJ_CLASS: {
        ObjClass* klass = AS_CLASS(callee);
        vm.stackTop[-argCount - 1] = OBJ_VAL(newInstance(klass));
//> Methods and Initializers call-init
        Value initializer;
        if (tableGet(&klass->methods, vm.initString,
                     &initializer)) {
          return call_vm(AS_CLOSURE(initializer), argCount);
//> no-init-arity-error
        } else if (argCount != 0) {
          runtimeError("Expected 0 arguments but got %d.",
                       argCount);
          return false;
//< no-init-arity-error
        }
//< Methods and Initializers call-init
        return true;
      }
//< Classes and Instances call-class
//> Closures call-value-closure
      case OBJ_CLOSURE:
        return call_vm(AS_CLOSURE(callee), argCount);
//< Closures call-value-closure
/* Calls and Functions call-value < Closures call-value-closure
      case OBJ_FUNCTION: // [switch]
        return call(AS_FUNCTION(callee), argCount);
*/
//> call-native
      case OBJ_NATIVE: {
        NativeFn native = AS_NATIVE(callee);
        Value result = native(argCount, vm.stackTop - argCount);
        vm.stackTop -= argCount + 1;
        push(result);
        return true;
      }
//< call-native
      default:
        break; // Non-callable object type.
    }
  }
  runtimeError("Can only call functions and classes.");
  return false;
}
//< Calls and Functions call-value
//> Methods and Initializers invoke-from-class
static bool invokeFromClass(ObjClass* klass, ObjString* name,
                            int argCount) {
  Value method;
  if (!tableGet(&klass->methods, name, &method)) {
    runtimeError("Undefined property '%s'.", name->chars);
    return false;
  }
  return call_vm(AS_CLOSURE(method), argCount);
}
//< Methods and Initializers invoke-from-class
//> Methods and Initializers invoke
static bool invoke(ObjString* name, int argCount) {
  Value receiver = peek_vm(argCount);
//> invoke-check-type

  if (!IS_INSTANCE(receiver)) {
    runtimeError("Only instances have methods.");
    return false;
  }

//< invoke-check-type
  ObjInstance* instance = AS_INSTANCE(receiver);
//> invoke-field

  Value value;
  if (tableGet(&instance->fields, name, &value)) {
    vm.stackTop[-argCount - 1] = value;
    return callValue(value, argCount);
  }

//< invoke-field
  return invokeFromClass(instance->klass, name, argCount);
}
//< Methods and Initializers invoke
//> Methods and Initializers bind-method
static bool bindMethod(ObjClass* klass, ObjString* name) {
  Value method;
  if (!tableGet(&klass->methods, name, &method)) {
    runtimeError("Undefined property '%s'.", name->chars);
    return false;
  }

  ObjBoundMethod* bound = newBoundMethod(peek_vm(0),
                                         AS_CLOSURE(method));
  pop();
  push(OBJ_VAL(bound));
  return true;
}
//< Methods and Initializers bind-method
//> Closures capture-upvalue
static ObjUpvalue* captureUpvalue(Value* local) {
//> look-for-existing-upvalue
  ObjUpvalue* prevUpvalue = NULL;
  ObjUpvalue* upvalue = vm.openUpvalues;
  while (upvalue != NULL && upvalue->location > local) {
    prevUpvalue = upvalue;
    upvalue = upvalue->next;
  }

  if (upvalue != NULL && upvalue->location == local) {
    return upvalue;
  }

//< look-for-existing-upvalue
  ObjUpvalue* createdUpvalue = newUpvalue(local);
//> insert-upvalue-in-list
  createdUpvalue->next = upvalue;

  if (prevUpvalue == NULL) {
    vm.openUpvalues = createdUpvalue;
  } else {
    prevUpvalue->next = createdUpvalue;
  }

//< insert-upvalue-in-list
  return createdUpvalue;
}
//< Closures capture-upvalue
//> Closures close-upvalues
static void closeUpvalues(Value* last) {
  while (vm.openUpvalues != NULL &&
         vm.openUpvalues->location >= last) {
    ObjUpvalue* upvalue = vm.openUpvalues;
    upvalue->closed = *upvalue->location;
    upvalue->location = &upvalue->closed;
    vm.openUpvalues = upvalue->next;
  }
}
//< Closures close-upvalues
//> Methods and Initializers define-method
static void defineMethod(ObjString* name) {
  Value method = peek_vm(0);
  ObjClass* klass = AS_CLASS(peek_vm(1));
  tableSet(&klass->methods, name, method);
  pop();
}
//< Methods and Initializers define-method
//> Types of Values is-falsey
static bool isFalsey(Value value) {
  return IS_NIL(value) || (IS_BOOL(value) && !AS_BOOL(value));
}
//< Types of Values is-falsey
//> Strings concatenate
static void concatenate() {
/* Strings concatenate < Garbage Collection concatenate-peek
  ObjString* b = AS_STRING(pop());
  ObjString* a = AS_STRING(pop());
*/
//> Garbage Collection concatenate-peek
  ObjString* b = AS_STRING(peek_vm(0));
  ObjString* a = AS_STRING(peek_vm(1));
//< Garbage Collection concatenate-peek

  int length = a->length + b->length;
  char* chars = ALLOCATE(char, length + 1);
  memcpy(chars, a->chars, a->length);
  memcpy(chars + a->length, b->chars, b->length);
  chars[length] = '\0';

  ObjString* result = takeString(chars, length);
//> Garbage Collection concatenate-pop
  pop();
  pop();
//< Garbage Collection concatenate-pop
  push(OBJ_VAL(result));
}
//< Strings concatenate
//> run
static InterpretResult run() {
//> Calls and Functions run
  CallFrame* frame = &vm.frames[vm.frameCount - 1];

/* A Virtual Machine run < Calls and Functions run
#define READ_BYTE() (*vm.ip++)
*/
#define READ_BYTE() (*frame->ip++)
/* A Virtual Machine read-constant < Calls and Functions run
#define READ_CONSTANT() (vm.chunk->constants.values[READ_BYTE()])
*/

/* Jumping Back and Forth read-short < Calls and Functions run
#define READ_SHORT() \
    (vm.ip += 2, (uint16_t)((vm.ip[-2] << 8) | vm.ip[-1]))
*/
#define READ_SHORT() \
    (frame->ip += 2, \
    (uint16_t)((frame->ip[-2] << 8) | frame->ip[-1]))

/* Calls and Functions run < Closures read-constant
#define READ_CONSTANT() \
    (frame->function->chunk.constants.values[READ_BYTE()])
*/
//> Closures read-constant
#define READ_CONSTANT() \
    (frame->closure->function->chunk.constants.values[READ_BYTE()])
//< Closures read-constant

//< Calls and Functions run
//> Global Variables read-string
#define READ_STRING() AS_STRING(READ_CONSTANT())
//< Global Variables read-string
/* A Virtual Machine binary-op < Types of Values binary-op
#define BINARY_OP(op) \
    do { \
      double b = pop(); \
      double a = pop(); \
      push(a op b); \
    } while (false)
*/
//> Types of Values binary-op
#define BINARY_OP(valueType, op) \
    do { \
      if (!IS_NUMBER(peek_vm(0)) || !IS_NUMBER(peek_vm(1))) { \
        runtimeError("Operands must be numbers."); \
        return INTERPRET_RUNTIME_ERROR; \
      } \
      double b = AS_NUMBER(pop()); \
      double a = AS_NUMBER(pop()); \
      push(valueType(a op b)); \
    } while (false)
//< Types of Values binary-op

  for (;;) {
//> trace-execution
#ifdef DEBUG_TRACE_EXECUTION
//> trace-stack
    printf("          ");
    for (Value* slot = vm.stack; slot < vm.stackTop; slot++) {
      printf("[ ");
      printValue(*slot);
      printf(" ]");
    }
    printf("\n");
//< trace-stack
/* A Virtual Machine trace-execution < Calls and Functions trace-execution
    disassembleInstruction(vm.chunk,
                           (int)(vm.ip - vm.chunk->code));
*/
/* Calls and Functions trace-execution < Closures disassemble-instruction
    disassembleInstruction(&frame->function->chunk,
        (int)(frame->ip - frame->function->chunk.code));
*/
//> Closures disassemble-instruction
    disassembleInstruction(&frame->closure->function->chunk,
        (int)(frame->ip - frame->closure->function->chunk.code));
//< Closures disassemble-instruction
#endif

//< trace-execution
    uint8_t instruction;
    switch (instruction = READ_BYTE()) {
//> op-constant
      case OP_CONSTANT: {
        Value constant = READ_CONSTANT();
/* A Virtual Machine op-constant < A Virtual Machine push-constant
        printValue(constant);
        printf("\n");
*/
//> push-constant
        push(constant);
//< push-constant
        break;
      }
//< op-constant
//> Types of Values interpret-literals
      case OP_NIL: push(NIL_VAL); break;
      case OP_TRUE: push(BOOL_VAL(true)); break;
      case OP_FALSE: push(BOOL_VAL(false)); break;
//< Types of Values interpret-literals
//> Global Variables interpret-pop
      case OP_POP: pop(); break;
//< Global Variables interpret-pop
//> Local Variables interpret-get-local
      case OP_GET_LOCAL: {
        uint8_t slot = READ_BYTE();
/* Local Variables interpret-get-local < Calls and Functions push-local
        push(vm.stack[slot]); // [slot]
*/
//> Calls and Functions push-local
        push(frame->slots[slot]);
//< Calls and Functions push-local
        break;
      }
//< Local Variables interpret-get-local
//> Local Variables interpret-set-local
      case OP_SET_LOCAL: {
        uint8_t slot = READ_BYTE();
/* Local Variables interpret-set-local < Calls and Functions set-local
        vm.stack[slot] = peek(0);
*/
//> Calls and Functions set-local
        frame->slots[slot] = peek_vm(0);
//< Calls and Functions set-local
        break;
      }
//< Local Variables interpret-set-local
//> Global Variables interpret-get-global
      case OP_GET_GLOBAL: {
        ObjString* name = READ_STRING();
        Value value;
        if (!tableGet(&vm.globals, name, &value)) {
          runtimeError("Undefined variable '%s'.", name->chars);
          return INTERPRET_RUNTIME_ERROR;
        }
        push(value);
        break;
      }
//< Global Variables interpret-get-global
//> Global Variables interpret-define-global
      case OP_DEFINE_GLOBAL: {
        ObjString* name = READ_STRING();
        tableSet(&vm.globals, name, peek_vm(0));
        pop();
        break;
      }
//< Global Variables interpret-define-global
//> Global Variables interpret-set-global
      case OP_SET_GLOBAL: {
        ObjString* name = READ_STRING();
        if (tableSet(&vm.globals, name, peek_vm(0))) {
          tableDelete(&vm.globals, name); // [delete]
          runtimeError("Undefined variable '%s'.", name->chars);
          return INTERPRET_RUNTIME_ERROR;
        }
        break;
      }
//< Global Variables interpret-set-global
//> Closures interpret-get-upvalue
      case OP_GET_UPVALUE: {
        uint8_t slot = READ_BYTE();
        push(*frame->closure->upvalues[slot]->location);
        break;
      }
//< Closures interpret-get-upvalue
//> Closures interpret-set-upvalue
      case OP_SET_UPVALUE: {
        uint8_t slot = READ_BYTE();
        *frame->closure->upvalues[slot]->location = peek_vm(0);
        break;
      }
//< Closures interpret-set-upvalue
//> Classes and Instances interpret-get-property
      case OP_GET_PROPERTY: {
//> get-not-instance
        if (!IS_INSTANCE(peek_vm(0))) {
          runtimeError("Only instances have properties.");
          return INTERPRET_RUNTIME_ERROR;
        }

//< get-not-instance
        ObjInstance* instance = AS_INSTANCE(peek_vm(0));
        ObjString* name = READ_STRING();

        Value value;
        if (tableGet(&instance->fields, name, &value)) {
          pop(); // Instance.
          push(value);
          break;
        }
//> get-undefined

//< get-undefined
/* Classes and Instances get-undefined < Methods and Initializers get-method
        runtimeError("Undefined property '%s'.", name->chars);
        return INTERPRET_RUNTIME_ERROR;
*/
//> Methods and Initializers get-method
        if (!bindMethod(instance->klass, name)) {
          return INTERPRET_RUNTIME_ERROR;
        }
        break;
//< Methods and Initializers get-method
      }
//< Classes and Instances interpret-get-property
//> Classes and Instances interpret-set-property
      case OP_SET_PROPERTY: {
//> set-not-instance
        if (!IS_INSTANCE(peek_vm(1))) {
          runtimeError("Only instances have fields.");
          return INTERPRET_RUNTIME_ERROR;
        }

//< set-not-instance
        ObjInstance* instance = AS_INSTANCE(peek_vm(1));
        tableSet(&instance->fields, READ_STRING(), peek_vm(0));
        Value value = pop();
        pop();
        push(value);
        break;
      }
//< Classes and Instances interpret-set-property
//> Superclasses interpret-get-super
      case OP_GET_SUPER: {
        ObjString* name = READ_STRING();
        ObjClass* superclass = AS_CLASS(pop());

        if (!bindMethod(superclass, name)) {
          return INTERPRET_RUNTIME_ERROR;
        }
        break;
      }
//< Superclasses interpret-get-super
//> Types of Values interpret-equal
      case OP_EQUAL: {
        Value b = pop();
        Value a = pop();
        push(BOOL_VAL(valuesEqual(a, b)));
        break;
      }
//< Types of Values interpret-equal
//> Types of Values interpret-comparison
      case OP_GREATER:  BINARY_OP(BOOL_VAL, >); break;
      case OP_LESS:     BINARY_OP(BOOL_VAL, <); break;
//< Types of Values interpret-comparison
/* A Virtual Machine op-binary < Types of Values op-arithmetic
      case OP_ADD:      BINARY_OP(+); break;
      case OP_SUBTRACT: BINARY_OP(-); break;
      case OP_MULTIPLY: BINARY_OP(*); break;
      case OP_DIVIDE:   BINARY_OP(/); break;
*/
/* A Virtual Machine op-negate < Types of Values op-negate
      case OP_NEGATE:   push(-pop()); break;
*/
/* Types of Values op-arithmetic < Strings add-strings
      case OP_ADD:      BINARY_OP(NUMBER_VAL, +); break;
*/
//> Strings add-strings
      case OP_ADD: {
        if (IS_STRING(peek_vm(0)) && IS_STRING(peek_vm(1))) {
          concatenate();
        } else if (IS_NUMBER(peek_vm(0)) && IS_NUMBER(peek_vm(1))) {
          double b = AS_NUMBER(pop());
          double a = AS_NUMBER(pop());
          push(NUMBER_VAL(a + b));
        } else {
          runtimeError(
              "Operands must be two numbers or two strings.");
          return INTERPRET_RUNTIME_ERROR;
        }
        break;
      }
//< Strings add-strings
//> Types of Values op-arithmetic
      case OP_SUBTRACT: BINARY_OP(NUMBER_VAL, -); break;
      case OP_MULTIPLY: BINARY_OP(NUMBER_VAL, *); break;
      case OP_DIVIDE:   BINARY_OP(NUMBER_VAL, /); break;
//< Types of Values op-arithmetic
//> Types of Values op-not
      case OP_NOT:
        push(BOOL_VAL(isFalsey(pop())));
        break;
//< Types of Values op-not
//> Types of Values op-negate
      case OP_NEGATE:
        if (!IS_NUMBER(peek_vm(0))) {
          runtimeError("Operand must be a number.");
          return INTERPRET_RUNTIME_ERROR;
        }
        push(NUMBER_VAL(-AS_NUMBER(pop())));
        break;
//< Types of Values op-negate
//> Global Variables interpret-print
      case OP_PRINT: {
        printValue(pop());
        printf("\n");
        break;
      }
//< Global Variables interpret-print
//> Jumping Back and Forth op-jump
      case OP_JUMP: {
        uint16_t offset = READ_SHORT();
/* Jumping Back and Forth op-jump < Calls and Functions jump
        vm.ip += offset;
*/
//> Calls and Functions jump
        frame->ip += offset;
//< Calls and Functions jump
        break;
      }
//< Jumping Back and Forth op-jump
//> Jumping Back and Forth op-jump-if-false
      case OP_JUMP_IF_FALSE: {
        uint16_t offset = READ_SHORT();
/* Jumping Back and Forth op-jump-if-false < Calls and Functions jump-if-false
        if (isFalsey(peek(0))) vm.ip += offset;
*/
//> Calls and Functions jump-if-false
        if (isFalsey(peek_vm(0))) frame->ip += offset;
//< Calls and Functions jump-if-false
        break;
      }
//< Jumping Back and Forth op-jump-if-false
//> Jumping Back and Forth op-loop
      case OP_LOOP: {
        uint16_t offset = READ_SHORT();
/* Jumping Back and Forth op-loop < Calls and Functions loop
        vm.ip -= offset;
*/
//> Calls and Functions loop
        frame->ip -= offset;
//< Calls and Functions loop
        break;
      }
//< Jumping Back and Forth op-loop
//> Calls and Functions interpret-call
      case OP_CALL: {
        int argCount = READ_BYTE();
        if (!callValue(peek_vm(argCount), argCount)) {
          return INTERPRET_RUNTIME_ERROR;
        }
//> update-frame-after-call
        frame = &vm.frames[vm.frameCount - 1];
//< update-frame-after-call
        break;
      }
//< Calls and Functions interpret-call
//> Methods and Initializers interpret-invoke
      case OP_INVOKE: {
        ObjString* method = READ_STRING();
        int argCount = READ_BYTE();
        if (!invoke(method, argCount)) {
          return INTERPRET_RUNTIME_ERROR;
        }
        frame = &vm.frames[vm.frameCount - 1];
        break;
      }
//< Methods and Initializers interpret-invoke
//> Superclasses interpret-super-invoke
      case OP_SUPER_INVOKE: {
        ObjString* method = READ_STRING();
        int argCount = READ_BYTE();
        ObjClass* superclass = AS_CLASS(pop());
        if (!invokeFromClass(superclass, method, argCount)) {
          return INTERPRET_RUNTIME_ERROR;
        }
        frame = &vm.frames[vm.frameCount - 1];
        break;
      }
//< Superclasses interpret-super-invoke
//> Closures interpret-closure
      case OP_CLOSURE: {
        ObjFunction* function = AS_FUNCTION(READ_CONSTANT());
        ObjClosure* closure = newClosure(function);
        push(OBJ_VAL(closure));
//> interpret-capture-upvalues
        for (int i = 0; i < closure->upvalueCount; i++) {
          uint8_t isLocal = READ_BYTE();
          uint8_t index = READ_BYTE();
          if (isLocal) {
            closure->upvalues[i] =
                captureUpvalue(frame->slots + index);
          } else {
            closure->upvalues[i] = frame->closure->upvalues[index];
          }
        }
//< interpret-capture-upvalues
        break;
      }
//< Closures interpret-closure
//> Closures interpret-close-upvalue
      case OP_CLOSE_UPVALUE:
        closeUpvalues(vm.stackTop - 1);
        pop();
        break;
//< Closures interpret-close-upvalue
      case OP_RETURN: {
/* A Virtual Machine print-return < Global Variables op-return
        printValue(pop());
        printf("\n");
*/
/* Global Variables op-return < Calls and Functions interpret-return
        // Exit interpreter.
*/
/* A Virtual Machine run < Calls and Functions interpret-return
        return INTERPRET_OK;
*/
//> Calls and Functions interpret-return
        Value result = pop();
//> Closures return-close-upvalues
        closeUpvalues(frame->slots);
//< Closures return-close-upvalues
        vm.frameCount--;
        if (vm.frameCount == 0) {
          pop();
          return INTERPRET_OK;
        }

        vm.stackTop = frame->slots;
        push(result);
        frame = &vm.frames[vm.frameCount - 1];
        break;
//< Calls and Functions interpret-return
      }
//> Classes and Instances interpret-class
      case OP_CLASS:
        push(OBJ_VAL(newClass(READ_STRING())));
        break;
//< Classes and Instances interpret-class
//> Superclasses interpret-inherit
      case OP_INHERIT: {
        Value superclass = peek_vm(1);
//> inherit-non-class
        if (!IS_CLASS(superclass)) {
          runtimeError("Superclass must be a class.");
          return INTERPRET_RUNTIME_ERROR;
        }

//< inherit-non-class
        ObjClass* subclass = AS_CLASS(peek_vm(0));
        tableAddAll(&AS_CLASS(superclass)->methods,
                    &subclass->methods);
        pop(); // Subclass.
        break;
      }
//< Superclasses interpret-inherit
//> Methods and Initializers interpret-method
      case OP_METHOD:
        defineMethod(READ_STRING());
        break;
//< Methods and Initializers interpret-method
    }
  }

#undef READ_BYTE
//> Jumping Back and Forth undef-read-short
#undef READ_SHORT
//< Jumping Back and Forth undef-read-short
//> undef-read-constant
#undef READ_CONSTANT
//< undef-read-constant
//> Global Variables undef-read-string
#undef READ_STRING
//< Global Variables undef-read-string
//> undef-binary-op
#undef BINARY_OP
//< undef-binary-op
}
//< run
//> omit
void hack(bool b) {
  // Hack to avoid unused function error. run() is not used in the
  // scanning chapter.
  run();
  if (b) hack(false);
}
//< omit
//> interpret
/* A Virtual Machine interpret < Scanning on Demand vm-interpret-c
InterpretResult interpret(Chunk* chunk) {
  vm.chunk = chunk;
  vm.ip = vm.chunk->code;
  return run();
*/
//> Scanning on Demand vm-interpret-c
InterpretResult interpret(const char* source) {
/* Scanning on Demand vm-interpret-c < Compiling Expressions interpret-chunk
  compile(source);
  return INTERPRET_OK;
*/
/* Compiling Expressions interpret-chunk < Calls and Functions interpret-stub
  Chunk chunk;
  initChunk(&chunk);

  if (!compile(source, &chunk)) {
    freeChunk(&chunk);
    return INTERPRET_COMPILE_ERROR;
  }

  vm.chunk = &chunk;
  vm.ip = vm.chunk->code;
*/
//> Calls and Functions interpret-stub
  ObjFunction* function = compile(source);
  if (function == NULL) return INTERPRET_COMPILE_ERROR;

  push(OBJ_VAL(function));
//< Calls and Functions interpret-stub
/* Calls and Functions interpret-stub < Calls and Functions interpret
  CallFrame* frame = &vm.frames[vm.frameCount++];
  frame->function = function;
  frame->ip = function->chunk.code;
  frame->slots = vm.stack;
*/
/* Calls and Functions interpret < Closures interpret
  call(function, 0);
*/
//> Closures interpret
  ObjClosure* closure = newClosure(function);
  pop();
  push(OBJ_VAL(closure));
  call_vm(closure, 0);
//< Closures interpret
//< Scanning on Demand vm-interpret-c
//> Compiling Expressions interpret-chunk

/* Compiling Expressions interpret-chunk < Calls and Functions end-interpret
  InterpretResult result = run();

  freeChunk(&chunk);
  return result;
*/
//> Calls and Functions end-interpret
  return run();
//< Calls and Functions end-interpret
//< Compiling Expressions interpret-chunk
}
//< interpret



//> Chunks of Bytecode main-c
//> Scanning on Demand main-includes
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//< Scanning on Demand main-includes
//> main-include-chunk
//< main-include-chunk
//> main-include-debug
//< main-include-debug
//> A Virtual Machine main-include-vm
//< A Virtual Machine main-include-vm
//> Scanning on Demand repl

static void repl() {
  char line[1024];
  for (;;) {
    printf("> ");

    if (!fgets(line, sizeof(line), stdin)) {
      printf("\n");
      break;
    }

    interpret(line);
  }
}
//< Scanning on Demand repl
//> Scanning on Demand read-file
static char* readFile(const char* path) {
  FILE* file = fopen(path, "rb");
//> no-file
  if (file == NULL) {
    fprintf(stderr, "Could not open file \"%s\".\n", path);
    exit(74);
  }
//< no-file

  fseek(file, 0L, SEEK_END);
  size_t fileSize = ftell(file);
  rewind(file);

  char* buffer = (char*)malloc(fileSize + 1);
//> no-buffer
  if (buffer == NULL) {
    fprintf(stderr, "Not enough memory to read \"%s\".\n", path);
    exit(74);
  }
  
//< no-buffer
  size_t bytesRead = fread(buffer, sizeof(char), fileSize, file);
//> no-read
  if (bytesRead < fileSize) {
    fprintf(stderr, "Could not read file \"%s\".\n", path);
    exit(74);
  }
  
//< no-read
  buffer[bytesRead] = '\0';

  fclose(file);
  return buffer;
}
//< Scanning on Demand read-file
//> Scanning on Demand run-file
static void runFile(const char* path) {
  char* source = readFile(path);
  InterpretResult result = interpret(source);
  free(source); // [owner]

  if (result == INTERPRET_COMPILE_ERROR) exit(65);
  if (result == INTERPRET_RUNTIME_ERROR) exit(70);
}
//< Scanning on Demand run-file

int main(int argc, const char* argv[]) {
//> A Virtual Machine main-init-vm
  initVM();

//< A Virtual Machine main-init-vm
/* Chunks of Bytecode main-chunk < Scanning on Demand args
  Chunk chunk;
  initChunk(&chunk);
*/
/* Chunks of Bytecode main-constant < Scanning on Demand args

  int constant = addConstant(&chunk, 1.2);
*/
/* Chunks of Bytecode main-constant < Chunks of Bytecode main-chunk-line
  writeChunk(&chunk, OP_CONSTANT);
  writeChunk(&chunk, constant);

*/
/* Chunks of Bytecode main-chunk-line < Scanning on Demand args
  writeChunk(&chunk, OP_CONSTANT, 123);
  writeChunk(&chunk, constant, 123);
*/
/* A Virtual Machine main-chunk < Scanning on Demand args

  constant = addConstant(&chunk, 3.4);
  writeChunk(&chunk, OP_CONSTANT, 123);
  writeChunk(&chunk, constant, 123);

  writeChunk(&chunk, OP_ADD, 123);

  constant = addConstant(&chunk, 5.6);
  writeChunk(&chunk, OP_CONSTANT, 123);
  writeChunk(&chunk, constant, 123);

  writeChunk(&chunk, OP_DIVIDE, 123);
*/
/* A Virtual Machine main-negate < Scanning on Demand args
  writeChunk(&chunk, OP_NEGATE, 123);
*/
/* Chunks of Bytecode main-chunk < Chunks of Bytecode main-chunk-line
  writeChunk(&chunk, OP_RETURN);
*/
/* Chunks of Bytecode main-chunk-line < Scanning on Demand args

  writeChunk(&chunk, OP_RETURN, 123);
*/
/* Chunks of Bytecode main-disassemble-chunk < Scanning on Demand args

  disassembleChunk(&chunk, "test chunk");
*/
/* A Virtual Machine main-interpret < Scanning on Demand args
  interpret(&chunk);
*/
//> Scanning on Demand args
  if (argc == 1) {
    repl();
  } else if (argc == 2) {
    runFile(argv[1]);
  } else {
    fprintf(stderr, "Usage: clox [path]\n");
    exit(64);
  }
  
  freeVM();
//< Scanning on Demand args
/* A Virtual Machine main-free-vm < Scanning on Demand args
  freeVM();
*/
/* Chunks of Bytecode main-chunk < Scanning on Demand args
  freeChunk(&chunk);
*/
  return 0;
}



