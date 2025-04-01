#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn rewind(__stream: *mut FILE);
    fn clock() -> clock_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Obj {
    pub type_0: ObjType,
    pub isMarked: bool,
    pub next: *mut Obj,
}
pub type ObjType = libc::c_uint;
pub const OBJ_UPVALUE: ObjType = 7;
pub const OBJ_STRING: ObjType = 6;
pub const OBJ_NATIVE: ObjType = 5;
pub const OBJ_INSTANCE: ObjType = 4;
pub const OBJ_FUNCTION: ObjType = 3;
pub const OBJ_CLOSURE: ObjType = 2;
pub const OBJ_CLASS: ObjType = 1;
pub const OBJ_BOUND_METHOD: ObjType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjString {
    pub obj: Obj,
    pub length: libc::c_int,
    pub chars: *mut libc::c_char,
    pub hash: uint32_t,
}
pub type Value = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueArray {
    pub capacity: libc::c_int,
    pub count: libc::c_int,
    pub values: *mut Value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VM {
    pub frames: [CallFrame; 64],
    pub frameCount: libc::c_int,
    pub stack: [Value; 16384],
    pub stackTop: *mut Value,
    pub globals: Table,
    pub strings: Table,
    pub initString: *mut ObjString,
    pub openUpvalues: *mut ObjUpvalue,
    pub bytesAllocated: size_t,
    pub nextGC: size_t,
    pub objects: *mut Obj,
    pub grayCount: libc::c_int,
    pub grayCapacity: libc::c_int,
    pub grayStack: *mut *mut Obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjUpvalue {
    pub obj: Obj,
    pub location: *mut Value,
    pub closed: Value,
    pub next: *mut ObjUpvalue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub count: libc::c_int,
    pub capacity: libc::c_int,
    pub entries: *mut Entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Entry {
    pub key: *mut ObjString,
    pub value: Value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallFrame {
    pub closure: *mut ObjClosure,
    pub ip: *mut uint8_t,
    pub slots: *mut Value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjClosure {
    pub obj: Obj,
    pub function: *mut ObjFunction,
    pub upvalues: *mut *mut ObjUpvalue,
    pub upvalueCount: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjFunction {
    pub obj: Obj,
    pub arity: libc::c_int,
    pub upvalueCount: libc::c_int,
    pub chunk: Chunk,
    pub name: *mut ObjString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Chunk {
    pub count: libc::c_int,
    pub capacity: libc::c_int,
    pub code: *mut uint8_t,
    pub lines: *mut libc::c_int,
    pub constants: ValueArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjNative {
    pub obj: Obj,
    pub function: NativeFn,
}
pub type NativeFn = Option::<unsafe extern "C" fn(libc::c_int, *mut Value) -> Value>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjInstance {
    pub obj: Obj,
    pub klass: *mut ObjClass,
    pub fields: Table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjClass {
    pub obj: Obj,
    pub name: *mut ObjString,
    pub methods: Table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjBoundMethod {
    pub obj: Obj,
    pub receiver: Value,
    pub method: *mut ObjClosure,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Compiler {
    pub enclosing: *mut Compiler,
    pub function: *mut ObjFunction,
    pub type_0: FunctionType,
    pub locals: [Local; 256],
    pub localCount: libc::c_int,
    pub upvalues: [Upvalue; 256],
    pub scopeDepth: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upvalue {
    pub index: uint8_t,
    pub isLocal: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Local {
    pub name: Token,
    pub depth: libc::c_int,
    pub isCaptured: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub type_0: TokenType,
    pub start: *const libc::c_char,
    pub length: libc::c_int,
    pub line: libc::c_int,
}
pub type TokenType = libc::c_uint;
pub const TOKEN_EOF: TokenType = 39;
pub const TOKEN_ERROR: TokenType = 38;
pub const TOKEN_WHILE: TokenType = 37;
pub const TOKEN_VAR: TokenType = 36;
pub const TOKEN_TRUE: TokenType = 35;
pub const TOKEN_THIS: TokenType = 34;
pub const TOKEN_SUPER: TokenType = 33;
pub const TOKEN_RETURN: TokenType = 32;
pub const TOKEN_PRINT: TokenType = 31;
pub const TOKEN_OR: TokenType = 30;
pub const TOKEN_NIL: TokenType = 29;
pub const TOKEN_IF: TokenType = 28;
pub const TOKEN_FUN: TokenType = 27;
pub const TOKEN_FOR: TokenType = 26;
pub const TOKEN_FALSE: TokenType = 25;
pub const TOKEN_ELSE: TokenType = 24;
pub const TOKEN_CLASS: TokenType = 23;
pub const TOKEN_AND: TokenType = 22;
pub const TOKEN_NUMBER: TokenType = 21;
pub const TOKEN_STRING: TokenType = 20;
pub const TOKEN_IDENTIFIER: TokenType = 19;
pub const TOKEN_LESS_EQUAL: TokenType = 18;
pub const TOKEN_LESS: TokenType = 17;
pub const TOKEN_GREATER_EQUAL: TokenType = 16;
pub const TOKEN_GREATER: TokenType = 15;
pub const TOKEN_EQUAL_EQUAL: TokenType = 14;
pub const TOKEN_EQUAL: TokenType = 13;
pub const TOKEN_BANG_EQUAL: TokenType = 12;
pub const TOKEN_BANG: TokenType = 11;
pub const TOKEN_STAR: TokenType = 10;
pub const TOKEN_SLASH: TokenType = 9;
pub const TOKEN_SEMICOLON: TokenType = 8;
pub const TOKEN_PLUS: TokenType = 7;
pub const TOKEN_MINUS: TokenType = 6;
pub const TOKEN_DOT: TokenType = 5;
pub const TOKEN_COMMA: TokenType = 4;
pub const TOKEN_RIGHT_BRACE: TokenType = 3;
pub const TOKEN_LEFT_BRACE: TokenType = 2;
pub const TOKEN_RIGHT_PAREN: TokenType = 1;
pub const TOKEN_LEFT_PAREN: TokenType = 0;
pub type FunctionType = libc::c_uint;
pub const TYPE_SCRIPT: FunctionType = 3;
pub const TYPE_METHOD: FunctionType = 2;
pub const TYPE_INITIALIZER: FunctionType = 1;
pub const TYPE_FUNCTION: FunctionType = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const OP_METHOD: C2RustUnnamed = 36;
pub const OP_INHERIT: C2RustUnnamed = 35;
pub const OP_CLASS: C2RustUnnamed = 34;
pub const OP_RETURN: C2RustUnnamed = 33;
pub const OP_CLOSE_UPVALUE: C2RustUnnamed = 32;
pub const OP_CLOSURE: C2RustUnnamed = 31;
pub const OP_SUPER_INVOKE: C2RustUnnamed = 30;
pub const OP_INVOKE: C2RustUnnamed = 29;
pub const OP_CALL: C2RustUnnamed = 28;
pub const OP_LOOP: C2RustUnnamed = 27;
pub const OP_JUMP_IF_FALSE: C2RustUnnamed = 26;
pub const OP_JUMP: C2RustUnnamed = 25;
pub const OP_PRINT: C2RustUnnamed = 24;
pub const OP_NEGATE: C2RustUnnamed = 23;
pub const OP_NOT: C2RustUnnamed = 22;
pub const OP_DIVIDE: C2RustUnnamed = 21;
pub const OP_MULTIPLY: C2RustUnnamed = 20;
pub const OP_SUBTRACT: C2RustUnnamed = 19;
pub const OP_ADD: C2RustUnnamed = 18;
pub const OP_LESS: C2RustUnnamed = 17;
pub const OP_GREATER: C2RustUnnamed = 16;
pub const OP_EQUAL: C2RustUnnamed = 15;
pub const OP_GET_SUPER: C2RustUnnamed = 14;
pub const OP_SET_PROPERTY: C2RustUnnamed = 13;
pub const OP_GET_PROPERTY: C2RustUnnamed = 12;
pub const OP_SET_UPVALUE: C2RustUnnamed = 11;
pub const OP_GET_UPVALUE: C2RustUnnamed = 10;
pub const OP_SET_GLOBAL: C2RustUnnamed = 9;
pub const OP_DEFINE_GLOBAL: C2RustUnnamed = 8;
pub const OP_GET_GLOBAL: C2RustUnnamed = 7;
pub const OP_SET_LOCAL: C2RustUnnamed = 6;
pub const OP_GET_LOCAL: C2RustUnnamed = 5;
pub const OP_POP: C2RustUnnamed = 4;
pub const OP_FALSE: C2RustUnnamed = 3;
pub const OP_TRUE: C2RustUnnamed = 2;
pub const OP_NIL: C2RustUnnamed = 1;
pub const OP_CONSTANT: C2RustUnnamed = 0;
pub type InterpretResult = libc::c_uint;
pub const INTERPRET_RUNTIME_ERROR: InterpretResult = 2;
pub const INTERPRET_COMPILE_ERROR: InterpretResult = 1;
pub const INTERPRET_OK: InterpretResult = 0;
pub type clock_t = __clock_t;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Parser {
    pub current: Token,
    pub previous: Token,
    pub hadError: bool,
    pub panicMode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Scanner {
    pub start: *const libc::c_char,
    pub current: *const libc::c_char,
    pub line: libc::c_int,
}
pub type Precedence = libc::c_uint;
pub const PREC_PRIMARY: Precedence = 10;
pub const PREC_CALL: Precedence = 9;
pub const PREC_UNARY: Precedence = 8;
pub const PREC_FACTOR: Precedence = 7;
pub const PREC_TERM: Precedence = 6;
pub const PREC_COMPARISON: Precedence = 5;
pub const PREC_EQUALITY: Precedence = 4;
pub const PREC_AND: Precedence = 3;
pub const PREC_OR: Precedence = 2;
pub const PREC_ASSIGNMENT: Precedence = 1;
pub const PREC_NONE: Precedence = 0;
pub type ParseFn = Option::<unsafe extern "C" fn(bool) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParseRule {
    pub prefix: ParseFn,
    pub infix: ParseFn,
    pub precedence: Precedence,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClassCompiler {
    pub enclosing: *mut ClassCompiler,
    pub hasSuperclass: bool,
}
#[inline]
unsafe extern "C" fn valueToNum(mut value: Value) -> libc::c_double {
    let mut num: libc::c_double = 0.;
    memcpy(
        &mut num as *mut libc::c_double as *mut libc::c_void,
        &mut value as *mut Value as *const libc::c_void,
        ::core::mem::size_of::<Value>() as libc::c_ulong,
    );
    return num;
}
#[inline]
unsafe extern "C" fn numToValue(mut num: libc::c_double) -> Value {
    let mut value: Value = 0;
    memcpy(
        &mut value as *mut Value as *mut libc::c_void,
        &mut num as *mut libc::c_double as *const libc::c_void,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
    return value;
}
#[inline]
unsafe extern "C" fn isObjType(mut value: Value, mut type_0: ObjType) -> bool {
    return value
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((value
            & !(0x8000000000000000 as libc::c_ulong
                | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj))
            .type_0 as libc::c_uint == type_0 as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn initChunk(mut chunk: *mut Chunk) {
    (*chunk).count = 0 as libc::c_int;
    (*chunk).capacity = 0 as libc::c_int;
    (*chunk).code = 0 as *mut uint8_t;
    (*chunk).lines = 0 as *mut libc::c_int;
    initValueArray(&mut (*chunk).constants);
}
#[no_mangle]
pub unsafe extern "C" fn freeChunk(mut chunk: *mut Chunk) {
    reallocate(
        (*chunk).code as *mut libc::c_void,
        (::core::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul((*chunk).capacity as libc::c_ulong),
        0 as libc::c_int as size_t,
    );
    reallocate(
        (*chunk).lines as *mut libc::c_void,
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((*chunk).capacity as libc::c_ulong),
        0 as libc::c_int as size_t,
    );
    freeValueArray(&mut (*chunk).constants);
    initChunk(chunk);
}
#[no_mangle]
pub unsafe extern "C" fn writeChunk(
    mut chunk: *mut Chunk,
    mut byte: uint8_t,
    mut line: libc::c_int,
) {
    if (*chunk).capacity < (*chunk).count + 1 as libc::c_int {
        let mut oldCapacity: libc::c_int = (*chunk).capacity;
        (*chunk)
            .capacity = if oldCapacity < 8 as libc::c_int {
            8 as libc::c_int
        } else {
            oldCapacity * 2 as libc::c_int
        };
        (*chunk)
            .code = reallocate(
            (*chunk).code as *mut libc::c_void,
            (::core::mem::size_of::<uint8_t>() as libc::c_ulong)
                .wrapping_mul(oldCapacity as libc::c_ulong),
            (::core::mem::size_of::<uint8_t>() as libc::c_ulong)
                .wrapping_mul((*chunk).capacity as libc::c_ulong),
        ) as *mut uint8_t;
        (*chunk)
            .lines = reallocate(
            (*chunk).lines as *mut libc::c_void,
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(oldCapacity as libc::c_ulong),
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul((*chunk).capacity as libc::c_ulong),
        ) as *mut libc::c_int;
    }
    *((*chunk).code).offset((*chunk).count as isize) = byte;
    *((*chunk).lines).offset((*chunk).count as isize) = line;
    (*chunk).count += 1;
    (*chunk).count;
}
#[no_mangle]
pub unsafe extern "C" fn addConstant(
    mut chunk: *mut Chunk,
    mut value: Value,
) -> libc::c_int {
    push(value);
    writeValueArray(&mut (*chunk).constants, value);
    pop();
    return (*chunk).constants.count - 1 as libc::c_int;
}
#[no_mangle]
pub static mut parser: Parser = Parser {
    current: Token {
        type_0: TOKEN_LEFT_PAREN,
        start: 0 as *const libc::c_char,
        length: 0,
        line: 0,
    },
    previous: Token {
        type_0: TOKEN_LEFT_PAREN,
        start: 0 as *const libc::c_char,
        length: 0,
        line: 0,
    },
    hadError: false,
    panicMode: false,
};
#[no_mangle]
pub static mut current: *mut Compiler = 0 as *const Compiler as *mut Compiler;
#[no_mangle]
pub static mut currentClass: *mut ClassCompiler = 0 as *const ClassCompiler
    as *mut ClassCompiler;
unsafe extern "C" fn currentChunk() -> *mut Chunk {
    return &mut (*(*current).function).chunk;
}
unsafe extern "C" fn errorAt(mut token: *mut Token, mut message: *const libc::c_char) {
    if parser.panicMode {
        return;
    }
    parser.panicMode = 1 as libc::c_int != 0;
    fprintf(
        stderr,
        b"[line %d] Error\0" as *const u8 as *const libc::c_char,
        (*token).line,
    );
    if (*token).type_0 as libc::c_uint == TOKEN_EOF as libc::c_int as libc::c_uint {
        fprintf(stderr, b" at end\0" as *const u8 as *const libc::c_char);
    } else if !((*token).type_0 as libc::c_uint
        == TOKEN_ERROR as libc::c_int as libc::c_uint)
    {
        fprintf(
            stderr,
            b" at '%.*s'\0" as *const u8 as *const libc::c_char,
            (*token).length,
            (*token).start,
        );
    }
    fprintf(stderr, b": %s\n\0" as *const u8 as *const libc::c_char, message);
    parser.hadError = 1 as libc::c_int != 0;
}
unsafe extern "C" fn error(mut message: *const libc::c_char) {
    errorAt(&mut parser.previous, message);
}
unsafe extern "C" fn errorAtCurrent(mut message: *const libc::c_char) {
    errorAt(&mut parser.current, message);
}
unsafe extern "C" fn advance_compiler() {
    parser.previous = parser.current;
    loop {
        parser.current = scanToken();
        if parser.current.type_0 as libc::c_uint
            != TOKEN_ERROR as libc::c_int as libc::c_uint
        {
            break;
        }
        errorAtCurrent(parser.current.start);
    };
}
unsafe extern "C" fn consume(mut type_0: TokenType, mut message: *const libc::c_char) {
    if parser.current.type_0 as libc::c_uint == type_0 as libc::c_uint {
        advance_compiler();
        return;
    }
    errorAtCurrent(message);
}
unsafe extern "C" fn check(mut type_0: TokenType) -> bool {
    return parser.current.type_0 as libc::c_uint == type_0 as libc::c_uint;
}
unsafe extern "C" fn match_compiler(mut type_0: TokenType) -> bool {
    if !check(type_0) {
        return 0 as libc::c_int != 0;
    }
    advance_compiler();
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn emitByte(mut byte: uint8_t) {
    writeChunk(currentChunk(), byte, parser.previous.line);
}
unsafe extern "C" fn emitBytes(mut byte1: uint8_t, mut byte2: uint8_t) {
    emitByte(byte1);
    emitByte(byte2);
}
unsafe extern "C" fn emitLoop(mut loopStart: libc::c_int) {
    emitByte(OP_LOOP as libc::c_int as uint8_t);
    let mut offset: libc::c_int = (*currentChunk()).count - loopStart + 2 as libc::c_int;
    if offset > 65535 as libc::c_int {
        error(b"Loop body too large.\0" as *const u8 as *const libc::c_char);
    }
    emitByte((offset >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t);
    emitByte((offset & 0xff as libc::c_int) as uint8_t);
}
unsafe extern "C" fn emitJump(mut instruction: uint8_t) -> libc::c_int {
    emitByte(instruction);
    emitByte(0xff as libc::c_int as uint8_t);
    emitByte(0xff as libc::c_int as uint8_t);
    return (*currentChunk()).count - 2 as libc::c_int;
}
unsafe extern "C" fn emitReturn() {
    if (*current).type_0 as libc::c_uint
        == TYPE_INITIALIZER as libc::c_int as libc::c_uint
    {
        emitBytes(OP_GET_LOCAL as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t);
    } else {
        emitByte(OP_NIL as libc::c_int as uint8_t);
    }
    emitByte(OP_RETURN as libc::c_int as uint8_t);
}
unsafe extern "C" fn makeConstant(mut value: Value) -> uint8_t {
    let mut constant: libc::c_int = addConstant(currentChunk(), value);
    if constant > 255 as libc::c_int {
        error(b"Too many constants in one chunk.\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int as uint8_t;
    }
    return constant as uint8_t;
}
unsafe extern "C" fn emitConstant(mut value: Value) {
    emitBytes(OP_CONSTANT as libc::c_int as uint8_t, makeConstant(value));
}
unsafe extern "C" fn patchJump(mut offset: libc::c_int) {
    let mut jump: libc::c_int = (*currentChunk()).count - offset - 2 as libc::c_int;
    if jump > 65535 as libc::c_int {
        error(b"Too much code to jump over.\0" as *const u8 as *const libc::c_char);
    }
    *((*currentChunk()).code)
        .offset(
            offset as isize,
        ) = (jump >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    *((*currentChunk()).code)
        .offset(
            (offset + 1 as libc::c_int) as isize,
        ) = (jump & 0xff as libc::c_int) as uint8_t;
}
unsafe extern "C" fn initCompiler(
    mut compiler: *mut Compiler,
    mut type_0: FunctionType,
) {
    (*compiler).enclosing = current;
    (*compiler).function = 0 as *mut ObjFunction;
    (*compiler).type_0 = type_0;
    (*compiler).localCount = 0 as libc::c_int;
    (*compiler).scopeDepth = 0 as libc::c_int;
    (*compiler).function = newFunction();
    current = compiler;
    if type_0 as libc::c_uint != TYPE_SCRIPT as libc::c_int as libc::c_uint {
        (*(*current).function)
            .name = copyString(parser.previous.start, parser.previous.length);
    }
    let fresh0 = (*current).localCount;
    (*current).localCount = (*current).localCount + 1;
    let mut local: *mut Local = &mut *((*current).locals)
        .as_mut_ptr()
        .offset(fresh0 as isize) as *mut Local;
    (*local).depth = 0 as libc::c_int;
    (*local).isCaptured = 0 as libc::c_int != 0;
    if type_0 as libc::c_uint != TYPE_FUNCTION as libc::c_int as libc::c_uint {
        (*local).name.start = b"this\0" as *const u8 as *const libc::c_char;
        (*local).name.length = 4 as libc::c_int;
    } else {
        (*local).name.start = b"\0" as *const u8 as *const libc::c_char;
        (*local).name.length = 0 as libc::c_int;
    };
}
unsafe extern "C" fn endCompiler() -> *mut ObjFunction {
    emitReturn();
    let mut function_0: *mut ObjFunction = (*current).function;
    current = (*current).enclosing;
    return function_0;
}
unsafe extern "C" fn beginScope() {
    (*current).scopeDepth += 1;
    (*current).scopeDepth;
}
unsafe extern "C" fn endScope() {
    (*current).scopeDepth -= 1;
    (*current).scopeDepth;
    while (*current).localCount > 0 as libc::c_int
        && (*current).locals[((*current).localCount - 1 as libc::c_int) as usize].depth
            > (*current).scopeDepth
    {
        if (*current)
            .locals[((*current).localCount - 1 as libc::c_int) as usize]
            .isCaptured
        {
            emitByte(OP_CLOSE_UPVALUE as libc::c_int as uint8_t);
        } else {
            emitByte(OP_POP as libc::c_int as uint8_t);
        }
        (*current).localCount -= 1;
        (*current).localCount;
    }
}
unsafe extern "C" fn identifierConstant(mut name: *mut Token) -> uint8_t {
    return makeConstant(
        0x8000000000000000 as libc::c_ulong
            | 0x7ffc000000000000 as libc::c_long as uint64_t
            | copyString((*name).start, (*name).length) as uintptr_t,
    );
}
unsafe extern "C" fn identifiersEqual(mut a: *mut Token, mut b: *mut Token) -> bool {
    if (*a).length != (*b).length {
        return 0 as libc::c_int != 0;
    }
    return memcmp(
        (*a).start as *const libc::c_void,
        (*b).start as *const libc::c_void,
        (*a).length as libc::c_ulong,
    ) == 0 as libc::c_int;
}
unsafe extern "C" fn resolveLocal(
    mut compiler: *mut Compiler,
    mut name: *mut Token,
) -> libc::c_int {
    let mut i: libc::c_int = (*compiler).localCount - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut local: *mut Local = &mut *((*compiler).locals)
            .as_mut_ptr()
            .offset(i as isize) as *mut Local;
        if identifiersEqual(name, &mut (*local).name) {
            if (*local).depth == -(1 as libc::c_int) {
                error(
                    b"Can't read local variable in its own initializer.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return i;
        }
        i -= 1;
        i;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn addUpvalue(
    mut compiler: *mut Compiler,
    mut index: uint8_t,
    mut isLocal: bool,
) -> libc::c_int {
    let mut upvalueCount: libc::c_int = (*(*compiler).function).upvalueCount;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < upvalueCount {
        let mut upvalue: *mut Upvalue = &mut *((*compiler).upvalues)
            .as_mut_ptr()
            .offset(i as isize) as *mut Upvalue;
        if (*upvalue).index as libc::c_int == index as libc::c_int
            && (*upvalue).isLocal as libc::c_int == isLocal as libc::c_int
        {
            return i;
        }
        i += 1;
        i;
    }
    if upvalueCount == 255 as libc::c_int + 1 as libc::c_int {
        error(
            b"Too many closure variables in function.\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    (*compiler).upvalues[upvalueCount as usize].isLocal = isLocal;
    (*compiler).upvalues[upvalueCount as usize].index = index;
    let fresh1 = (*(*compiler).function).upvalueCount;
    (*(*compiler).function).upvalueCount = (*(*compiler).function).upvalueCount + 1;
    return fresh1;
}
unsafe extern "C" fn resolveUpvalue(
    mut compiler: *mut Compiler,
    mut name: *mut Token,
) -> libc::c_int {
    if ((*compiler).enclosing).is_null() {
        return -(1 as libc::c_int);
    }
    let mut local: libc::c_int = resolveLocal((*compiler).enclosing, name);
    if local != -(1 as libc::c_int) {
        (*(*compiler).enclosing)
            .locals[local as usize]
            .isCaptured = 1 as libc::c_int != 0;
        return addUpvalue(compiler, local as uint8_t, 1 as libc::c_int != 0);
    }
    let mut upvalue: libc::c_int = resolveUpvalue((*compiler).enclosing, name);
    if upvalue != -(1 as libc::c_int) {
        return addUpvalue(compiler, upvalue as uint8_t, 0 as libc::c_int != 0);
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn addLocal(mut name: Token) {
    if (*current).localCount == 255 as libc::c_int + 1 as libc::c_int {
        error(
            b"Too many local variables in function.\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    let fresh2 = (*current).localCount;
    (*current).localCount = (*current).localCount + 1;
    let mut local: *mut Local = &mut *((*current).locals)
        .as_mut_ptr()
        .offset(fresh2 as isize) as *mut Local;
    (*local).name = name;
    (*local).depth = -(1 as libc::c_int);
    (*local).isCaptured = 0 as libc::c_int != 0;
}
unsafe extern "C" fn declareVariable() {
    if (*current).scopeDepth == 0 as libc::c_int {
        return;
    }
    let mut name: *mut Token = &mut parser.previous;
    let mut i: libc::c_int = (*current).localCount - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut local: *mut Local = &mut *((*current).locals)
            .as_mut_ptr()
            .offset(i as isize) as *mut Local;
        if (*local).depth != -(1 as libc::c_int)
            && (*local).depth < (*current).scopeDepth
        {
            break;
        }
        if identifiersEqual(name, &mut (*local).name) {
            error(
                b"Already a variable with this name in this scope.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        i -= 1;
        i;
    }
    addLocal(*name);
}
unsafe extern "C" fn parseVariable(mut errorMessage: *const libc::c_char) -> uint8_t {
    consume(TOKEN_IDENTIFIER, errorMessage);
    declareVariable();
    if (*current).scopeDepth > 0 as libc::c_int {
        return 0 as libc::c_int as uint8_t;
    }
    return identifierConstant(&mut parser.previous);
}
unsafe extern "C" fn markInitialized() {
    if (*current).scopeDepth == 0 as libc::c_int {
        return;
    }
    (*current)
        .locals[((*current).localCount - 1 as libc::c_int) as usize]
        .depth = (*current).scopeDepth;
}
unsafe extern "C" fn defineVariable(mut global: uint8_t) {
    if (*current).scopeDepth > 0 as libc::c_int {
        markInitialized();
        return;
    }
    emitBytes(OP_DEFINE_GLOBAL as libc::c_int as uint8_t, global);
}
unsafe extern "C" fn argumentList() -> uint8_t {
    let mut argCount: uint8_t = 0 as libc::c_int as uint8_t;
    if !check(TOKEN_RIGHT_PAREN) {
        loop {
            expression();
            if argCount as libc::c_int == 255 as libc::c_int {
                error(
                    b"Can't have more than 255 arguments.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            argCount = argCount.wrapping_add(1);
            argCount;
            if !match_compiler(TOKEN_COMMA) {
                break;
            }
        }
    }
    consume(
        TOKEN_RIGHT_PAREN,
        b"Expect ')' after arguments.\0" as *const u8 as *const libc::c_char,
    );
    return argCount;
}
unsafe extern "C" fn and_(mut canAssign: bool) {
    let mut endJump: libc::c_int = emitJump(OP_JUMP_IF_FALSE as libc::c_int as uint8_t);
    emitByte(OP_POP as libc::c_int as uint8_t);
    parsePrecedence(PREC_AND);
    patchJump(endJump);
}
unsafe extern "C" fn binary(mut canAssign: bool) {
    let mut operatorType: TokenType = parser.previous.type_0;
    let mut rule: *mut ParseRule = getRule(operatorType);
    parsePrecedence(
        ((*rule).precedence as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint) as Precedence,
    );
    match operatorType as libc::c_uint {
        12 => {
            emitBytes(
                OP_EQUAL as libc::c_int as uint8_t,
                OP_NOT as libc::c_int as uint8_t,
            );
        }
        14 => {
            emitByte(OP_EQUAL as libc::c_int as uint8_t);
        }
        15 => {
            emitByte(OP_GREATER as libc::c_int as uint8_t);
        }
        16 => {
            emitBytes(
                OP_LESS as libc::c_int as uint8_t,
                OP_NOT as libc::c_int as uint8_t,
            );
        }
        17 => {
            emitByte(OP_LESS as libc::c_int as uint8_t);
        }
        18 => {
            emitBytes(
                OP_GREATER as libc::c_int as uint8_t,
                OP_NOT as libc::c_int as uint8_t,
            );
        }
        7 => {
            emitByte(OP_ADD as libc::c_int as uint8_t);
        }
        6 => {
            emitByte(OP_SUBTRACT as libc::c_int as uint8_t);
        }
        10 => {
            emitByte(OP_MULTIPLY as libc::c_int as uint8_t);
        }
        9 => {
            emitByte(OP_DIVIDE as libc::c_int as uint8_t);
        }
        _ => return,
    };
}
unsafe extern "C" fn call_compiler(mut canAssign: bool) {
    let mut argCount: uint8_t = argumentList();
    emitBytes(OP_CALL as libc::c_int as uint8_t, argCount);
}
unsafe extern "C" fn dot(mut canAssign: bool) {
    consume(
        TOKEN_IDENTIFIER,
        b"Expect property name after '.'.\0" as *const u8 as *const libc::c_char,
    );
    let mut name: uint8_t = identifierConstant(&mut parser.previous);
    if canAssign as libc::c_int != 0 && match_compiler(TOKEN_EQUAL) as libc::c_int != 0 {
        expression();
        emitBytes(OP_SET_PROPERTY as libc::c_int as uint8_t, name);
    } else if match_compiler(TOKEN_LEFT_PAREN) {
        let mut argCount: uint8_t = argumentList();
        emitBytes(OP_INVOKE as libc::c_int as uint8_t, name);
        emitByte(argCount);
    } else {
        emitBytes(OP_GET_PROPERTY as libc::c_int as uint8_t, name);
    };
}
unsafe extern "C" fn literal(mut canAssign: bool) {
    match parser.previous.type_0 as libc::c_uint {
        25 => {
            emitByte(OP_FALSE as libc::c_int as uint8_t);
        }
        29 => {
            emitByte(OP_NIL as libc::c_int as uint8_t);
        }
        35 => {
            emitByte(OP_TRUE as libc::c_int as uint8_t);
        }
        _ => return,
    };
}
unsafe extern "C" fn grouping(mut canAssign: bool) {
    expression();
    consume(
        TOKEN_RIGHT_PAREN,
        b"Expect ')' after expression.\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn number_compiler(mut canAssign: bool) {
    let mut value: libc::c_double = strtod(
        parser.previous.start,
        0 as *mut *mut libc::c_char,
    );
    emitConstant(numToValue(value));
}
unsafe extern "C" fn or_(mut canAssign: bool) {
    let mut elseJump: libc::c_int = emitJump(OP_JUMP_IF_FALSE as libc::c_int as uint8_t);
    let mut endJump: libc::c_int = emitJump(OP_JUMP as libc::c_int as uint8_t);
    patchJump(elseJump);
    emitByte(OP_POP as libc::c_int as uint8_t);
    parsePrecedence(PREC_OR);
    patchJump(endJump);
}
unsafe extern "C" fn string_compiler(mut canAssign: bool) {
    emitConstant(
        0x8000000000000000 as libc::c_ulong
            | 0x7ffc000000000000 as libc::c_long as uint64_t
            | copyString(
                (parser.previous.start).offset(1 as libc::c_int as isize),
                parser.previous.length - 2 as libc::c_int,
            ) as uintptr_t,
    );
}
unsafe extern "C" fn namedVariable(mut name: Token, mut canAssign: bool) {
    let mut getOp: uint8_t = 0;
    let mut setOp: uint8_t = 0;
    let mut arg: libc::c_int = resolveLocal(current, &mut name);
    if arg != -(1 as libc::c_int) {
        getOp = OP_GET_LOCAL as libc::c_int as uint8_t;
        setOp = OP_SET_LOCAL as libc::c_int as uint8_t;
    } else {
        arg = resolveUpvalue(current, &mut name);
        if arg != -(1 as libc::c_int) {
            getOp = OP_GET_UPVALUE as libc::c_int as uint8_t;
            setOp = OP_SET_UPVALUE as libc::c_int as uint8_t;
        } else {
            arg = identifierConstant(&mut name) as libc::c_int;
            getOp = OP_GET_GLOBAL as libc::c_int as uint8_t;
            setOp = OP_SET_GLOBAL as libc::c_int as uint8_t;
        }
    }
    if canAssign as libc::c_int != 0 && match_compiler(TOKEN_EQUAL) as libc::c_int != 0 {
        expression();
        emitBytes(setOp, arg as uint8_t);
    } else {
        emitBytes(getOp, arg as uint8_t);
    };
}
unsafe extern "C" fn variable(mut canAssign: bool) {
    namedVariable(parser.previous, canAssign);
}
unsafe extern "C" fn syntheticToken(mut text: *const libc::c_char) -> Token {
    let mut token: Token = Token {
        type_0: TOKEN_LEFT_PAREN,
        start: 0 as *const libc::c_char,
        length: 0,
        line: 0,
    };
    token.start = text;
    token.length = strlen(text) as libc::c_int;
    return token;
}
unsafe extern "C" fn super_(mut canAssign: bool) {
    if currentClass.is_null() {
        error(
            b"Can't use 'super' outside of a class.\0" as *const u8
                as *const libc::c_char,
        );
    } else if !(*currentClass).hasSuperclass {
        error(
            b"Can't use 'super' in a class with no superclass.\0" as *const u8
                as *const libc::c_char,
        );
    }
    consume(
        TOKEN_DOT,
        b"Expect '.' after 'super'.\0" as *const u8 as *const libc::c_char,
    );
    consume(
        TOKEN_IDENTIFIER,
        b"Expect superclass method name.\0" as *const u8 as *const libc::c_char,
    );
    let mut name: uint8_t = identifierConstant(&mut parser.previous);
    namedVariable(
        syntheticToken(b"this\0" as *const u8 as *const libc::c_char),
        0 as libc::c_int != 0,
    );
    if match_compiler(TOKEN_LEFT_PAREN) {
        let mut argCount: uint8_t = argumentList();
        namedVariable(
            syntheticToken(b"super\0" as *const u8 as *const libc::c_char),
            0 as libc::c_int != 0,
        );
        emitBytes(OP_SUPER_INVOKE as libc::c_int as uint8_t, name);
        emitByte(argCount);
    } else {
        namedVariable(
            syntheticToken(b"super\0" as *const u8 as *const libc::c_char),
            0 as libc::c_int != 0,
        );
        emitBytes(OP_GET_SUPER as libc::c_int as uint8_t, name);
    };
}
unsafe extern "C" fn this_(mut canAssign: bool) {
    if currentClass.is_null() {
        error(
            b"Can't use 'this' outside of a class.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    variable(0 as libc::c_int != 0);
}
unsafe extern "C" fn unary(mut canAssign: bool) {
    let mut operatorType: TokenType = parser.previous.type_0;
    parsePrecedence(PREC_UNARY);
    match operatorType as libc::c_uint {
        11 => {
            emitByte(OP_NOT as libc::c_int as uint8_t);
        }
        6 => {
            emitByte(OP_NEGATE as libc::c_int as uint8_t);
        }
        _ => return,
    };
}
#[no_mangle]
pub static mut rules: [ParseRule; 40] = unsafe {
    [
        {
            let mut init = ParseRule {
                prefix: Some(grouping as unsafe extern "C" fn(bool) -> ()),
                infix: Some(call_compiler as unsafe extern "C" fn(bool) -> ()),
                precedence: PREC_CALL,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: Some(dot as unsafe extern "C" fn(bool) -> ()),
                precedence: PREC_CALL,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: Some(unary as unsafe extern "C" fn(bool) -> ()),
                infix: Some(binary as unsafe extern "C" fn(bool) -> ()),
                precedence: PREC_TERM,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: Some(binary as unsafe extern "C" fn(bool) -> ()),
                precedence: PREC_TERM,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: Some(binary as unsafe extern "C" fn(bool) -> ()),
                precedence: PREC_FACTOR,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: Some(binary as unsafe extern "C" fn(bool) -> ()),
                precedence: PREC_FACTOR,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: Some(unary as unsafe extern "C" fn(bool) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: Some(binary as unsafe extern "C" fn(bool) -> ()),
                precedence: PREC_EQUALITY,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: Some(binary as unsafe extern "C" fn(bool) -> ()),
                precedence: PREC_EQUALITY,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: Some(binary as unsafe extern "C" fn(bool) -> ()),
                precedence: PREC_COMPARISON,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: Some(binary as unsafe extern "C" fn(bool) -> ()),
                precedence: PREC_COMPARISON,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: Some(binary as unsafe extern "C" fn(bool) -> ()),
                precedence: PREC_COMPARISON,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: Some(binary as unsafe extern "C" fn(bool) -> ()),
                precedence: PREC_COMPARISON,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: Some(variable as unsafe extern "C" fn(bool) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: Some(string_compiler as unsafe extern "C" fn(bool) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: Some(number_compiler as unsafe extern "C" fn(bool) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: Some(and_ as unsafe extern "C" fn(bool) -> ()),
                precedence: PREC_AND,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: Some(literal as unsafe extern "C" fn(bool) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: Some(literal as unsafe extern "C" fn(bool) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: Some(or_ as unsafe extern "C" fn(bool) -> ()),
                precedence: PREC_OR,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: Some(super_ as unsafe extern "C" fn(bool) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: Some(this_ as unsafe extern "C" fn(bool) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: Some(literal as unsafe extern "C" fn(bool) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = ParseRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
    ]
};
unsafe extern "C" fn parsePrecedence(mut precedence: Precedence) {
    advance_compiler();
    let mut prefixRule: ParseFn = (*getRule(parser.previous.type_0)).prefix;
    if prefixRule.is_none() {
        error(b"Expect expression.\0" as *const u8 as *const libc::c_char);
        return;
    }
    let mut canAssign: bool = precedence as libc::c_uint
        <= PREC_ASSIGNMENT as libc::c_int as libc::c_uint;
    prefixRule.expect("non-null function pointer")(canAssign);
    while precedence as libc::c_uint
        <= (*getRule(parser.current.type_0)).precedence as libc::c_uint
    {
        advance_compiler();
        let mut infixRule: ParseFn = (*getRule(parser.previous.type_0)).infix;
        infixRule.expect("non-null function pointer")(canAssign);
    }
    if canAssign as libc::c_int != 0 && match_compiler(TOKEN_EQUAL) as libc::c_int != 0 {
        error(b"Invalid assignment target.\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn getRule(mut type_0: TokenType) -> *mut ParseRule {
    return &mut *rules.as_mut_ptr().offset(type_0 as isize) as *mut ParseRule;
}
unsafe extern "C" fn expression() {
    parsePrecedence(PREC_ASSIGNMENT);
}
unsafe extern "C" fn block() {
    while !check(TOKEN_RIGHT_BRACE) && !check(TOKEN_EOF) {
        declaration();
    }
    consume(
        TOKEN_RIGHT_BRACE,
        b"Expect '}' after block.\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn function(mut type_0: FunctionType) {
    let mut compiler: Compiler = Compiler {
        enclosing: 0 as *mut Compiler,
        function: 0 as *mut ObjFunction,
        type_0: TYPE_FUNCTION,
        locals: [Local {
            name: Token {
                type_0: TOKEN_LEFT_PAREN,
                start: 0 as *const libc::c_char,
                length: 0,
                line: 0,
            },
            depth: 0,
            isCaptured: false,
        }; 256],
        localCount: 0,
        upvalues: [Upvalue {
            index: 0,
            isLocal: false,
        }; 256],
        scopeDepth: 0,
    };
    initCompiler(&mut compiler, type_0);
    beginScope();
    consume(
        TOKEN_LEFT_PAREN,
        b"Expect '(' after function name.\0" as *const u8 as *const libc::c_char,
    );
    if !check(TOKEN_RIGHT_PAREN) {
        loop {
            (*(*current).function).arity += 1;
            (*(*current).function).arity;
            if (*(*current).function).arity > 255 as libc::c_int {
                errorAtCurrent(
                    b"Can't have more than 255 parameters.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            let mut constant: uint8_t = parseVariable(
                b"Expect parameter name.\0" as *const u8 as *const libc::c_char,
            );
            defineVariable(constant);
            if !match_compiler(TOKEN_COMMA) {
                break;
            }
        }
    }
    consume(
        TOKEN_RIGHT_PAREN,
        b"Expect ')' after parameters.\0" as *const u8 as *const libc::c_char,
    );
    consume(
        TOKEN_LEFT_BRACE,
        b"Expect '{' before function body.\0" as *const u8 as *const libc::c_char,
    );
    block();
    let mut function_0: *mut ObjFunction = endCompiler();
    emitBytes(
        OP_CLOSURE as libc::c_int as uint8_t,
        makeConstant(
            0x8000000000000000 as libc::c_ulong
                | 0x7ffc000000000000 as libc::c_long as uint64_t
                | function_0 as uintptr_t,
        ),
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*function_0).upvalueCount {
        emitByte(
            (if compiler.upvalues[i as usize].isLocal as libc::c_int != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as uint8_t,
        );
        emitByte(compiler.upvalues[i as usize].index);
        i += 1;
        i;
    }
}
unsafe extern "C" fn method() {
    consume(
        TOKEN_IDENTIFIER,
        b"Expect method name.\0" as *const u8 as *const libc::c_char,
    );
    let mut constant: uint8_t = identifierConstant(&mut parser.previous);
    let mut type_0: FunctionType = TYPE_METHOD;
    if parser.previous.length == 4 as libc::c_int
        && memcmp(
            parser.previous.start as *const libc::c_void,
            b"init\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        type_0 = TYPE_INITIALIZER;
    }
    function(type_0);
    emitBytes(OP_METHOD as libc::c_int as uint8_t, constant);
}
unsafe extern "C" fn classDeclaration() {
    consume(
        TOKEN_IDENTIFIER,
        b"Expect class name.\0" as *const u8 as *const libc::c_char,
    );
    let mut className: Token = parser.previous;
    let mut nameConstant: uint8_t = identifierConstant(&mut parser.previous);
    declareVariable();
    emitBytes(OP_CLASS as libc::c_int as uint8_t, nameConstant);
    defineVariable(nameConstant);
    let mut classCompiler: ClassCompiler = ClassCompiler {
        enclosing: 0 as *mut ClassCompiler,
        hasSuperclass: false,
    };
    classCompiler.hasSuperclass = 0 as libc::c_int != 0;
    classCompiler.enclosing = currentClass;
    currentClass = &mut classCompiler;
    if match_compiler(TOKEN_LESS) {
        consume(
            TOKEN_IDENTIFIER,
            b"Expect superclass name.\0" as *const u8 as *const libc::c_char,
        );
        variable(0 as libc::c_int != 0);
        if identifiersEqual(&mut className, &mut parser.previous) {
            error(
                b"A class can't inherit from itself.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        beginScope();
        addLocal(syntheticToken(b"super\0" as *const u8 as *const libc::c_char));
        defineVariable(0 as libc::c_int as uint8_t);
        namedVariable(className, 0 as libc::c_int != 0);
        emitByte(OP_INHERIT as libc::c_int as uint8_t);
        classCompiler.hasSuperclass = 1 as libc::c_int != 0;
    }
    namedVariable(className, 0 as libc::c_int != 0);
    consume(
        TOKEN_LEFT_BRACE,
        b"Expect '{' before class body.\0" as *const u8 as *const libc::c_char,
    );
    while !check(TOKEN_RIGHT_BRACE) && !check(TOKEN_EOF) {
        method();
    }
    consume(
        TOKEN_RIGHT_BRACE,
        b"Expect '}' after class body.\0" as *const u8 as *const libc::c_char,
    );
    emitByte(OP_POP as libc::c_int as uint8_t);
    if classCompiler.hasSuperclass {
        endScope();
    }
    currentClass = (*currentClass).enclosing;
}
unsafe extern "C" fn funDeclaration() {
    let mut global: uint8_t = parseVariable(
        b"Expect function name.\0" as *const u8 as *const libc::c_char,
    );
    markInitialized();
    function(TYPE_FUNCTION);
    defineVariable(global);
}
unsafe extern "C" fn varDeclaration() {
    let mut global: uint8_t = parseVariable(
        b"Expect variable name.\0" as *const u8 as *const libc::c_char,
    );
    if match_compiler(TOKEN_EQUAL) {
        expression();
    } else {
        emitByte(OP_NIL as libc::c_int as uint8_t);
    }
    consume(
        TOKEN_SEMICOLON,
        b"Expect ';' after variable declaration.\0" as *const u8 as *const libc::c_char,
    );
    defineVariable(global);
}
unsafe extern "C" fn expressionStatement() {
    expression();
    consume(
        TOKEN_SEMICOLON,
        b"Expect ';' after expression.\0" as *const u8 as *const libc::c_char,
    );
    emitByte(OP_POP as libc::c_int as uint8_t);
}
unsafe extern "C" fn forStatement() {
    beginScope();
    consume(
        TOKEN_LEFT_PAREN,
        b"Expect '(' after 'for'.\0" as *const u8 as *const libc::c_char,
    );
    if !match_compiler(TOKEN_SEMICOLON) {
        if match_compiler(TOKEN_VAR) {
            varDeclaration();
        } else {
            expressionStatement();
        }
    }
    let mut loopStart: libc::c_int = (*currentChunk()).count;
    let mut exitJump: libc::c_int = -(1 as libc::c_int);
    if !match_compiler(TOKEN_SEMICOLON) {
        expression();
        consume(
            TOKEN_SEMICOLON,
            b"Expect ';' after loop condition.\0" as *const u8 as *const libc::c_char,
        );
        exitJump = emitJump(OP_JUMP_IF_FALSE as libc::c_int as uint8_t);
        emitByte(OP_POP as libc::c_int as uint8_t);
    }
    if !match_compiler(TOKEN_RIGHT_PAREN) {
        let mut bodyJump: libc::c_int = emitJump(OP_JUMP as libc::c_int as uint8_t);
        let mut incrementStart: libc::c_int = (*currentChunk()).count;
        expression();
        emitByte(OP_POP as libc::c_int as uint8_t);
        consume(
            TOKEN_RIGHT_PAREN,
            b"Expect ')' after for clauses.\0" as *const u8 as *const libc::c_char,
        );
        emitLoop(loopStart);
        loopStart = incrementStart;
        patchJump(bodyJump);
    }
    statement();
    emitLoop(loopStart);
    if exitJump != -(1 as libc::c_int) {
        patchJump(exitJump);
        emitByte(OP_POP as libc::c_int as uint8_t);
    }
    endScope();
}
unsafe extern "C" fn ifStatement() {
    consume(
        TOKEN_LEFT_PAREN,
        b"Expect '(' after 'if'.\0" as *const u8 as *const libc::c_char,
    );
    expression();
    consume(
        TOKEN_RIGHT_PAREN,
        b"Expect ')' after condition.\0" as *const u8 as *const libc::c_char,
    );
    let mut thenJump: libc::c_int = emitJump(OP_JUMP_IF_FALSE as libc::c_int as uint8_t);
    emitByte(OP_POP as libc::c_int as uint8_t);
    statement();
    let mut elseJump: libc::c_int = emitJump(OP_JUMP as libc::c_int as uint8_t);
    patchJump(thenJump);
    emitByte(OP_POP as libc::c_int as uint8_t);
    if match_compiler(TOKEN_ELSE) {
        statement();
    }
    patchJump(elseJump);
}
unsafe extern "C" fn printStatement() {
    expression();
    consume(
        TOKEN_SEMICOLON,
        b"Expect ';' after value.\0" as *const u8 as *const libc::c_char,
    );
    emitByte(OP_PRINT as libc::c_int as uint8_t);
}
unsafe extern "C" fn returnStatement() {
    if (*current).type_0 as libc::c_uint == TYPE_SCRIPT as libc::c_int as libc::c_uint {
        error(
            b"Can't return from top-level code.\0" as *const u8 as *const libc::c_char,
        );
    }
    if match_compiler(TOKEN_SEMICOLON) {
        emitReturn();
    } else {
        if (*current).type_0 as libc::c_uint
            == TYPE_INITIALIZER as libc::c_int as libc::c_uint
        {
            error(
                b"Can't return a value from an initializer.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        expression();
        consume(
            TOKEN_SEMICOLON,
            b"Expect ';' after return value.\0" as *const u8 as *const libc::c_char,
        );
        emitByte(OP_RETURN as libc::c_int as uint8_t);
    };
}
unsafe extern "C" fn whileStatement() {
    let mut loopStart: libc::c_int = (*currentChunk()).count;
    consume(
        TOKEN_LEFT_PAREN,
        b"Expect '(' after 'while'.\0" as *const u8 as *const libc::c_char,
    );
    expression();
    consume(
        TOKEN_RIGHT_PAREN,
        b"Expect ')' after condition.\0" as *const u8 as *const libc::c_char,
    );
    let mut exitJump: libc::c_int = emitJump(OP_JUMP_IF_FALSE as libc::c_int as uint8_t);
    emitByte(OP_POP as libc::c_int as uint8_t);
    statement();
    emitLoop(loopStart);
    patchJump(exitJump);
    emitByte(OP_POP as libc::c_int as uint8_t);
}
unsafe extern "C" fn synchronize() {
    parser.panicMode = 0 as libc::c_int != 0;
    while parser.current.type_0 as libc::c_uint
        != TOKEN_EOF as libc::c_int as libc::c_uint
    {
        if parser.previous.type_0 as libc::c_uint
            == TOKEN_SEMICOLON as libc::c_int as libc::c_uint
        {
            return;
        }
        match parser.current.type_0 as libc::c_uint {
            23 | 27 | 36 | 26 | 28 | 37 | 31 | 32 => return,
            _ => {}
        }
        advance_compiler();
    }
}
unsafe extern "C" fn declaration() {
    if match_compiler(TOKEN_CLASS) {
        classDeclaration();
    } else if match_compiler(TOKEN_FUN) {
        funDeclaration();
    } else if match_compiler(TOKEN_VAR) {
        varDeclaration();
    } else {
        statement();
    }
    if parser.panicMode {
        synchronize();
    }
}
unsafe extern "C" fn statement() {
    if match_compiler(TOKEN_PRINT) {
        printStatement();
    } else if match_compiler(TOKEN_FOR) {
        forStatement();
    } else if match_compiler(TOKEN_IF) {
        ifStatement();
    } else if match_compiler(TOKEN_RETURN) {
        returnStatement();
    } else if match_compiler(TOKEN_WHILE) {
        whileStatement();
    } else if match_compiler(TOKEN_LEFT_BRACE) {
        beginScope();
        block();
        endScope();
    } else {
        expressionStatement();
    };
}
#[no_mangle]
pub unsafe extern "C" fn compile(mut source: *const libc::c_char) -> *mut ObjFunction {
    initScanner(source);
    let mut compiler: Compiler = Compiler {
        enclosing: 0 as *mut Compiler,
        function: 0 as *mut ObjFunction,
        type_0: TYPE_FUNCTION,
        locals: [Local {
            name: Token {
                type_0: TOKEN_LEFT_PAREN,
                start: 0 as *const libc::c_char,
                length: 0,
                line: 0,
            },
            depth: 0,
            isCaptured: false,
        }; 256],
        localCount: 0,
        upvalues: [Upvalue {
            index: 0,
            isLocal: false,
        }; 256],
        scopeDepth: 0,
    };
    initCompiler(&mut compiler, TYPE_SCRIPT);
    parser.hadError = 0 as libc::c_int != 0;
    parser.panicMode = 0 as libc::c_int != 0;
    advance_compiler();
    while !match_compiler(TOKEN_EOF) {
        declaration();
    }
    let mut function_0: *mut ObjFunction = endCompiler();
    return if parser.hadError as libc::c_int != 0 {
        0 as *mut ObjFunction
    } else {
        function_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn markCompilerRoots() {
    let mut compiler: *mut Compiler = current;
    while !compiler.is_null() {
        markObject((*compiler).function as *mut Obj);
        compiler = (*compiler).enclosing;
    }
}
#[no_mangle]
pub unsafe extern "C" fn disassembleChunk(
    mut chunk: *mut Chunk,
    mut name: *const libc::c_char,
) {
    printf(b"== %s ==\n\0" as *const u8 as *const libc::c_char, name);
    let mut offset: libc::c_int = 0 as libc::c_int;
    while offset < (*chunk).count {
        offset = disassembleInstruction(chunk, offset);
    }
}
unsafe extern "C" fn constantInstruction(
    mut name: *const libc::c_char,
    mut chunk: *mut Chunk,
    mut offset: libc::c_int,
) -> libc::c_int {
    let mut constant: uint8_t = *((*chunk).code)
        .offset((offset + 1 as libc::c_int) as isize);
    printf(
        b"%-16s %4d '\0" as *const u8 as *const libc::c_char,
        name,
        constant as libc::c_int,
    );
    printValue(*((*chunk).constants.values).offset(constant as isize));
    printf(b"'\n\0" as *const u8 as *const libc::c_char);
    return offset + 2 as libc::c_int;
}
unsafe extern "C" fn invokeInstruction(
    mut name: *const libc::c_char,
    mut chunk: *mut Chunk,
    mut offset: libc::c_int,
) -> libc::c_int {
    let mut constant: uint8_t = *((*chunk).code)
        .offset((offset + 1 as libc::c_int) as isize);
    let mut argCount: uint8_t = *((*chunk).code)
        .offset((offset + 2 as libc::c_int) as isize);
    printf(
        b"%-16s (%d args) %4d '\0" as *const u8 as *const libc::c_char,
        name,
        argCount as libc::c_int,
        constant as libc::c_int,
    );
    printValue(*((*chunk).constants.values).offset(constant as isize));
    printf(b"'\n\0" as *const u8 as *const libc::c_char);
    return offset + 3 as libc::c_int;
}
unsafe extern "C" fn simpleInstruction(
    mut name: *const libc::c_char,
    mut offset: libc::c_int,
) -> libc::c_int {
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, name);
    return offset + 1 as libc::c_int;
}
unsafe extern "C" fn byteInstruction(
    mut name: *const libc::c_char,
    mut chunk: *mut Chunk,
    mut offset: libc::c_int,
) -> libc::c_int {
    let mut slot: uint8_t = *((*chunk).code)
        .offset((offset + 1 as libc::c_int) as isize);
    printf(
        b"%-16s %4d\n\0" as *const u8 as *const libc::c_char,
        name,
        slot as libc::c_int,
    );
    return offset + 2 as libc::c_int;
}
unsafe extern "C" fn jumpInstruction(
    mut name: *const libc::c_char,
    mut sign: libc::c_int,
    mut chunk: *mut Chunk,
    mut offset: libc::c_int,
) -> libc::c_int {
    let mut jump: uint16_t = ((*((*chunk).code)
        .offset((offset + 1 as libc::c_int) as isize) as libc::c_int)
        << 8 as libc::c_int) as uint16_t;
    jump = (jump as libc::c_int
        | *((*chunk).code).offset((offset + 2 as libc::c_int) as isize) as libc::c_int)
        as uint16_t;
    printf(
        b"%-16s %4d -> %d\n\0" as *const u8 as *const libc::c_char,
        name,
        offset,
        offset + 3 as libc::c_int + sign * jump as libc::c_int,
    );
    return offset + 3 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn disassembleInstruction(
    mut chunk: *mut Chunk,
    mut offset: libc::c_int,
) -> libc::c_int {
    printf(b"%04d \0" as *const u8 as *const libc::c_char, offset);
    if offset > 0 as libc::c_int
        && *((*chunk).lines).offset(offset as isize)
            == *((*chunk).lines).offset((offset - 1 as libc::c_int) as isize)
    {
        printf(b"   | \0" as *const u8 as *const libc::c_char);
    } else {
        printf(
            b"%4d \0" as *const u8 as *const libc::c_char,
            *((*chunk).lines).offset(offset as isize),
        );
    }
    let mut instruction: uint8_t = *((*chunk).code).offset(offset as isize);
    match instruction as libc::c_int {
        0 => {
            return constantInstruction(
                b"OP_CONSTANT\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        1 => {
            return simpleInstruction(
                b"OP_NIL\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        2 => {
            return simpleInstruction(
                b"OP_TRUE\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        3 => {
            return simpleInstruction(
                b"OP_FALSE\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        4 => {
            return simpleInstruction(
                b"OP_POP\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        5 => {
            return byteInstruction(
                b"OP_GET_LOCAL\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        6 => {
            return byteInstruction(
                b"OP_SET_LOCAL\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        7 => {
            return constantInstruction(
                b"OP_GET_GLOBAL\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        8 => {
            return constantInstruction(
                b"OP_DEFINE_GLOBAL\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        9 => {
            return constantInstruction(
                b"OP_SET_GLOBAL\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        10 => {
            return byteInstruction(
                b"OP_GET_UPVALUE\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        11 => {
            return byteInstruction(
                b"OP_SET_UPVALUE\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        12 => {
            return constantInstruction(
                b"OP_GET_PROPERTY\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        13 => {
            return constantInstruction(
                b"OP_SET_PROPERTY\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        14 => {
            return constantInstruction(
                b"OP_GET_SUPER\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        15 => {
            return simpleInstruction(
                b"OP_EQUAL\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        16 => {
            return simpleInstruction(
                b"OP_GREATER\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        17 => {
            return simpleInstruction(
                b"OP_LESS\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        18 => {
            return simpleInstruction(
                b"OP_ADD\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        19 => {
            return simpleInstruction(
                b"OP_SUBTRACT\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        20 => {
            return simpleInstruction(
                b"OP_MULTIPLY\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        21 => {
            return simpleInstruction(
                b"OP_DIVIDE\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        22 => {
            return simpleInstruction(
                b"OP_NOT\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        23 => {
            return simpleInstruction(
                b"OP_NEGATE\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        24 => {
            return simpleInstruction(
                b"OP_PRINT\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        25 => {
            return jumpInstruction(
                b"OP_JUMP\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                chunk,
                offset,
            );
        }
        26 => {
            return jumpInstruction(
                b"OP_JUMP_IF_FALSE\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                chunk,
                offset,
            );
        }
        27 => {
            return jumpInstruction(
                b"OP_LOOP\0" as *const u8 as *const libc::c_char,
                -(1 as libc::c_int),
                chunk,
                offset,
            );
        }
        28 => {
            return byteInstruction(
                b"OP_CALL\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        29 => {
            return invokeInstruction(
                b"OP_INVOKE\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        30 => {
            return invokeInstruction(
                b"OP_SUPER_INVOKE\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        31 => {
            offset += 1;
            offset;
            let fresh3 = offset;
            offset = offset + 1;
            let mut constant: uint8_t = *((*chunk).code).offset(fresh3 as isize);
            printf(
                b"%-16s %4d \0" as *const u8 as *const libc::c_char,
                b"OP_CLOSURE\0" as *const u8 as *const libc::c_char,
                constant as libc::c_int,
            );
            printValue(*((*chunk).constants.values).offset(constant as isize));
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            let mut function_0: *mut ObjFunction = (*((*chunk).constants.values)
                .offset(constant as isize)
                & !(0x8000000000000000 as libc::c_ulong
                    | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                as *mut ObjFunction;
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < (*function_0).upvalueCount {
                let fresh4 = offset;
                offset = offset + 1;
                let mut isLocal: libc::c_int = *((*chunk).code).offset(fresh4 as isize)
                    as libc::c_int;
                let fresh5 = offset;
                offset = offset + 1;
                let mut index: libc::c_int = *((*chunk).code).offset(fresh5 as isize)
                    as libc::c_int;
                printf(
                    b"%04d      |                     %s %d\n\0" as *const u8
                        as *const libc::c_char,
                    offset - 2 as libc::c_int,
                    if isLocal != 0 {
                        b"local\0" as *const u8 as *const libc::c_char
                    } else {
                        b"upvalue\0" as *const u8 as *const libc::c_char
                    },
                    index,
                );
                j += 1;
                j;
            }
            return offset;
        }
        32 => {
            return simpleInstruction(
                b"OP_CLOSE_UPVALUE\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        33 => {
            return simpleInstruction(
                b"OP_RETURN\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        34 => {
            return constantInstruction(
                b"OP_CLASS\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        35 => {
            return simpleInstruction(
                b"OP_INHERIT\0" as *const u8 as *const libc::c_char,
                offset,
            );
        }
        36 => {
            return constantInstruction(
                b"OP_METHOD\0" as *const u8 as *const libc::c_char,
                chunk,
                offset,
            );
        }
        _ => {
            printf(
                b"Unknown opcode %d\n\0" as *const u8 as *const libc::c_char,
                instruction as libc::c_int,
            );
            return offset + 1 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn reallocate(
    mut pointer: *mut libc::c_void,
    mut oldSize: size_t,
    mut newSize: size_t,
) -> *mut libc::c_void {
    vm
        .bytesAllocated = (vm.bytesAllocated as libc::c_ulong)
        .wrapping_add(newSize.wrapping_sub(oldSize)) as size_t as size_t;
    if newSize > oldSize {
        if vm.bytesAllocated > vm.nextGC {
            collectGarbage();
        }
    }
    if newSize == 0 as libc::c_int as libc::c_ulong {
        free(pointer);
        return 0 as *mut libc::c_void;
    }
    let mut result: *mut libc::c_void = realloc(pointer, newSize);
    if result.is_null() {
        exit(1 as libc::c_int);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn markObject(mut object: *mut Obj) {
    if object.is_null() {
        return;
    }
    if (*object).isMarked {
        return;
    }
    (*object).isMarked = 1 as libc::c_int != 0;
    if vm.grayCapacity < vm.grayCount + 1 as libc::c_int {
        vm
            .grayCapacity = if vm.grayCapacity < 8 as libc::c_int {
            8 as libc::c_int
        } else {
            vm.grayCapacity * 2 as libc::c_int
        };
        vm
            .grayStack = realloc(
            vm.grayStack as *mut libc::c_void,
            (::core::mem::size_of::<*mut Obj>() as libc::c_ulong)
                .wrapping_mul(vm.grayCapacity as libc::c_ulong),
        ) as *mut *mut Obj;
        if (vm.grayStack).is_null() {
            exit(1 as libc::c_int);
        }
    }
    let fresh6 = vm.grayCount;
    vm.grayCount = vm.grayCount + 1;
    let ref mut fresh7 = *(vm.grayStack).offset(fresh6 as isize);
    *fresh7 = object;
}
#[no_mangle]
pub unsafe extern "C" fn markValue(mut value: Value) {
    if value
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
    {
        markObject(
            (value
                & !(0x8000000000000000 as libc::c_ulong
                    | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj,
        );
    }
}
unsafe extern "C" fn markArray(mut array: *mut ValueArray) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*array).count {
        markValue(*((*array).values).offset(i as isize));
        i += 1;
        i;
    }
}
unsafe extern "C" fn blackenObject(mut object: *mut Obj) {
    match (*object).type_0 as libc::c_uint {
        0 => {
            let mut bound: *mut ObjBoundMethod = object as *mut ObjBoundMethod;
            markValue((*bound).receiver);
            markObject((*bound).method as *mut Obj);
        }
        1 => {
            let mut klass: *mut ObjClass = object as *mut ObjClass;
            markObject((*klass).name as *mut Obj);
            markTable(&mut (*klass).methods);
        }
        2 => {
            let mut closure: *mut ObjClosure = object as *mut ObjClosure;
            markObject((*closure).function as *mut Obj);
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < (*closure).upvalueCount {
                markObject(*((*closure).upvalues).offset(i as isize) as *mut Obj);
                i += 1;
                i;
            }
        }
        3 => {
            let mut function_0: *mut ObjFunction = object as *mut ObjFunction;
            markObject((*function_0).name as *mut Obj);
            markArray(&mut (*function_0).chunk.constants);
        }
        4 => {
            let mut instance: *mut ObjInstance = object as *mut ObjInstance;
            markObject((*instance).klass as *mut Obj);
            markTable(&mut (*instance).fields);
        }
        7 => {
            markValue((*(object as *mut ObjUpvalue)).closed);
        }
        5 | 6 | _ => {}
    };
}
unsafe extern "C" fn freeObject(mut object: *mut Obj) {
    match (*object).type_0 as libc::c_uint {
        0 => {
            reallocate(
                object as *mut libc::c_void,
                ::core::mem::size_of::<ObjBoundMethod>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
        }
        1 => {
            let mut klass: *mut ObjClass = object as *mut ObjClass;
            freeTable(&mut (*klass).methods);
            reallocate(
                object as *mut libc::c_void,
                ::core::mem::size_of::<ObjClass>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
        }
        2 => {
            let mut closure: *mut ObjClosure = object as *mut ObjClosure;
            reallocate(
                (*closure).upvalues as *mut libc::c_void,
                (::core::mem::size_of::<*mut ObjUpvalue>() as libc::c_ulong)
                    .wrapping_mul((*closure).upvalueCount as libc::c_ulong),
                0 as libc::c_int as size_t,
            );
            reallocate(
                object as *mut libc::c_void,
                ::core::mem::size_of::<ObjClosure>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
        }
        3 => {
            let mut function_0: *mut ObjFunction = object as *mut ObjFunction;
            freeChunk(&mut (*function_0).chunk);
            reallocate(
                object as *mut libc::c_void,
                ::core::mem::size_of::<ObjFunction>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
        }
        4 => {
            let mut instance: *mut ObjInstance = object as *mut ObjInstance;
            freeTable(&mut (*instance).fields);
            reallocate(
                object as *mut libc::c_void,
                ::core::mem::size_of::<ObjInstance>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
        }
        5 => {
            reallocate(
                object as *mut libc::c_void,
                ::core::mem::size_of::<ObjNative>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
        }
        6 => {
            let mut string: *mut ObjString = object as *mut ObjString;
            reallocate(
                (*string).chars as *mut libc::c_void,
                (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(
                        ((*string).length + 1 as libc::c_int) as libc::c_ulong,
                    ),
                0 as libc::c_int as size_t,
            );
            reallocate(
                object as *mut libc::c_void,
                ::core::mem::size_of::<ObjString>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
        }
        7 => {
            reallocate(
                object as *mut libc::c_void,
                ::core::mem::size_of::<ObjUpvalue>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn markRoots() {
    let mut slot: *mut Value = (vm.stack).as_mut_ptr();
    while slot < vm.stackTop {
        markValue(*slot);
        slot = slot.offset(1);
        slot;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < vm.frameCount {
        markObject(vm.frames[i as usize].closure as *mut Obj);
        i += 1;
        i;
    }
    let mut upvalue: *mut ObjUpvalue = vm.openUpvalues;
    while !upvalue.is_null() {
        markObject(upvalue as *mut Obj);
        upvalue = (*upvalue).next;
    }
    markTable(&mut vm.globals);
    markCompilerRoots();
    markObject(vm.initString as *mut Obj);
}
unsafe extern "C" fn traceReferences() {
    while vm.grayCount > 0 as libc::c_int {
        vm.grayCount -= 1;
        let mut object: *mut Obj = *(vm.grayStack).offset(vm.grayCount as isize);
        blackenObject(object);
    }
}
unsafe extern "C" fn sweep() {
    let mut previous: *mut Obj = 0 as *mut Obj;
    let mut object: *mut Obj = vm.objects;
    while !object.is_null() {
        if (*object).isMarked {
            (*object).isMarked = 0 as libc::c_int != 0;
            previous = object;
            object = (*object).next;
        } else {
            let mut unreached: *mut Obj = object;
            object = (*object).next;
            if !previous.is_null() {
                (*previous).next = object;
            } else {
                vm.objects = object;
            }
            freeObject(unreached);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn collectGarbage() {
    markRoots();
    traceReferences();
    tableRemoveWhite(&mut vm.strings);
    sweep();
    vm.nextGC = (vm.bytesAllocated).wrapping_mul(2 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn freeObjects() {
    let mut object: *mut Obj = vm.objects;
    while !object.is_null() {
        let mut next: *mut Obj = (*object).next;
        freeObject(object);
        object = next;
    }
    free(vm.grayStack as *mut libc::c_void);
}
unsafe extern "C" fn allocateObject(mut size: size_t, mut type_0: ObjType) -> *mut Obj {
    let mut object: *mut Obj = reallocate(
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        size,
    ) as *mut Obj;
    (*object).type_0 = type_0;
    (*object).isMarked = 0 as libc::c_int != 0;
    (*object).next = vm.objects;
    vm.objects = object;
    return object;
}
#[no_mangle]
pub unsafe extern "C" fn newBoundMethod(
    mut receiver: Value,
    mut method_0: *mut ObjClosure,
) -> *mut ObjBoundMethod {
    let mut bound: *mut ObjBoundMethod = allocateObject(
        ::core::mem::size_of::<ObjBoundMethod>() as libc::c_ulong,
        OBJ_BOUND_METHOD,
    ) as *mut ObjBoundMethod;
    (*bound).receiver = receiver;
    (*bound).method = method_0;
    return bound;
}
#[no_mangle]
pub unsafe extern "C" fn newClass(mut name: *mut ObjString) -> *mut ObjClass {
    let mut klass: *mut ObjClass = allocateObject(
        ::core::mem::size_of::<ObjClass>() as libc::c_ulong,
        OBJ_CLASS,
    ) as *mut ObjClass;
    (*klass).name = name;
    initTable(&mut (*klass).methods);
    return klass;
}
#[no_mangle]
pub unsafe extern "C" fn newClosure(
    mut function_0: *mut ObjFunction,
) -> *mut ObjClosure {
    let mut upvalues: *mut *mut ObjUpvalue = reallocate(
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        (::core::mem::size_of::<*mut ObjUpvalue>() as libc::c_ulong)
            .wrapping_mul((*function_0).upvalueCount as libc::c_ulong),
    ) as *mut *mut ObjUpvalue;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*function_0).upvalueCount {
        let ref mut fresh8 = *upvalues.offset(i as isize);
        *fresh8 = 0 as *mut ObjUpvalue;
        i += 1;
        i;
    }
    let mut closure: *mut ObjClosure = allocateObject(
        ::core::mem::size_of::<ObjClosure>() as libc::c_ulong,
        OBJ_CLOSURE,
    ) as *mut ObjClosure;
    (*closure).function = function_0;
    (*closure).upvalues = upvalues;
    (*closure).upvalueCount = (*function_0).upvalueCount;
    return closure;
}
#[no_mangle]
pub unsafe extern "C" fn newFunction() -> *mut ObjFunction {
    let mut function_0: *mut ObjFunction = allocateObject(
        ::core::mem::size_of::<ObjFunction>() as libc::c_ulong,
        OBJ_FUNCTION,
    ) as *mut ObjFunction;
    (*function_0).arity = 0 as libc::c_int;
    (*function_0).upvalueCount = 0 as libc::c_int;
    (*function_0).name = 0 as *mut ObjString;
    initChunk(&mut (*function_0).chunk);
    return function_0;
}
#[no_mangle]
pub unsafe extern "C" fn newInstance(mut klass: *mut ObjClass) -> *mut ObjInstance {
    let mut instance: *mut ObjInstance = allocateObject(
        ::core::mem::size_of::<ObjInstance>() as libc::c_ulong,
        OBJ_INSTANCE,
    ) as *mut ObjInstance;
    (*instance).klass = klass;
    initTable(&mut (*instance).fields);
    return instance;
}
#[no_mangle]
pub unsafe extern "C" fn newNative(mut function_0: NativeFn) -> *mut ObjNative {
    let mut native: *mut ObjNative = allocateObject(
        ::core::mem::size_of::<ObjNative>() as libc::c_ulong,
        OBJ_NATIVE,
    ) as *mut ObjNative;
    (*native).function = function_0;
    return native;
}
unsafe extern "C" fn allocateString(
    mut chars: *mut libc::c_char,
    mut length: libc::c_int,
    mut hash: uint32_t,
) -> *mut ObjString {
    let mut string: *mut ObjString = allocateObject(
        ::core::mem::size_of::<ObjString>() as libc::c_ulong,
        OBJ_STRING,
    ) as *mut ObjString;
    (*string).length = length;
    (*string).chars = chars;
    (*string).hash = hash;
    push(
        0x8000000000000000 as libc::c_ulong
            | 0x7ffc000000000000 as libc::c_long as uint64_t | string as uintptr_t,
    );
    tableSet(
        &mut vm.strings,
        string,
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 1 as libc::c_int as libc::c_ulong,
    );
    pop();
    return string;
}
unsafe extern "C" fn hashString(
    mut key: *const libc::c_char,
    mut length: libc::c_int,
) -> uint32_t {
    let mut hash: uint32_t = 2166136261 as libc::c_uint;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < length {
        hash ^= *key.offset(i as isize) as uint8_t as libc::c_uint;
        hash = (hash as libc::c_uint)
            .wrapping_mul(16777619 as libc::c_int as libc::c_uint) as uint32_t
            as uint32_t;
        i += 1;
        i;
    }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn takeString(
    mut chars: *mut libc::c_char,
    mut length: libc::c_int,
) -> *mut ObjString {
    let mut hash: uint32_t = hashString(chars, length);
    let mut interned: *mut ObjString = tableFindString(
        &mut vm.strings,
        chars,
        length,
        hash,
    );
    if !interned.is_null() {
        reallocate(
            chars as *mut libc::c_void,
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul((length + 1 as libc::c_int) as libc::c_ulong),
            0 as libc::c_int as size_t,
        );
        return interned;
    }
    return allocateString(chars, length, hash);
}
#[no_mangle]
pub unsafe extern "C" fn copyString(
    mut chars: *const libc::c_char,
    mut length: libc::c_int,
) -> *mut ObjString {
    let mut hash: uint32_t = hashString(chars, length);
    let mut interned: *mut ObjString = tableFindString(
        &mut vm.strings,
        chars,
        length,
        hash,
    );
    if !interned.is_null() {
        return interned;
    }
    let mut heapChars: *mut libc::c_char = reallocate(
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul((length + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        heapChars as *mut libc::c_void,
        chars as *const libc::c_void,
        length as libc::c_ulong,
    );
    *heapChars.offset(length as isize) = '\0' as i32 as libc::c_char;
    return allocateString(heapChars, length, hash);
}
#[no_mangle]
pub unsafe extern "C" fn newUpvalue(mut slot: *mut Value) -> *mut ObjUpvalue {
    let mut upvalue: *mut ObjUpvalue = allocateObject(
        ::core::mem::size_of::<ObjUpvalue>() as libc::c_ulong,
        OBJ_UPVALUE,
    ) as *mut ObjUpvalue;
    (*upvalue)
        .closed = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 1 as libc::c_int as libc::c_ulong;
    (*upvalue).location = slot;
    (*upvalue).next = 0 as *mut ObjUpvalue;
    return upvalue;
}
unsafe extern "C" fn printFunction(mut function_0: *mut ObjFunction) {
    if ((*function_0).name).is_null() {
        printf(b"<script>\0" as *const u8 as *const libc::c_char);
        return;
    }
    printf(
        b"<fn %s>\0" as *const u8 as *const libc::c_char,
        (*(*function_0).name).chars,
    );
}
#[no_mangle]
pub unsafe extern "C" fn printObject(mut value: Value) {
    match (*((value
        & !(0x8000000000000000 as libc::c_ulong
            | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj))
        .type_0 as libc::c_uint
    {
        0 => {
            printFunction(
                (*(*((value
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjBoundMethod))
                    .method)
                    .function,
            );
        }
        1 => {
            printf(
                b"%s\0" as *const u8 as *const libc::c_char,
                (*(*((value
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjClass))
                    .name)
                    .chars,
            );
        }
        2 => {
            printFunction(
                (*((value
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjClosure))
                    .function,
            );
        }
        3 => {
            printFunction(
                (value
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjFunction,
            );
        }
        4 => {
            printf(
                b"%s instance\0" as *const u8 as *const libc::c_char,
                (*(*(*((value
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjInstance))
                    .klass)
                    .name)
                    .chars,
            );
        }
        5 => {
            printf(b"<native fn>\0" as *const u8 as *const libc::c_char);
        }
        6 => {
            printf(
                b"%s\0" as *const u8 as *const libc::c_char,
                (*((value
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjString))
                    .chars,
            );
        }
        7 => {
            printf(b"upvalue\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    };
}
#[no_mangle]
pub static mut scanner: Scanner = Scanner {
    start: 0 as *const libc::c_char,
    current: 0 as *const libc::c_char,
    line: 0,
};
#[no_mangle]
pub unsafe extern "C" fn initScanner(mut source: *const libc::c_char) {
    scanner.start = source;
    scanner.current = source;
    scanner.line = 1 as libc::c_int;
}
unsafe extern "C" fn isAlpha(mut c: libc::c_char) -> bool {
    return c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32
        || c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32
        || c as libc::c_int == '_' as i32;
}
unsafe extern "C" fn isDigit(mut c: libc::c_char) -> bool {
    return c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32;
}
unsafe extern "C" fn isAtEnd() -> bool {
    return *scanner.current as libc::c_int == '\0' as i32;
}
unsafe extern "C" fn advance_scanner() -> libc::c_char {
    scanner.current = (scanner.current).offset(1);
    scanner.current;
    return *(scanner.current).offset(-(1 as libc::c_int) as isize);
}
unsafe extern "C" fn peek_scanner() -> libc::c_char {
    return *scanner.current;
}
unsafe extern "C" fn peekNext() -> libc::c_char {
    if isAtEnd() {
        return '\0' as i32 as libc::c_char;
    }
    return *(scanner.current).offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn match_scanner(mut expected: libc::c_char) -> bool {
    if isAtEnd() {
        return 0 as libc::c_int != 0;
    }
    if *scanner.current as libc::c_int != expected as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    scanner.current = (scanner.current).offset(1);
    scanner.current;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn makeToken(mut type_0: TokenType) -> Token {
    let mut token: Token = Token {
        type_0: TOKEN_LEFT_PAREN,
        start: 0 as *const libc::c_char,
        length: 0,
        line: 0,
    };
    token.type_0 = type_0;
    token.start = scanner.start;
    token
        .length = (scanner.current).offset_from(scanner.start) as libc::c_long
        as libc::c_int;
    token.line = scanner.line;
    return token;
}
unsafe extern "C" fn errorToken(mut message: *const libc::c_char) -> Token {
    let mut token: Token = Token {
        type_0: TOKEN_LEFT_PAREN,
        start: 0 as *const libc::c_char,
        length: 0,
        line: 0,
    };
    token.type_0 = TOKEN_ERROR;
    token.start = message;
    token.length = strlen(message) as libc::c_int;
    token.line = scanner.line;
    return token;
}
unsafe extern "C" fn skipWhitespace() {
    loop {
        let mut c: libc::c_char = peek_scanner();
        match c as libc::c_int {
            32 | 13 | 9 => {
                advance_scanner();
            }
            10 => {
                scanner.line += 1;
                scanner.line;
                advance_scanner();
            }
            47 => {
                if peekNext() as libc::c_int == '/' as i32 {
                    while peek_scanner() as libc::c_int != '\n' as i32 && !isAtEnd() {
                        advance_scanner();
                    }
                } else {
                    return
                }
            }
            _ => return,
        }
    };
}
unsafe extern "C" fn checkKeyword(
    mut start: libc::c_int,
    mut length: libc::c_int,
    mut rest: *const libc::c_char,
    mut type_0: TokenType,
) -> TokenType {
    if (scanner.current).offset_from(scanner.start) as libc::c_long
        == (start + length) as libc::c_long
        && memcmp(
            (scanner.start).offset(start as isize) as *const libc::c_void,
            rest as *const libc::c_void,
            length as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        return type_0;
    }
    return TOKEN_IDENTIFIER;
}
unsafe extern "C" fn identifierType() -> TokenType {
    match *(scanner.start).offset(0 as libc::c_int as isize) as libc::c_int {
        97 => {
            return checkKeyword(
                1 as libc::c_int,
                2 as libc::c_int,
                b"nd\0" as *const u8 as *const libc::c_char,
                TOKEN_AND,
            );
        }
        99 => {
            return checkKeyword(
                1 as libc::c_int,
                4 as libc::c_int,
                b"lass\0" as *const u8 as *const libc::c_char,
                TOKEN_CLASS,
            );
        }
        101 => {
            return checkKeyword(
                1 as libc::c_int,
                3 as libc::c_int,
                b"lse\0" as *const u8 as *const libc::c_char,
                TOKEN_ELSE,
            );
        }
        102 => {
            if (scanner.current).offset_from(scanner.start) as libc::c_long
                > 1 as libc::c_int as libc::c_long
            {
                match *(scanner.start).offset(1 as libc::c_int as isize) as libc::c_int {
                    97 => {
                        return checkKeyword(
                            2 as libc::c_int,
                            3 as libc::c_int,
                            b"lse\0" as *const u8 as *const libc::c_char,
                            TOKEN_FALSE,
                        );
                    }
                    111 => {
                        return checkKeyword(
                            2 as libc::c_int,
                            1 as libc::c_int,
                            b"r\0" as *const u8 as *const libc::c_char,
                            TOKEN_FOR,
                        );
                    }
                    117 => {
                        return checkKeyword(
                            2 as libc::c_int,
                            1 as libc::c_int,
                            b"n\0" as *const u8 as *const libc::c_char,
                            TOKEN_FUN,
                        );
                    }
                    _ => {}
                }
            }
        }
        105 => {
            return checkKeyword(
                1 as libc::c_int,
                1 as libc::c_int,
                b"f\0" as *const u8 as *const libc::c_char,
                TOKEN_IF,
            );
        }
        110 => {
            return checkKeyword(
                1 as libc::c_int,
                2 as libc::c_int,
                b"il\0" as *const u8 as *const libc::c_char,
                TOKEN_NIL,
            );
        }
        111 => {
            return checkKeyword(
                1 as libc::c_int,
                1 as libc::c_int,
                b"r\0" as *const u8 as *const libc::c_char,
                TOKEN_OR,
            );
        }
        112 => {
            return checkKeyword(
                1 as libc::c_int,
                4 as libc::c_int,
                b"rint\0" as *const u8 as *const libc::c_char,
                TOKEN_PRINT,
            );
        }
        114 => {
            return checkKeyword(
                1 as libc::c_int,
                5 as libc::c_int,
                b"eturn\0" as *const u8 as *const libc::c_char,
                TOKEN_RETURN,
            );
        }
        115 => {
            return checkKeyword(
                1 as libc::c_int,
                4 as libc::c_int,
                b"uper\0" as *const u8 as *const libc::c_char,
                TOKEN_SUPER,
            );
        }
        116 => {
            if (scanner.current).offset_from(scanner.start) as libc::c_long
                > 1 as libc::c_int as libc::c_long
            {
                match *(scanner.start).offset(1 as libc::c_int as isize) as libc::c_int {
                    104 => {
                        return checkKeyword(
                            2 as libc::c_int,
                            2 as libc::c_int,
                            b"is\0" as *const u8 as *const libc::c_char,
                            TOKEN_THIS,
                        );
                    }
                    114 => {
                        return checkKeyword(
                            2 as libc::c_int,
                            2 as libc::c_int,
                            b"ue\0" as *const u8 as *const libc::c_char,
                            TOKEN_TRUE,
                        );
                    }
                    _ => {}
                }
            }
        }
        118 => {
            return checkKeyword(
                1 as libc::c_int,
                2 as libc::c_int,
                b"ar\0" as *const u8 as *const libc::c_char,
                TOKEN_VAR,
            );
        }
        119 => {
            return checkKeyword(
                1 as libc::c_int,
                4 as libc::c_int,
                b"hile\0" as *const u8 as *const libc::c_char,
                TOKEN_WHILE,
            );
        }
        _ => {}
    }
    return TOKEN_IDENTIFIER;
}
unsafe extern "C" fn identifier() -> Token {
    while isAlpha(peek_scanner()) as libc::c_int != 0
        || isDigit(peek_scanner()) as libc::c_int != 0
    {
        advance_scanner();
    }
    return makeToken(identifierType());
}
unsafe extern "C" fn number_scanner() -> Token {
    while isDigit(peek_scanner()) {
        advance_scanner();
    }
    if peek_scanner() as libc::c_int == '.' as i32
        && isDigit(peekNext()) as libc::c_int != 0
    {
        advance_scanner();
        while isDigit(peek_scanner()) {
            advance_scanner();
        }
    }
    return makeToken(TOKEN_NUMBER);
}
unsafe extern "C" fn string_scanner() -> Token {
    while peek_scanner() as libc::c_int != '"' as i32 && !isAtEnd() {
        if peek_scanner() as libc::c_int == '\n' as i32 {
            scanner.line += 1;
            scanner.line;
        }
        advance_scanner();
    }
    if isAtEnd() {
        return errorToken(b"Unterminated string.\0" as *const u8 as *const libc::c_char);
    }
    advance_scanner();
    return makeToken(TOKEN_STRING);
}
#[no_mangle]
pub unsafe extern "C" fn scanToken() -> Token {
    skipWhitespace();
    scanner.start = scanner.current;
    if isAtEnd() {
        return makeToken(TOKEN_EOF);
    }
    let mut c: libc::c_char = advance_scanner();
    if isAlpha(c) {
        return identifier();
    }
    if isDigit(c) {
        return number_scanner();
    }
    match c as libc::c_int {
        40 => return makeToken(TOKEN_LEFT_PAREN),
        41 => return makeToken(TOKEN_RIGHT_PAREN),
        123 => return makeToken(TOKEN_LEFT_BRACE),
        125 => return makeToken(TOKEN_RIGHT_BRACE),
        59 => return makeToken(TOKEN_SEMICOLON),
        44 => return makeToken(TOKEN_COMMA),
        46 => return makeToken(TOKEN_DOT),
        45 => return makeToken(TOKEN_MINUS),
        43 => return makeToken(TOKEN_PLUS),
        47 => return makeToken(TOKEN_SLASH),
        42 => return makeToken(TOKEN_STAR),
        33 => {
            return makeToken(
                (if match_scanner('=' as i32 as libc::c_char) as libc::c_int != 0 {
                    TOKEN_BANG_EQUAL as libc::c_int
                } else {
                    TOKEN_BANG as libc::c_int
                }) as TokenType,
            );
        }
        61 => {
            return makeToken(
                (if match_scanner('=' as i32 as libc::c_char) as libc::c_int != 0 {
                    TOKEN_EQUAL_EQUAL as libc::c_int
                } else {
                    TOKEN_EQUAL as libc::c_int
                }) as TokenType,
            );
        }
        60 => {
            return makeToken(
                (if match_scanner('=' as i32 as libc::c_char) as libc::c_int != 0 {
                    TOKEN_LESS_EQUAL as libc::c_int
                } else {
                    TOKEN_LESS as libc::c_int
                }) as TokenType,
            );
        }
        62 => {
            return makeToken(
                (if match_scanner('=' as i32 as libc::c_char) as libc::c_int != 0 {
                    TOKEN_GREATER_EQUAL as libc::c_int
                } else {
                    TOKEN_GREATER as libc::c_int
                }) as TokenType,
            );
        }
        34 => return string_scanner(),
        _ => {}
    }
    return errorToken(b"Unexpected character.\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn initTable(mut table: *mut Table) {
    (*table).count = 0 as libc::c_int;
    (*table).capacity = 0 as libc::c_int;
    (*table).entries = 0 as *mut Entry;
}
#[no_mangle]
pub unsafe extern "C" fn freeTable(mut table: *mut Table) {
    reallocate(
        (*table).entries as *mut libc::c_void,
        (::core::mem::size_of::<Entry>() as libc::c_ulong)
            .wrapping_mul((*table).capacity as libc::c_ulong),
        0 as libc::c_int as size_t,
    );
    initTable(table);
}
unsafe extern "C" fn findEntry(
    mut entries: *mut Entry,
    mut capacity: libc::c_int,
    mut key: *mut ObjString,
) -> *mut Entry {
    let mut index: uint32_t = (*key).hash
        & (capacity - 1 as libc::c_int) as libc::c_uint;
    let mut tombstone: *mut Entry = 0 as *mut Entry;
    loop {
        let mut entry: *mut Entry = &mut *entries.offset(index as isize) as *mut Entry;
        if ((*entry).key).is_null() {
            if (*entry).value
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 1 as libc::c_int as libc::c_ulong
            {
                return if !tombstone.is_null() { tombstone } else { entry }
            } else if tombstone.is_null() {
                tombstone = entry;
            }
        } else if (*entry).key == key {
            return entry
        }
        index = index.wrapping_add(1 as libc::c_int as libc::c_uint)
            & (capacity - 1 as libc::c_int) as libc::c_uint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tableGet(
    mut table: *mut Table,
    mut key: *mut ObjString,
    mut value: *mut Value,
) -> bool {
    if (*table).count == 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    let mut entry: *mut Entry = findEntry((*table).entries, (*table).capacity, key);
    if ((*entry).key).is_null() {
        return 0 as libc::c_int != 0;
    }
    *value = (*entry).value;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn adjustCapacity(mut table: *mut Table, mut capacity: libc::c_int) {
    let mut entries: *mut Entry = reallocate(
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        (::core::mem::size_of::<Entry>() as libc::c_ulong)
            .wrapping_mul(capacity as libc::c_ulong),
    ) as *mut Entry;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < capacity {
        let ref mut fresh9 = (*entries.offset(i as isize)).key;
        *fresh9 = 0 as *mut ObjString;
        (*entries.offset(i as isize))
            .value = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 1 as libc::c_int as libc::c_ulong;
        i += 1;
        i;
    }
    (*table).count = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < (*table).capacity {
        let mut entry: *mut Entry = &mut *((*table).entries).offset(i_0 as isize)
            as *mut Entry;
        if !((*entry).key).is_null() {
            let mut dest: *mut Entry = findEntry(entries, capacity, (*entry).key);
            (*dest).key = (*entry).key;
            (*dest).value = (*entry).value;
            (*table).count += 1;
            (*table).count;
        }
        i_0 += 1;
        i_0;
    }
    reallocate(
        (*table).entries as *mut libc::c_void,
        (::core::mem::size_of::<Entry>() as libc::c_ulong)
            .wrapping_mul((*table).capacity as libc::c_ulong),
        0 as libc::c_int as size_t,
    );
    (*table).entries = entries;
    (*table).capacity = capacity;
}
#[no_mangle]
pub unsafe extern "C" fn tableSet(
    mut table: *mut Table,
    mut key: *mut ObjString,
    mut value: Value,
) -> bool {
    if ((*table).count + 1 as libc::c_int) as libc::c_double
        > (*table).capacity as libc::c_double * 0.75f64
    {
        let mut capacity: libc::c_int = if (*table).capacity < 8 as libc::c_int {
            8 as libc::c_int
        } else {
            (*table).capacity * 2 as libc::c_int
        };
        adjustCapacity(table, capacity);
    }
    let mut entry: *mut Entry = findEntry((*table).entries, (*table).capacity, key);
    let mut isNewKey: bool = ((*entry).key).is_null();
    if isNewKey as libc::c_int != 0
        && (*entry).value
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 1 as libc::c_int as libc::c_ulong
    {
        (*table).count += 1;
        (*table).count;
    }
    (*entry).key = key;
    (*entry).value = value;
    return isNewKey;
}
#[no_mangle]
pub unsafe extern "C" fn tableDelete(
    mut table: *mut Table,
    mut key: *mut ObjString,
) -> bool {
    if (*table).count == 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    let mut entry: *mut Entry = findEntry((*table).entries, (*table).capacity, key);
    if ((*entry).key).is_null() {
        return 0 as libc::c_int != 0;
    }
    (*entry).key = 0 as *mut ObjString;
    (*entry)
        .value = if 1 as libc::c_int != 0 {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 3 as libc::c_int as libc::c_ulong
    } else {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 2 as libc::c_int as libc::c_ulong
    };
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn tableAddAll(mut from: *mut Table, mut to: *mut Table) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*from).capacity {
        let mut entry: *mut Entry = &mut *((*from).entries).offset(i as isize)
            as *mut Entry;
        if !((*entry).key).is_null() {
            tableSet(to, (*entry).key, (*entry).value);
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn tableFindString(
    mut table: *mut Table,
    mut chars: *const libc::c_char,
    mut length: libc::c_int,
    mut hash: uint32_t,
) -> *mut ObjString {
    if (*table).count == 0 as libc::c_int {
        return 0 as *mut ObjString;
    }
    let mut index: uint32_t = hash
        & ((*table).capacity - 1 as libc::c_int) as libc::c_uint;
    loop {
        let mut entry: *mut Entry = &mut *((*table).entries).offset(index as isize)
            as *mut Entry;
        if ((*entry).key).is_null() {
            if (*entry).value
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 1 as libc::c_int as libc::c_ulong
            {
                return 0 as *mut ObjString;
            }
        } else if (*(*entry).key).length == length && (*(*entry).key).hash == hash
            && memcmp(
                (*(*entry).key).chars as *const libc::c_void,
                chars as *const libc::c_void,
                length as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            return (*entry).key
        }
        index = index.wrapping_add(1 as libc::c_int as libc::c_uint)
            & ((*table).capacity - 1 as libc::c_int) as libc::c_uint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tableRemoveWhite(mut table: *mut Table) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*table).capacity {
        let mut entry: *mut Entry = &mut *((*table).entries).offset(i as isize)
            as *mut Entry;
        if !((*entry).key).is_null() && !(*(*entry).key).obj.isMarked {
            tableDelete(table, (*entry).key);
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn markTable(mut table: *mut Table) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*table).capacity {
        let mut entry: *mut Entry = &mut *((*table).entries).offset(i as isize)
            as *mut Entry;
        markObject((*entry).key as *mut Obj);
        markValue((*entry).value);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn initValueArray(mut array: *mut ValueArray) {
    (*array).values = 0 as *mut Value;
    (*array).capacity = 0 as libc::c_int;
    (*array).count = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn writeValueArray(mut array: *mut ValueArray, mut value: Value) {
    if (*array).capacity < (*array).count + 1 as libc::c_int {
        let mut oldCapacity: libc::c_int = (*array).capacity;
        (*array)
            .capacity = if oldCapacity < 8 as libc::c_int {
            8 as libc::c_int
        } else {
            oldCapacity * 2 as libc::c_int
        };
        (*array)
            .values = reallocate(
            (*array).values as *mut libc::c_void,
            (::core::mem::size_of::<Value>() as libc::c_ulong)
                .wrapping_mul(oldCapacity as libc::c_ulong),
            (::core::mem::size_of::<Value>() as libc::c_ulong)
                .wrapping_mul((*array).capacity as libc::c_ulong),
        ) as *mut Value;
    }
    *((*array).values).offset((*array).count as isize) = value;
    (*array).count += 1;
    (*array).count;
}
#[no_mangle]
pub unsafe extern "C" fn freeValueArray(mut array: *mut ValueArray) {
    reallocate(
        (*array).values as *mut libc::c_void,
        (::core::mem::size_of::<Value>() as libc::c_ulong)
            .wrapping_mul((*array).capacity as libc::c_ulong),
        0 as libc::c_int as size_t,
    );
    initValueArray(array);
}
#[no_mangle]
pub unsafe extern "C" fn printValue(mut value: Value) {
    if value | 1 as libc::c_int as libc::c_ulong
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 3 as libc::c_int as libc::c_ulong
    {
        printf(
            if value
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 3 as libc::c_int as libc::c_ulong
            {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
        );
    } else if value
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 1 as libc::c_int as libc::c_ulong
    {
        printf(b"nil\0" as *const u8 as *const libc::c_char);
    } else if value & 0x7ffc000000000000 as libc::c_long as uint64_t
        != 0x7ffc000000000000 as libc::c_long as uint64_t
    {
        printf(b"%g\0" as *const u8 as *const libc::c_char, valueToNum(value));
    } else if value
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
    {
        printObject(value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn valuesEqual(mut a: Value, mut b: Value) -> bool {
    if a & 0x7ffc000000000000 as libc::c_long as uint64_t
        != 0x7ffc000000000000 as libc::c_long as uint64_t
        && b & 0x7ffc000000000000 as libc::c_long as uint64_t
            != 0x7ffc000000000000 as libc::c_long as uint64_t
    {
        return valueToNum(a) == valueToNum(b);
    }
    return a == b;
}
#[no_mangle]
pub static mut vm: VM = VM {
    frames: [CallFrame {
        closure: 0 as *const ObjClosure as *mut ObjClosure,
        ip: 0 as *const uint8_t as *mut uint8_t,
        slots: 0 as *const Value as *mut Value,
    }; 64],
    frameCount: 0,
    stack: [0; 16384],
    stackTop: 0 as *const Value as *mut Value,
    globals: Table {
        count: 0,
        capacity: 0,
        entries: 0 as *const Entry as *mut Entry,
    },
    strings: Table {
        count: 0,
        capacity: 0,
        entries: 0 as *const Entry as *mut Entry,
    },
    initString: 0 as *const ObjString as *mut ObjString,
    openUpvalues: 0 as *const ObjUpvalue as *mut ObjUpvalue,
    bytesAllocated: 0,
    nextGC: 0,
    objects: 0 as *const Obj as *mut Obj,
    grayCount: 0,
    grayCapacity: 0,
    grayStack: 0 as *const *mut Obj as *mut *mut Obj,
};
unsafe extern "C" fn clockNative(
    mut argCount: libc::c_int,
    mut args: *mut Value,
) -> Value {
    return numToValue(
        clock() as libc::c_double / 1000000 as libc::c_int as __clock_t as libc::c_double,
    );
}
unsafe extern "C" fn resetStack() {
    vm.stackTop = (vm.stack).as_mut_ptr();
    vm.frameCount = 0 as libc::c_int;
    vm.openUpvalues = 0 as *mut ObjUpvalue;
}
unsafe extern "C" fn runtimeError(mut format: *const libc::c_char, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    fputs(b"\n\0" as *const u8 as *const libc::c_char, stderr);
    let mut i: libc::c_int = vm.frameCount - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut frame: *mut CallFrame = &mut *(vm.frames).as_mut_ptr().offset(i as isize)
            as *mut CallFrame;
        let mut function_0: *mut ObjFunction = (*(*frame).closure).function;
        let mut instruction: size_t = (((*frame).ip)
            .offset_from((*function_0).chunk.code) as libc::c_long
            - 1 as libc::c_int as libc::c_long) as size_t;
        fprintf(
            stderr,
            b"[line %d] in \0" as *const u8 as *const libc::c_char,
            *((*function_0).chunk.lines).offset(instruction as isize),
        );
        if ((*function_0).name).is_null() {
            fprintf(stderr, b"script\n\0" as *const u8 as *const libc::c_char);
        } else {
            fprintf(
                stderr,
                b"%s()\n\0" as *const u8 as *const libc::c_char,
                (*(*function_0).name).chars,
            );
        }
        i -= 1;
        i;
    }
    resetStack();
}
unsafe extern "C" fn defineNative(
    mut name: *const libc::c_char,
    mut function_0: NativeFn,
) {
    push(
        0x8000000000000000 as libc::c_ulong
            | 0x7ffc000000000000 as libc::c_long as uint64_t
            | copyString(name, strlen(name) as libc::c_int) as uintptr_t,
    );
    push(
        0x8000000000000000 as libc::c_ulong
            | 0x7ffc000000000000 as libc::c_long as uint64_t
            | newNative(function_0) as uintptr_t,
    );
    tableSet(
        &mut vm.globals,
        (vm.stack[0 as libc::c_int as usize]
            & !(0x8000000000000000 as libc::c_ulong
                | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
            as *mut ObjString,
        vm.stack[1 as libc::c_int as usize],
    );
    pop();
    pop();
}
#[no_mangle]
pub unsafe extern "C" fn initVM() {
    resetStack();
    vm.objects = 0 as *mut Obj;
    vm.bytesAllocated = 0 as libc::c_int as size_t;
    vm.nextGC = (1024 as libc::c_int * 1024 as libc::c_int) as size_t;
    vm.grayCount = 0 as libc::c_int;
    vm.grayCapacity = 0 as libc::c_int;
    vm.grayStack = 0 as *mut *mut Obj;
    initTable(&mut vm.globals);
    initTable(&mut vm.strings);
    vm.initString = 0 as *mut ObjString;
    vm
        .initString = copyString(
        b"init\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int,
    );
    defineNative(
        b"clock\0" as *const u8 as *const libc::c_char,
        Some(clockNative as unsafe extern "C" fn(libc::c_int, *mut Value) -> Value),
    );
}
#[no_mangle]
pub unsafe extern "C" fn freeVM() {
    freeTable(&mut vm.globals);
    freeTable(&mut vm.strings);
    vm.initString = 0 as *mut ObjString;
    freeObjects();
}
#[no_mangle]
pub unsafe extern "C" fn push(mut value: Value) {
    *vm.stackTop = value;
    vm.stackTop = (vm.stackTop).offset(1);
    vm.stackTop;
}
#[no_mangle]
pub unsafe extern "C" fn pop() -> Value {
    vm.stackTop = (vm.stackTop).offset(-1);
    vm.stackTop;
    return *vm.stackTop;
}
unsafe extern "C" fn peek_vm(mut distance: libc::c_int) -> Value {
    return *(vm.stackTop).offset((-(1 as libc::c_int) - distance) as isize);
}
unsafe extern "C" fn call_vm(
    mut closure: *mut ObjClosure,
    mut argCount: libc::c_int,
) -> bool {
    if argCount != (*(*closure).function).arity {
        runtimeError(
            b"Expected %d arguments but got %d.\0" as *const u8 as *const libc::c_char,
            (*(*closure).function).arity,
            argCount,
        );
        return 0 as libc::c_int != 0;
    }
    if vm.frameCount == 64 as libc::c_int {
        runtimeError(b"Stack overflow.\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int != 0;
    }
    let fresh10 = vm.frameCount;
    vm.frameCount = vm.frameCount + 1;
    let mut frame: *mut CallFrame = &mut *(vm.frames)
        .as_mut_ptr()
        .offset(fresh10 as isize) as *mut CallFrame;
    (*frame).closure = closure;
    (*frame).ip = (*(*closure).function).chunk.code;
    (*frame)
        .slots = (vm.stackTop)
        .offset(-(argCount as isize))
        .offset(-(1 as libc::c_int as isize));
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn callValue(mut callee: Value, mut argCount: libc::c_int) -> bool {
    if callee
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
    {
        match (*((callee
            & !(0x8000000000000000 as libc::c_ulong
                | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj))
            .type_0 as libc::c_uint
        {
            0 => {
                let mut bound: *mut ObjBoundMethod = (callee
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjBoundMethod;
                *(vm.stackTop)
                    .offset((-argCount - 1 as libc::c_int) as isize) = (*bound).receiver;
                return call_vm((*bound).method, argCount);
            }
            1 => {
                let mut klass: *mut ObjClass = (callee
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjClass;
                *(vm.stackTop)
                    .offset(
                        (-argCount - 1 as libc::c_int) as isize,
                    ) = 0x8000000000000000 as libc::c_ulong
                    | 0x7ffc000000000000 as libc::c_long as uint64_t
                    | newInstance(klass) as uintptr_t;
                let mut initializer: Value = 0;
                if tableGet(&mut (*klass).methods, vm.initString, &mut initializer) {
                    return call_vm(
                        (initializer
                            & !(0x8000000000000000 as libc::c_ulong
                                | 0x7ffc000000000000 as libc::c_long as uint64_t))
                            as *mut Obj as *mut ObjClosure,
                        argCount,
                    )
                } else if argCount != 0 as libc::c_int {
                    runtimeError(
                        b"Expected 0 arguments but got %d.\0" as *const u8
                            as *const libc::c_char,
                        argCount,
                    );
                    return 0 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
            2 => {
                return call_vm(
                    (callee
                        & !(0x8000000000000000 as libc::c_ulong
                            | 0x7ffc000000000000 as libc::c_long as uint64_t))
                        as *mut Obj as *mut ObjClosure,
                    argCount,
                );
            }
            5 => {
                let mut native: NativeFn = (*((callee
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjNative))
                    .function;
                let mut result: Value = native
                    .expect(
                        "non-null function pointer",
                    )(argCount, (vm.stackTop).offset(-(argCount as isize)));
                vm
                    .stackTop = (vm.stackTop)
                    .offset(-((argCount + 1 as libc::c_int) as isize));
                push(result);
                return 1 as libc::c_int != 0;
            }
            _ => {}
        }
    }
    runtimeError(
        b"Can only call functions and classes.\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn invokeFromClass(
    mut klass: *mut ObjClass,
    mut name: *mut ObjString,
    mut argCount: libc::c_int,
) -> bool {
    let mut method_0: Value = 0;
    if !tableGet(&mut (*klass).methods, name, &mut method_0) {
        runtimeError(
            b"Undefined property '%s'.\0" as *const u8 as *const libc::c_char,
            (*name).chars,
        );
        return 0 as libc::c_int != 0;
    }
    return call_vm(
        (method_0
            & !(0x8000000000000000 as libc::c_ulong
                | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
            as *mut ObjClosure,
        argCount,
    );
}
unsafe extern "C" fn invoke(
    mut name: *mut ObjString,
    mut argCount: libc::c_int,
) -> bool {
    let mut receiver: Value = peek_vm(argCount);
    if !isObjType(receiver, OBJ_INSTANCE) {
        runtimeError(
            b"Only instances have methods.\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    let mut instance: *mut ObjInstance = (receiver
        & !(0x8000000000000000 as libc::c_ulong
            | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
        as *mut ObjInstance;
    let mut value: Value = 0;
    if tableGet(&mut (*instance).fields, name, &mut value) {
        *(vm.stackTop).offset((-argCount - 1 as libc::c_int) as isize) = value;
        return callValue(value, argCount);
    }
    return invokeFromClass((*instance).klass, name, argCount);
}
unsafe extern "C" fn bindMethod(
    mut klass: *mut ObjClass,
    mut name: *mut ObjString,
) -> bool {
    let mut method_0: Value = 0;
    if !tableGet(&mut (*klass).methods, name, &mut method_0) {
        runtimeError(
            b"Undefined property '%s'.\0" as *const u8 as *const libc::c_char,
            (*name).chars,
        );
        return 0 as libc::c_int != 0;
    }
    let mut bound: *mut ObjBoundMethod = newBoundMethod(
        peek_vm(0 as libc::c_int),
        (method_0
            & !(0x8000000000000000 as libc::c_ulong
                | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
            as *mut ObjClosure,
    );
    pop();
    push(
        0x8000000000000000 as libc::c_ulong
            | 0x7ffc000000000000 as libc::c_long as uint64_t | bound as uintptr_t,
    );
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn captureUpvalue(mut local: *mut Value) -> *mut ObjUpvalue {
    let mut prevUpvalue: *mut ObjUpvalue = 0 as *mut ObjUpvalue;
    let mut upvalue: *mut ObjUpvalue = vm.openUpvalues;
    while !upvalue.is_null() && (*upvalue).location > local {
        prevUpvalue = upvalue;
        upvalue = (*upvalue).next;
    }
    if !upvalue.is_null() && (*upvalue).location == local {
        return upvalue;
    }
    let mut createdUpvalue: *mut ObjUpvalue = newUpvalue(local);
    (*createdUpvalue).next = upvalue;
    if prevUpvalue.is_null() {
        vm.openUpvalues = createdUpvalue;
    } else {
        (*prevUpvalue).next = createdUpvalue;
    }
    return createdUpvalue;
}
unsafe extern "C" fn closeUpvalues(mut last: *mut Value) {
    while !(vm.openUpvalues).is_null() && (*vm.openUpvalues).location >= last {
        let mut upvalue: *mut ObjUpvalue = vm.openUpvalues;
        (*upvalue).closed = *(*upvalue).location;
        (*upvalue).location = &mut (*upvalue).closed;
        vm.openUpvalues = (*upvalue).next;
    }
}
unsafe extern "C" fn defineMethod(mut name: *mut ObjString) {
    let mut method_0: Value = peek_vm(0 as libc::c_int);
    let mut klass: *mut ObjClass = (peek_vm(1 as libc::c_int)
        & !(0x8000000000000000 as libc::c_ulong
            | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
        as *mut ObjClass;
    tableSet(&mut (*klass).methods, name, method_0);
    pop();
}
unsafe extern "C" fn isFalsey(mut value: Value) -> bool {
    return value
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 1 as libc::c_int as libc::c_ulong
        || value | 1 as libc::c_int as libc::c_ulong
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 3 as libc::c_int as libc::c_ulong
            && !(value
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 3 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn concatenate() {
    let mut b: *mut ObjString = (peek_vm(0 as libc::c_int)
        & !(0x8000000000000000 as libc::c_ulong
            | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
        as *mut ObjString;
    let mut a: *mut ObjString = (peek_vm(1 as libc::c_int)
        & !(0x8000000000000000 as libc::c_ulong
            | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
        as *mut ObjString;
    let mut length: libc::c_int = (*a).length + (*b).length;
    let mut chars: *mut libc::c_char = reallocate(
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul((length + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        chars as *mut libc::c_void,
        (*a).chars as *const libc::c_void,
        (*a).length as libc::c_ulong,
    );
    memcpy(
        chars.offset((*a).length as isize) as *mut libc::c_void,
        (*b).chars as *const libc::c_void,
        (*b).length as libc::c_ulong,
    );
    *chars.offset(length as isize) = '\0' as i32 as libc::c_char;
    let mut result: *mut ObjString = takeString(chars, length);
    pop();
    pop();
    push(
        0x8000000000000000 as libc::c_ulong
            | 0x7ffc000000000000 as libc::c_long as uint64_t | result as uintptr_t,
    );
}
unsafe extern "C" fn run() -> InterpretResult {
    let mut frame: *mut CallFrame = &mut *(vm.frames)
        .as_mut_ptr()
        .offset((vm.frameCount - 1 as libc::c_int) as isize) as *mut CallFrame;
    loop {
        let mut instruction: uint8_t = 0;
        let fresh11 = (*frame).ip;
        (*frame).ip = ((*frame).ip).offset(1);
        instruction = *fresh11;
        match instruction as libc::c_int {
            0 => {
                let fresh12 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut constant: Value = *((*(*(*frame).closure).function)
                    .chunk
                    .constants
                    .values)
                    .offset(*fresh12 as isize);
                push(constant);
            }
            1 => {
                push(
                    0x7ffc000000000000 as libc::c_long as uint64_t
                        | 1 as libc::c_int as libc::c_ulong,
                );
            }
            2 => {
                push(
                    if 1 as libc::c_int != 0 {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 3 as libc::c_int as libc::c_ulong
                    } else {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 2 as libc::c_int as libc::c_ulong
                    },
                );
            }
            3 => {
                push(
                    if 0 as libc::c_int != 0 {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 3 as libc::c_int as libc::c_ulong
                    } else {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 2 as libc::c_int as libc::c_ulong
                    },
                );
            }
            4 => {
                pop();
            }
            5 => {
                let fresh13 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut slot: uint8_t = *fresh13;
                push(*((*frame).slots).offset(slot as isize));
            }
            6 => {
                let fresh14 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut slot_0: uint8_t = *fresh14;
                *((*frame).slots).offset(slot_0 as isize) = peek_vm(0 as libc::c_int);
            }
            7 => {
                let fresh15 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut name: *mut ObjString = (*((*(*(*frame).closure).function)
                    .chunk
                    .constants
                    .values)
                    .offset(*fresh15 as isize)
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjString;
                let mut value: Value = 0;
                if !tableGet(&mut vm.globals, name, &mut value) {
                    runtimeError(
                        b"Undefined variable '%s'.\0" as *const u8
                            as *const libc::c_char,
                        (*name).chars,
                    );
                    return INTERPRET_RUNTIME_ERROR;
                }
                push(value);
            }
            8 => {
                let fresh16 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut name_0: *mut ObjString = (*((*(*(*frame).closure).function)
                    .chunk
                    .constants
                    .values)
                    .offset(*fresh16 as isize)
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjString;
                tableSet(&mut vm.globals, name_0, peek_vm(0 as libc::c_int));
                pop();
            }
            9 => {
                let fresh17 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut name_1: *mut ObjString = (*((*(*(*frame).closure).function)
                    .chunk
                    .constants
                    .values)
                    .offset(*fresh17 as isize)
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjString;
                if tableSet(&mut vm.globals, name_1, peek_vm(0 as libc::c_int)) {
                    tableDelete(&mut vm.globals, name_1);
                    runtimeError(
                        b"Undefined variable '%s'.\0" as *const u8
                            as *const libc::c_char,
                        (*name_1).chars,
                    );
                    return INTERPRET_RUNTIME_ERROR;
                }
            }
            10 => {
                let fresh18 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut slot_1: uint8_t = *fresh18;
                push(
                    *(**((*(*frame).closure).upvalues).offset(slot_1 as isize)).location,
                );
            }
            11 => {
                let fresh19 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut slot_2: uint8_t = *fresh19;
                *(**((*(*frame).closure).upvalues).offset(slot_2 as isize))
                    .location = peek_vm(0 as libc::c_int);
            }
            12 => {
                if !isObjType(peek_vm(0 as libc::c_int), OBJ_INSTANCE) {
                    runtimeError(
                        b"Only instances have properties.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return INTERPRET_RUNTIME_ERROR;
                }
                let mut instance: *mut ObjInstance = (peek_vm(0 as libc::c_int)
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjInstance;
                let fresh20 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut name_2: *mut ObjString = (*((*(*(*frame).closure).function)
                    .chunk
                    .constants
                    .values)
                    .offset(*fresh20 as isize)
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjString;
                let mut value_0: Value = 0;
                if tableGet(&mut (*instance).fields, name_2, &mut value_0) {
                    pop();
                    push(value_0);
                } else if !bindMethod((*instance).klass, name_2) {
                    return INTERPRET_RUNTIME_ERROR
                }
            }
            13 => {
                if !isObjType(peek_vm(1 as libc::c_int), OBJ_INSTANCE) {
                    runtimeError(
                        b"Only instances have fields.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return INTERPRET_RUNTIME_ERROR;
                }
                let mut instance_0: *mut ObjInstance = (peek_vm(1 as libc::c_int)
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjInstance;
                let fresh21 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                tableSet(
                    &mut (*instance_0).fields,
                    (*((*(*(*frame).closure).function).chunk.constants.values)
                        .offset(*fresh21 as isize)
                        & !(0x8000000000000000 as libc::c_ulong
                            | 0x7ffc000000000000 as libc::c_long as uint64_t))
                        as *mut Obj as *mut ObjString,
                    peek_vm(0 as libc::c_int),
                );
                let mut value_1: Value = pop();
                pop();
                push(value_1);
            }
            14 => {
                let fresh22 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut name_3: *mut ObjString = (*((*(*(*frame).closure).function)
                    .chunk
                    .constants
                    .values)
                    .offset(*fresh22 as isize)
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjString;
                let mut superclass: *mut ObjClass = (pop()
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjClass;
                if !bindMethod(superclass, name_3) {
                    return INTERPRET_RUNTIME_ERROR;
                }
            }
            15 => {
                let mut b: Value = pop();
                let mut a: Value = pop();
                push(
                    if valuesEqual(a, b) as libc::c_int != 0 {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 3 as libc::c_int as libc::c_ulong
                    } else {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 2 as libc::c_int as libc::c_ulong
                    },
                );
            }
            16 => {
                if !(peek_vm(0 as libc::c_int)
                    & 0x7ffc000000000000 as libc::c_long as uint64_t
                    != 0x7ffc000000000000 as libc::c_long as uint64_t)
                    || !(peek_vm(1 as libc::c_int)
                        & 0x7ffc000000000000 as libc::c_long as uint64_t
                        != 0x7ffc000000000000 as libc::c_long as uint64_t)
                {
                    runtimeError(
                        b"Operands must be numbers.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return INTERPRET_RUNTIME_ERROR;
                }
                let mut b_0: libc::c_double = valueToNum(pop());
                let mut a_0: libc::c_double = valueToNum(pop());
                push(
                    if a_0 > b_0 {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 3 as libc::c_int as libc::c_ulong
                    } else {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 2 as libc::c_int as libc::c_ulong
                    },
                );
            }
            17 => {
                if !(peek_vm(0 as libc::c_int)
                    & 0x7ffc000000000000 as libc::c_long as uint64_t
                    != 0x7ffc000000000000 as libc::c_long as uint64_t)
                    || !(peek_vm(1 as libc::c_int)
                        & 0x7ffc000000000000 as libc::c_long as uint64_t
                        != 0x7ffc000000000000 as libc::c_long as uint64_t)
                {
                    runtimeError(
                        b"Operands must be numbers.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return INTERPRET_RUNTIME_ERROR;
                }
                let mut b_1: libc::c_double = valueToNum(pop());
                let mut a_1: libc::c_double = valueToNum(pop());
                push(
                    if a_1 < b_1 {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 3 as libc::c_int as libc::c_ulong
                    } else {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 2 as libc::c_int as libc::c_ulong
                    },
                );
            }
            18 => {
                if isObjType(peek_vm(0 as libc::c_int), OBJ_STRING) as libc::c_int != 0
                    && isObjType(peek_vm(1 as libc::c_int), OBJ_STRING) as libc::c_int
                        != 0
                {
                    concatenate();
                } else if peek_vm(0 as libc::c_int)
                    & 0x7ffc000000000000 as libc::c_long as uint64_t
                    != 0x7ffc000000000000 as libc::c_long as uint64_t
                    && peek_vm(1 as libc::c_int)
                        & 0x7ffc000000000000 as libc::c_long as uint64_t
                        != 0x7ffc000000000000 as libc::c_long as uint64_t
                {
                    let mut b_2: libc::c_double = valueToNum(pop());
                    let mut a_2: libc::c_double = valueToNum(pop());
                    push(numToValue(a_2 + b_2));
                } else {
                    runtimeError(
                        b"Operands must be two numbers or two strings.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return INTERPRET_RUNTIME_ERROR;
                }
            }
            19 => {
                if !(peek_vm(0 as libc::c_int)
                    & 0x7ffc000000000000 as libc::c_long as uint64_t
                    != 0x7ffc000000000000 as libc::c_long as uint64_t)
                    || !(peek_vm(1 as libc::c_int)
                        & 0x7ffc000000000000 as libc::c_long as uint64_t
                        != 0x7ffc000000000000 as libc::c_long as uint64_t)
                {
                    runtimeError(
                        b"Operands must be numbers.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return INTERPRET_RUNTIME_ERROR;
                }
                let mut b_3: libc::c_double = valueToNum(pop());
                let mut a_3: libc::c_double = valueToNum(pop());
                push(numToValue(a_3 - b_3));
            }
            20 => {
                if !(peek_vm(0 as libc::c_int)
                    & 0x7ffc000000000000 as libc::c_long as uint64_t
                    != 0x7ffc000000000000 as libc::c_long as uint64_t)
                    || !(peek_vm(1 as libc::c_int)
                        & 0x7ffc000000000000 as libc::c_long as uint64_t
                        != 0x7ffc000000000000 as libc::c_long as uint64_t)
                {
                    runtimeError(
                        b"Operands must be numbers.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return INTERPRET_RUNTIME_ERROR;
                }
                let mut b_4: libc::c_double = valueToNum(pop());
                let mut a_4: libc::c_double = valueToNum(pop());
                push(numToValue(a_4 * b_4));
            }
            21 => {
                if !(peek_vm(0 as libc::c_int)
                    & 0x7ffc000000000000 as libc::c_long as uint64_t
                    != 0x7ffc000000000000 as libc::c_long as uint64_t)
                    || !(peek_vm(1 as libc::c_int)
                        & 0x7ffc000000000000 as libc::c_long as uint64_t
                        != 0x7ffc000000000000 as libc::c_long as uint64_t)
                {
                    runtimeError(
                        b"Operands must be numbers.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return INTERPRET_RUNTIME_ERROR;
                }
                let mut b_5: libc::c_double = valueToNum(pop());
                let mut a_5: libc::c_double = valueToNum(pop());
                push(numToValue(a_5 / b_5));
            }
            22 => {
                push(
                    if isFalsey(pop()) as libc::c_int != 0 {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 3 as libc::c_int as libc::c_ulong
                    } else {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 2 as libc::c_int as libc::c_ulong
                    },
                );
            }
            23 => {
                if !(peek_vm(0 as libc::c_int)
                    & 0x7ffc000000000000 as libc::c_long as uint64_t
                    != 0x7ffc000000000000 as libc::c_long as uint64_t)
                {
                    runtimeError(
                        b"Operand must be a number.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return INTERPRET_RUNTIME_ERROR;
                }
                push(numToValue(-valueToNum(pop())));
            }
            24 => {
                printValue(pop());
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            25 => {
                (*frame).ip = ((*frame).ip).offset(2 as libc::c_int as isize);
                let mut offset: uint16_t = ((*((*frame).ip)
                    .offset(-(2 as libc::c_int) as isize) as libc::c_int)
                    << 8 as libc::c_int
                    | *((*frame).ip).offset(-(1 as libc::c_int) as isize) as libc::c_int)
                    as uint16_t;
                (*frame).ip = ((*frame).ip).offset(offset as libc::c_int as isize);
            }
            26 => {
                (*frame).ip = ((*frame).ip).offset(2 as libc::c_int as isize);
                let mut offset_0: uint16_t = ((*((*frame).ip)
                    .offset(-(2 as libc::c_int) as isize) as libc::c_int)
                    << 8 as libc::c_int
                    | *((*frame).ip).offset(-(1 as libc::c_int) as isize) as libc::c_int)
                    as uint16_t;
                if isFalsey(peek_vm(0 as libc::c_int)) {
                    (*frame).ip = ((*frame).ip).offset(offset_0 as libc::c_int as isize);
                }
            }
            27 => {
                (*frame).ip = ((*frame).ip).offset(2 as libc::c_int as isize);
                let mut offset_1: uint16_t = ((*((*frame).ip)
                    .offset(-(2 as libc::c_int) as isize) as libc::c_int)
                    << 8 as libc::c_int
                    | *((*frame).ip).offset(-(1 as libc::c_int) as isize) as libc::c_int)
                    as uint16_t;
                (*frame).ip = ((*frame).ip).offset(-(offset_1 as libc::c_int as isize));
            }
            28 => {
                let fresh23 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut argCount: libc::c_int = *fresh23 as libc::c_int;
                if !callValue(peek_vm(argCount), argCount) {
                    return INTERPRET_RUNTIME_ERROR;
                }
                frame = &mut *(vm.frames)
                    .as_mut_ptr()
                    .offset((vm.frameCount - 1 as libc::c_int) as isize)
                    as *mut CallFrame;
            }
            29 => {
                let fresh24 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut method_0: *mut ObjString = (*((*(*(*frame).closure).function)
                    .chunk
                    .constants
                    .values)
                    .offset(*fresh24 as isize)
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjString;
                let fresh25 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut argCount_0: libc::c_int = *fresh25 as libc::c_int;
                if !invoke(method_0, argCount_0) {
                    return INTERPRET_RUNTIME_ERROR;
                }
                frame = &mut *(vm.frames)
                    .as_mut_ptr()
                    .offset((vm.frameCount - 1 as libc::c_int) as isize)
                    as *mut CallFrame;
            }
            30 => {
                let fresh26 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut method_1: *mut ObjString = (*((*(*(*frame).closure).function)
                    .chunk
                    .constants
                    .values)
                    .offset(*fresh26 as isize)
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjString;
                let fresh27 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut argCount_1: libc::c_int = *fresh27 as libc::c_int;
                let mut superclass_0: *mut ObjClass = (pop()
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjClass;
                if !invokeFromClass(superclass_0, method_1, argCount_1) {
                    return INTERPRET_RUNTIME_ERROR;
                }
                frame = &mut *(vm.frames)
                    .as_mut_ptr()
                    .offset((vm.frameCount - 1 as libc::c_int) as isize)
                    as *mut CallFrame;
            }
            31 => {
                let fresh28 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                let mut function_0: *mut ObjFunction = (*((*(*(*frame).closure).function)
                    .chunk
                    .constants
                    .values)
                    .offset(*fresh28 as isize)
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjFunction;
                let mut closure: *mut ObjClosure = newClosure(function_0);
                push(
                    0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t
                        | closure as uintptr_t,
                );
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < (*closure).upvalueCount {
                    let fresh29 = (*frame).ip;
                    (*frame).ip = ((*frame).ip).offset(1);
                    let mut isLocal: uint8_t = *fresh29;
                    let fresh30 = (*frame).ip;
                    (*frame).ip = ((*frame).ip).offset(1);
                    let mut index: uint8_t = *fresh30;
                    if isLocal != 0 {
                        let ref mut fresh31 = *((*closure).upvalues).offset(i as isize);
                        *fresh31 = captureUpvalue(
                            ((*frame).slots).offset(index as libc::c_int as isize),
                        );
                    } else {
                        let ref mut fresh32 = *((*closure).upvalues).offset(i as isize);
                        *fresh32 = *((*(*frame).closure).upvalues)
                            .offset(index as isize);
                    }
                    i += 1;
                    i;
                }
            }
            32 => {
                closeUpvalues((vm.stackTop).offset(-(1 as libc::c_int as isize)));
                pop();
            }
            33 => {
                let mut result: Value = pop();
                closeUpvalues((*frame).slots);
                vm.frameCount -= 1;
                vm.frameCount;
                if vm.frameCount == 0 as libc::c_int {
                    pop();
                    return INTERPRET_OK;
                }
                vm.stackTop = (*frame).slots;
                push(result);
                frame = &mut *(vm.frames)
                    .as_mut_ptr()
                    .offset((vm.frameCount - 1 as libc::c_int) as isize)
                    as *mut CallFrame;
            }
            34 => {
                let fresh33 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                push(
                    0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t
                        | newClass(
                            (*((*(*(*frame).closure).function).chunk.constants.values)
                                .offset(*fresh33 as isize)
                                & !(0x8000000000000000 as libc::c_ulong
                                    | 0x7ffc000000000000 as libc::c_long as uint64_t))
                                as *mut Obj as *mut ObjString,
                        ) as uintptr_t,
                );
            }
            35 => {
                let mut superclass_1: Value = peek_vm(1 as libc::c_int);
                if !isObjType(superclass_1, OBJ_CLASS) {
                    runtimeError(
                        b"Superclass must be a class.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return INTERPRET_RUNTIME_ERROR;
                }
                let mut subclass: *mut ObjClass = (peek_vm(0 as libc::c_int)
                    & !(0x8000000000000000 as libc::c_ulong
                        | 0x7ffc000000000000 as libc::c_long as uint64_t)) as *mut Obj
                    as *mut ObjClass;
                tableAddAll(
                    &mut (*((superclass_1
                        & !(0x8000000000000000 as libc::c_ulong
                            | 0x7ffc000000000000 as libc::c_long as uint64_t))
                        as *mut Obj as *mut ObjClass))
                        .methods,
                    &mut (*subclass).methods,
                );
                pop();
            }
            36 => {
                let fresh34 = (*frame).ip;
                (*frame).ip = ((*frame).ip).offset(1);
                defineMethod(
                    (*((*(*(*frame).closure).function).chunk.constants.values)
                        .offset(*fresh34 as isize)
                        & !(0x8000000000000000 as libc::c_ulong
                            | 0x7ffc000000000000 as libc::c_long as uint64_t))
                        as *mut Obj as *mut ObjString,
                );
            }
            _ => {}
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn hack(mut b: bool) {
    run();
    if b {
        hack(0 as libc::c_int != 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn interpret(mut source: *const libc::c_char) -> InterpretResult {
    let mut function_0: *mut ObjFunction = compile(source);
    if function_0.is_null() {
        return INTERPRET_COMPILE_ERROR;
    }
    push(
        0x8000000000000000 as libc::c_ulong
            | 0x7ffc000000000000 as libc::c_long as uint64_t | function_0 as uintptr_t,
    );
    let mut closure: *mut ObjClosure = newClosure(function_0);
    pop();
    push(
        0x8000000000000000 as libc::c_ulong
            | 0x7ffc000000000000 as libc::c_long as uint64_t | closure as uintptr_t,
    );
    call_vm(closure, 0 as libc::c_int);
    return run();
}
unsafe extern "C" fn repl() {
    let mut line: [libc::c_char; 1024] = [0; 1024];
    loop {
        printf(b"> \0" as *const u8 as *const libc::c_char);
        if (fgets(
            line.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            stdin,
        ))
            .is_null()
        {
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            break;
        } else {
            interpret(line.as_mut_ptr());
        }
    };
}
unsafe extern "C" fn readFile(mut path: *const libc::c_char) -> *mut libc::c_char {
    let mut file: *mut FILE = fopen(path, b"rb\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        fprintf(
            stderr,
            b"Could not open file \"%s\".\n\0" as *const u8 as *const libc::c_char,
            path,
        );
        exit(74 as libc::c_int);
    }
    fseek(file, 0 as libc::c_long, 2 as libc::c_int);
    let mut fileSize: size_t = ftell(file) as size_t;
    rewind(file);
    let mut buffer: *mut libc::c_char = malloc(
        fileSize.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if buffer.is_null() {
        fprintf(
            stderr,
            b"Not enough memory to read \"%s\".\n\0" as *const u8 as *const libc::c_char,
            path,
        );
        exit(74 as libc::c_int);
    }
    let mut bytesRead: size_t = fread(
        buffer as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        fileSize,
        file,
    );
    if bytesRead < fileSize {
        fprintf(
            stderr,
            b"Could not read file \"%s\".\n\0" as *const u8 as *const libc::c_char,
            path,
        );
        exit(74 as libc::c_int);
    }
    *buffer.offset(bytesRead as isize) = '\0' as i32 as libc::c_char;
    fclose(file);
    return buffer;
}
unsafe extern "C" fn runFile(mut path: *const libc::c_char) {
    let mut source: *mut libc::c_char = readFile(path);
    let mut result: InterpretResult = interpret(source);
    free(source as *mut libc::c_void);
    if result as libc::c_uint == INTERPRET_COMPILE_ERROR as libc::c_int as libc::c_uint {
        exit(65 as libc::c_int);
    }
    if result as libc::c_uint == INTERPRET_RUNTIME_ERROR as libc::c_int as libc::c_uint {
        exit(70 as libc::c_int);
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
) -> libc::c_int {
    initVM();
    if argc == 1 as libc::c_int {
        repl();
    } else if argc == 2 as libc::c_int {
        runFile(*argv.offset(1 as libc::c_int as isize));
    } else {
        fprintf(stderr, b"Usage: clox [path]\n\0" as *const u8 as *const libc::c_char);
        exit(64 as libc::c_int);
    }
    freeVM();
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *const libc::c_char,
            ) as i32,
        )
    }
}
