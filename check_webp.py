import sys
import struct

def check_webp_loop(filepath):
    with open(filepath, 'rb') as f:
        data = f.read()
    
    if data[:4] != b'RIFF' or data[8:12] != b'WEBP':
        return "Not WebP"
        
    offset = 12
    while offset < len(data):
        chunk_id = data[offset:offset+4]
        chunk_size = struct.unpack('<I', data[offset+4:offset+8])[0]
        chunk_data = data[offset+8:offset+8+chunk_size]
        
        if chunk_id == b'ANIM':
            bg_color = struct.unpack('<I', chunk_data[0:4])[0]
            loop_count = struct.unpack('<H', chunk_data[4:6])[0]
            return f"Loop count: {loop_count}"
            
        offset += 8 + chunk_size
        if chunk_size % 2 == 1:
            offset += 1 # padding
            
    return "No ANIM chunk found"

print(check_webp_loop('assets/emojis_anim/alien.webp'))
print(check_webp_loop('assets/emojis_anim/kiss-mark.webp'))
