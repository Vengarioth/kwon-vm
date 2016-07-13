# Kwon Memory Model

## Preface

Kwon VM features a garbage collected memory model. Influenced by the [LuaJit model](http://wiki.luajit.org/New-Garbage-Collector).

## Memory

Memory is managed in big, aligned regions. A region is directly requested from the operating system. All regions have the same size.

### Regions

A Region is an allocated block of memory, containing a metadata and a data area. The metadata area describes how the data area is used. The data area is split into blocks. A block contains one or more 16 byte cells.

#### Sizes
| Type | Size |
|------|------|
| Region | Any power of two between 64kb and 1mb |
| Metadata | 1/64th of the Region's size |
| Block | 1 or more Cells |
| Cell | 16 bytes |

### Pointers

| 16 bits | 16 bits |
|-|-|
| region address | address within region |

-----

```
collection_pressure
fragmentation_pressure

managedMemory.allocate(type_size);
  has a region with free memory?
    fragmentation_pressure
      high -> fit allocator
        has enough subsequent free cells?
          allocate here

      low -> bump allocator
        is enough memory free at the end?
          allocate here

  collection_pressure
    high -> collect
      do collection & start over

    low -> create region
      allocate new region
        allocate in new region
```
