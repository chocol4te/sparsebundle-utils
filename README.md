# sparsebundle-utils

A collection of tools for use with Apple `.sparsebundle` images.

Todo:

- [ ] Parsing
- [ ] Metadata extraction
- [ ] Brute force password tool (not actually for "hacking", I tend to forget what password I used, but know it is in a list and manually trying takes ages)
- [ ] Conversion of a `sparsebundle` to a raw disk image

Stretch goal:

- [ ] Repair and recovery tools

## Structure

### `Info.plist`

Contains metadata about the sparsebundle such as band size and version.

### `Info.bckup`

Presumably a backup of `Info.plist`, I don't know when new backups are supposed to replace an existing one.

### `token`

Empty if no encryption is enabled. If enabled, the first 8 bytes read "encrcdsa" in ASCII. Bytes 26 and 27 hold the key size (128/256) in big endian form. 16 bytes starting at byte 36 and 20 bytes starting at offset 112 appear to be random, perhaps salts or IVs? In the AES-256 encrypted sparsebundle there are 64 bytes starting at offset 200 that appear to be random, in the AES-128 bundle there are 48 bytes starting at offset 200. All other data appears to be 32 bit integers in big endian form, but I am just assuming this due to the leading zeros in 4 byte groups.

### `bands/*`

Actual data, I do not yet know what order the files are read in.

### `mapped/*`

I think these files are used as a bitfield to mark which blocks of data in bands have been used?
