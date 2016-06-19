# Assembly
An assembly is a file containing a program in intermediate representation ready to be consumed by the virtual machine.

## File extension
Kwon Assembly Files should use the `.kas` file extension.

## Contents

### Header
The first section of the assembly file contains a header with a list of addresses for each of the following sections. The layout is optimized for compactness.

|name               |index  |
|-------------------|-------|
|string_ids         |0      |
|integer_ids        |1      |
|floating_point_ids |2      |
|data               |3      |

#### An Example
This list of addresses would mean string_ids is located 14 bytes into the file, integer_ids is located 25 bytes into the file and so on.
```
[14, 25, 13, ...]
```

### Constant Tables
The next tables contain a conversion from a constant id to an address into the data section.

Constant tables are not compressed to make them easily traversable. Numeric order is guaranteed. This makes it possible to look up an address by simply doing `address = (string_id * sizeof(u32) * 2) + sizeof(u32)`.

#### String Ids

The string_ids table contains a conversion from a string_id to an address in the data section.

To obtain the absolute address within the file add the address of the data section with the value from this table.

The data located at this address will be a compressed number representing the length in bytes of the string, followed by that exact number of bytes representing the string in `UTF8` encoding.

#### Layout

|string_id|address|
|-|-|
|u32|u32|

#### Integer Ids

same as string_ids

#### Floating Point Ids

same as string_ids

### Data

The Data section contains all the data described by the tables above, use them to index into this section.
