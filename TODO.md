1. Find an intelligent way to serialize command::Command to 12-byte slices.
1. Test hex methods ✅
1. Refactor the len methods for counting and allocating the size of the vectors for the to_bytes Payload method as a trait so that each message type can implement it and it will recursively give you back the size of your vector with allocations for the CompactInt that holds the vector length information as well.
1. Address OOM DOS vulnerability in deserialize ✅ (Never existed!)
1. Test updated ip deserialization 
