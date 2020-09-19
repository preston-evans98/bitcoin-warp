1. Find an intelligent way to serialize command::Command to 12-byte slices.
1. Switch Serialization for IpAddr to BigEndian
1. Transaction Output pub key script needs to imlplement a custom serialization and deserialization in Block struct
1. Fix msg and message in the send method in peer.rs
