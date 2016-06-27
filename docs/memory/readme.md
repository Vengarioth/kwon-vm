# Memory

## Primitives

### Integer Types

|Keyword|Name|Size|Signed|Min value|Max value|
|-|-|-|-|-|-|
|bool|Boolean|1 bit|no|0|1|
|u8|Byte|1 byte|no|0|255|
|i8|Signed Byte|1 byte|yes|-128|127|
|u16|Unsigned Short|4 bytes|no|0|65535|
|i16|Short|4 bytes|yes|-32768|32767|
|u32|Unsigned Integer|8 bytes|no|0|4294967295|
|i32|Integer|8 bytes|yes|-2147483648|2147483647|
|u64|Unsigned Long Integer|16 bytes|no|0|18446744073709551615|
|i64|Long Integer|16 bytes|yes|-9223372036854775808|9223372036854775807|

### Floating Point Types
|Keyword|Name|Size|Precision|
|-|-|-|-|
|float|Floating Point Number|8 bytes|?|
|double|Double Length Floating Point Number|16 bytes|?|

## Types

### Type Definition

```
type_definition {
  u32 type_id;
  u16 type_size;
}
```

### Structure Definition
```
structure_definition {
  u32                structure_id;
  u16                type_size;
  u8                 field_count;
  field_definition[] field_definition
}
```

### Field Definition
```
field_definition {
  u8      field_id;
  pointer type_definition;
}
```

## Structures

```
{
  pointer structure_definition;
  [member_data];
}
```

```
struct my_struct {
  int some_number;
  string some_string;
  int[] some_int_array;
}
```

//arrays are structs
Array {
  pointer element_type;
  pointer start;
  pointer end;
}

//slices are structs
Slice {
  pointer array;
  pointer start;
  pointer end;
}


```
struct binary_tree {
  pointer parent;
  pointer child_a;
  pointer child_b;
}
```

```
struct binary_tree {
  binary_tree parent;
  binary_tree child_a;
  binary_tree child_b;
}
```

myTreeElement.child_a = someOtherTreeElement;


```
struct Vector<T> {
  pointer type_t;
  pointer array_t;
}
```

```
struct LinkedList<T> {
  pointer type_t;
  pointer before;
  pointer after;
}
```

```
struct LinkedList<T> {
  LinkedList<T> before;
  LinkedList<T> after;
}
```
