## Terminology
i = integer (signed or unsigned)
s = signed
u = unsigned

## Supported Number types
i32, i64
f32, f64

## Bytes
u8

## Vectors
Numeric vectors are 128 bit values processed by SIMD instructions

## Names
These are UTF-8 encoded strings (n < 1114112).

## Type Uses(????)

## Heap types
* Function types 
* Aggregate types (dynamically allocated "managed" date, structs, arrays, unboxed scalars)
* External types (external references possibly owned by the embedder)

These types can be either "abstract" or "concrete". 
* Concrete heap types consists of a "type use" that classifies an object of the type defined in a module. 
  (the compiler relates it to the symbol, duh!)
* Abstract types are denoted by individual keywords

### Type X
* type func - supertype of all function types regardless of concrete definition.
	* type nofunc - common subtype of all function types, but has no values
	  (CLARIFY: Does this mean that it can be any arbitrary function?)
* type exn - exception references. Has NO concrete subtypes
	* type noexn  - common subtype of all forms of exception references. Has NO values. 
* type extern - supertype of external references. 
	* type noextern - common subtype of external references. No values.
* type any - supertype of all aggregates.
	* type none - common subtype of all aggregates. No values.
* type eq - subtype that includes all types for which references can be compared
* struct - supertype of all structure aggregates
* array - supertype of all array aggregates
* i31 - unboxed scalars. Limited to 31 bits... 
	* These types are not actually on the heap, but are represented to allow them to be mixed with actual references. The extra bit is for tagging
	* Though the types none, nofunc, noexn, noextern have no values, they can fulfill types like `void*`. 
# Return to page 11


