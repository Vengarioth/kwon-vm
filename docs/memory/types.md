# Types

## Type Definition

```
type_definition {
  u32 type_id;
  u16 type_size;
}
```

## Structure Definition
```
structure_definition {
  u32                structure_id;
  u16                type_size;
  u8                 field_count;
  field_definition[] field_definition
}
```

## Field Definition
```
field_definition {
  u8      field_id;
  pointer type_definition;
}
```

# Structures

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
