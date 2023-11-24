/// @description Insert description here
// You can write your code in this editor

switch (async_load[? "type"]) {
	
	
	case network_type_data:
	
		var _read = async_load[? "buffer"];
		var _size = async_load[? "size"];
		
		while (buffer_tell(_read) < _size) {
			var _length = buffer_read(_read, buffer_u64);
			var _copied = buffer_create(_length, buffer_grow, 1);
			buffer_copy(_read, buffer_tell(_read), _length, _copied, 0);
			buffer_seek(_read, buffer_seek_relative, _length);
			
			process(SnapBufferReadMessagePack(_copied, 0));
			
		}
		
	break;
	
}