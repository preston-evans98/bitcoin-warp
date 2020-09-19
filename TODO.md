1. Find an intelligent way to serialize command::Command to 12-byte slices.
1. Test u256 serialize and hex methods
1. Switch Serialization for IpAddr to BigEndian
1. Transaction Output pub key script needs to imlplement a custom serialization and deserialization in Block struct
1. Take out all the Compact Ints in the structure definitions.
