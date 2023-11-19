function process(_data) {

	show_debug_message(_data);
	
	switch _data[0] {
		
		case "initialize":
			var player = instance_create_layer(_data[2], _data[3], "Instances", oSelf);
			player.uuid = _data[1];
		break;
		
		case "position":
			
			var exists = false;
			
			with (oOther) {
				
				if _data[1] == self.uuid exists = true;
				
				self.x = _data[2];
				self.y = _data[3];
				
			}
			
			if !exists {
				
				var player = instance_create_layer(_data[2], _data[3], "Instances", oOther);
				player.uuid = _data[1];
			}
			
			
		break;
		
		case "disconnect": 
			
			with (oOther) {
				
				if uuid == _data[1] {
					instance_destroy(self);
					break;
				}
				
			}
		
		break;
		
	}
	
}

function send(_data) {
	
	var _encoded = buffer_create(1, buffer_grow, 1);
	SnapBufferWriteMessagePack(_encoded, _data);
	
	var _size_encoded = buffer_tell(_encoded);
	var _buff_with_size = buffer_create(_size_encoded + 1, buffer_fixed, 1);
	buffer_write(_buff_with_size, buffer_u64, _size_encoded);
	buffer_copy(_encoded, 0, _size_encoded, _buff_with_size, 1);
	show_debug_message("Sent: ");
	show_debug_message(buffer_prettyprint(_buff_with_size));
	network_send_raw(Networking.socket, _buff_with_size, _size_encoded + 1);
	
}